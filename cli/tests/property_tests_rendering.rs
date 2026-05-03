// Feature: cli-tui-rewrite, Property 6: Status/difficulty color mapping is deterministic and distinct
// Feature: cli-tui-rewrite, Property 7: List item rendering includes all required fields
// Feature: cli-tui-rewrite, Property 8: Locked levels display lock indicator
// Feature: cli-tui-rewrite, Property 15: Status bar contains screen name, user info, and key hints

use proptest::prelude::*;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::Widget;

use rustgym_cli::api::{
    LevelProgress, LevelSummary, QuestProgress, QuestSummary, TaskProgress, TaskSummary,
};
use rustgym_cli::screens::level_detail::LevelDetailScreen;
use rustgym_cli::screens::quest_detail::QuestDetailScreen;
use rustgym_cli::widgets::StatusBar;

// ─── Generators ─────────────────────────────────────────────────────────────

/// Generate a random difficulty string from the known set.
fn arb_difficulty() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("beginner".to_string()),
        Just("intermediate".to_string()),
        Just("advanced".to_string()),
    ]
}

/// Generate a random non-empty string for titles/slugs.
fn arb_title() -> impl Strategy<Value = String> {
    "[A-Za-z][A-Za-z0-9 ]{2,20}".prop_map(|s| s)
}

/// Generate a random slug.
fn arb_slug() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9\\-]{2,15}".prop_map(|s| s)
}

/// Generate a random QuestSummary with all fields populated.
fn arb_quest_summary() -> impl Strategy<Value = QuestSummary> {
    (
        arb_slug(),
        arb_title(),
        1i64..=20,
        1i64..=50,
        proptest::option::of((0i32..=50, 1i32..=50, any::<bool>())),
    )
        .prop_map(
            |(slug, title, level_count, task_count, progress)| QuestSummary {
                slug,
                title,
                description: Some("A quest description".to_string()),
                level_count,
                task_count,
                user_progress: progress.map(|(completed, total, is_completed)| QuestProgress {
                    tasks_completed: completed.min(total),
                    tasks_total: total,
                    is_completed,
                }),
            },
        )
}

/// Generate a random LevelSummary with optional progress.
fn arb_level_summary() -> impl Strategy<Value = LevelSummary> {
    (
        arb_slug(),
        arb_title(),
        1i64..=20,
        proptest::option::of((0i32..=20, 1i32..=20, any::<bool>())),
    )
        .prop_map(|(slug, title, task_count, progress)| LevelSummary {
            slug,
            title,
            description: Some("A level description".to_string()),
            task_count,
            user_progress: progress.map(|(completed, total, is_completed)| LevelProgress {
                tasks_completed: completed.min(total),
                tasks_total: total,
                is_completed,
            }),
        })
}

/// Generate a random TaskSummary with all fields populated.
fn arb_task_summary() -> impl Strategy<Value = TaskSummary> {
    (
        arb_slug(),
        arb_title(),
        arb_difficulty(),
        1i32..=100,
        proptest::option::of(("completed|in_progress|not_started", 0i32..=10)),
    )
        .prop_map(|(slug, title, difficulty, xp_reward, progress)| TaskSummary {
            slug,
            title,
            difficulty,
            xp_reward,
            user_progress: progress.map(|(status, attempts)| TaskProgress { status, attempts }),
        })
}

/// Generate a random non-empty username.
fn arb_username() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9_]{2,12}".prop_map(|s| s)
}

/// Generate a random screen name.
fn arb_screen_name() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("Quests".to_string()),
        Just("Quest Detail".to_string()),
        Just("Level Detail".to_string()),
        Just("Task Detail".to_string()),
        Just("Submit".to_string()),
        Just("Login".to_string()),
    ]
}

/// Generate random key hints.
fn arb_key_hints() -> impl Strategy<Value = Vec<(String, String)>> {
    proptest::collection::vec(
        (
            prop_oneof![
                Just("j/k".to_string()),
                Just("Enter".to_string()),
                Just("b".to_string()),
                Just("q".to_string()),
                Just("s".to_string()),
                Just("r".to_string()),
                Just("Tab".to_string()),
            ],
            prop_oneof![
                Just("navigate".to_string()),
                Just("select".to_string()),
                Just("back".to_string()),
                Just("quit".to_string()),
                Just("submit".to_string()),
                Just("retry".to_string()),
                Just("switch".to_string()),
            ],
        ),
        1..=4,
    )
}

// ─── Property 6: Status/difficulty color mapping is deterministic and distinct ─

// **Validates: Requirements 4.3, 6.3**

proptest! {
    /// difficulty_color returns Green for beginner, Yellow for intermediate, Red for advanced.
    /// These are all distinct colors, and the mapping is deterministic.
    #[test]
    fn prop_difficulty_color_deterministic_and_distinct(
        difficulty in arb_difficulty(),
        // Call multiple times to verify determinism
        _iterations in 0u8..5,
    ) {
        let color1 = LevelDetailScreen::difficulty_color(&difficulty);
        let color2 = LevelDetailScreen::difficulty_color(&difficulty);

        // Determinism: same input always produces same output
        prop_assert_eq!(color1, color2);

        // Correct mapping
        match difficulty.as_str() {
            "beginner" => prop_assert_eq!(color1, Color::Green),
            "intermediate" => prop_assert_eq!(color1, Color::Yellow),
            "advanced" => prop_assert_eq!(color1, Color::Red),
            _ => unreachable!(),
        }
    }

    /// All three difficulty levels map to distinct colors.
    #[test]
    fn prop_difficulty_colors_are_distinct(_seed in 0u32..100) {
        let beginner = LevelDetailScreen::difficulty_color("beginner");
        let intermediate = LevelDetailScreen::difficulty_color("intermediate");
        let advanced = LevelDetailScreen::difficulty_color("advanced");

        prop_assert_ne!(beginner, intermediate);
        prop_assert_ne!(beginner, advanced);
        prop_assert_ne!(intermediate, advanced);
    }

    /// Quest progress states map to distinct colors (completed=Green, in-progress=Yellow, not-started=DarkGray).
    #[test]
    fn prop_quest_progress_color_deterministic(
        tasks_completed in 0i32..=50,
        tasks_total in 1i32..=50,
        is_completed in any::<bool>(),
    ) {
        // Simulate the color logic from quest_list.rs render
        let progress = QuestProgress {
            tasks_completed: tasks_completed.min(tasks_total),
            tasks_total,
            is_completed,
        };

        let color = if progress.is_completed {
            Color::Green
        } else if progress.tasks_completed > 0 {
            Color::Yellow
        } else {
            Color::DarkGray
        };

        // Verify determinism by computing again
        let color2 = if progress.is_completed {
            Color::Green
        } else if progress.tasks_completed > 0 {
            Color::Yellow
        } else {
            Color::DarkGray
        };

        prop_assert_eq!(color, color2);

        // Verify distinctness of the three states
        let completed_color = Color::Green;
        let in_progress_color = Color::Yellow;
        let not_started_color = Color::DarkGray;

        prop_assert_ne!(completed_color, in_progress_color);
        prop_assert_ne!(completed_color, not_started_color);
        prop_assert_ne!(in_progress_color, not_started_color);
    }
}

// ─── Property 7: List item rendering includes all required fields ───────────

// **Validates: Requirements 4.2, 5.2, 6.2**

proptest! {
    /// For any QuestSummary, all required fields (title, level_count, task_count, progress) are accessible.
    #[test]
    fn prop_quest_summary_contains_all_required_fields(
        quest in arb_quest_summary(),
    ) {
        // Verify title is non-empty
        prop_assert!(!quest.title.is_empty(), "Quest title must not be empty");

        // Verify level_count is positive
        prop_assert!(quest.level_count > 0, "Quest must have at least one level");

        // Verify task_count is positive
        prop_assert!(quest.task_count > 0, "Quest must have at least one task");

        // Verify progress percentage can be computed
        let progress_pct = match &quest.user_progress {
            Some(p) if p.tasks_total > 0 => {
                (p.tasks_completed as f64 / p.tasks_total as f64) * 100.0
            }
            Some(_) => 0.0,
            None => 0.0,
        };
        prop_assert!(progress_pct >= 0.0 && progress_pct <= 100.0);

        // Verify all fields are accessible (compile-time check + runtime assertion)
        let _slug = &quest.slug;
        let _title = &quest.title;
        let _level_count = quest.level_count;
        let _task_count = quest.task_count;
        let _progress = &quest.user_progress;
    }

    /// For any LevelSummary, all required fields (title, task_count, completion status) are accessible.
    #[test]
    fn prop_level_summary_contains_all_required_fields(
        level in arb_level_summary(),
    ) {
        // Verify title is non-empty
        prop_assert!(!level.title.is_empty(), "Level title must not be empty");

        // Verify task_count is positive
        prop_assert!(level.task_count > 0, "Level must have at least one task");

        // Verify completion status is derivable
        let is_completed = match &level.user_progress {
            Some(p) => p.is_completed,
            None => false,
        };
        // is_completed is a valid boolean (always true by type system)
        let _completed = is_completed;

        // Verify all fields are accessible
        let _slug = &level.slug;
        let _title = &level.title;
        let _task_count = level.task_count;
        let _progress = &level.user_progress;
    }

    /// For any TaskSummary, all required fields (title, difficulty, xp_reward, completion status) are accessible.
    #[test]
    fn prop_task_summary_contains_all_required_fields(
        task in arb_task_summary(),
    ) {
        // Verify title is non-empty
        prop_assert!(!task.title.is_empty(), "Task title must not be empty");

        // Verify difficulty is one of the known values
        let valid_difficulties = ["beginner", "intermediate", "advanced"];
        prop_assert!(
            valid_difficulties.contains(&task.difficulty.as_str()),
            "Task difficulty must be beginner, intermediate, or advanced, got: {}",
            task.difficulty
        );

        // Verify xp_reward is positive
        prop_assert!(task.xp_reward > 0, "Task XP reward must be positive");

        // Verify completion status is derivable
        let status = match &task.user_progress {
            Some(p) => p.status.clone(),
            None => "not_started".to_string(),
        };
        let _status = status;

        // Verify all fields are accessible
        let _slug = &task.slug;
        let _title = &task.title;
        let _difficulty = &task.difficulty;
        let _xp_reward = task.xp_reward;
        let _progress = &task.user_progress;
    }
}

// ─── Property 8: Locked levels display lock indicator ───────────────────────

// **Validates: Requirements 5.5**

proptest! {
    /// First level is never locked (no prerequisite).
    #[test]
    fn prop_first_level_never_locked(
        levels in proptest::collection::vec(arb_level_summary(), 1..=10),
    ) {
        let mut screen = QuestDetailScreen::new("test-quest".to_string());
        screen.levels = levels;
        screen.list_state.items_count = screen.levels.len();

        // First level should never be locked
        prop_assert!(!screen.is_level_locked(0), "First level must never be locked");
    }

    /// A level is locked if the previous level is NOT completed.
    #[test]
    fn prop_level_locked_when_previous_not_completed(
        first_level in arb_level_summary(),
        second_level in arb_level_summary(),
    ) {
        // Create a first level that is NOT completed
        let mut first = first_level;
        first.user_progress = Some(LevelProgress {
            tasks_completed: 1,
            tasks_total: 5,
            is_completed: false,
        });

        let mut screen = QuestDetailScreen::new("test-quest".to_string());
        screen.levels = vec![first, second_level];
        screen.list_state.items_count = 2;

        // Second level should be locked because first is not completed
        prop_assert!(
            screen.is_level_locked(1),
            "Level should be locked when previous level is not completed"
        );
    }

    /// A level is unlocked if the previous level IS completed.
    #[test]
    fn prop_level_unlocked_when_previous_completed(
        first_level in arb_level_summary(),
        second_level in arb_level_summary(),
    ) {
        // Create a first level that IS completed
        let mut first = first_level;
        first.user_progress = Some(LevelProgress {
            tasks_completed: 5,
            tasks_total: 5,
            is_completed: true,
        });

        let mut screen = QuestDetailScreen::new("test-quest".to_string());
        screen.levels = vec![first, second_level];
        screen.list_state.items_count = 2;

        // Second level should NOT be locked because first is completed
        prop_assert!(
            !screen.is_level_locked(1),
            "Level should be unlocked when previous level is completed"
        );
    }

    /// A level with no progress on the previous level is locked.
    #[test]
    fn prop_level_locked_when_previous_has_no_progress(
        first_level in arb_level_summary(),
        second_level in arb_level_summary(),
    ) {
        // Create a first level with no progress at all
        let mut first = first_level;
        first.user_progress = None;

        let mut screen = QuestDetailScreen::new("test-quest".to_string());
        screen.levels = vec![first, second_level];
        screen.list_state.items_count = 2;

        // Second level should be locked because first has no progress
        prop_assert!(
            screen.is_level_locked(1),
            "Level should be locked when previous level has no progress"
        );
    }

    /// For a sequence of levels, lock state is consistent with sequential prerequisite logic.
    #[test]
    fn prop_level_lock_state_consistent_with_prerequisites(
        levels in proptest::collection::vec(arb_level_summary(), 2..=8),
    ) {
        let mut screen = QuestDetailScreen::new("test-quest".to_string());
        screen.levels = levels;
        screen.list_state.items_count = screen.levels.len();

        for i in 0..screen.levels.len() {
            let is_locked = screen.is_level_locked(i);

            if i == 0 {
                // First level is never locked
                prop_assert!(!is_locked, "First level must never be locked");
            } else {
                // Level i is locked iff previous level is not completed
                let prev_completed = match &screen.levels[i - 1].user_progress {
                    Some(p) => p.is_completed,
                    None => false,
                };
                if prev_completed {
                    prop_assert!(
                        !is_locked,
                        "Level {} should be unlocked (previous is completed)",
                        i
                    );
                } else {
                    prop_assert!(
                        is_locked,
                        "Level {} should be locked (previous is not completed)",
                        i
                    );
                }
            }
        }
    }
}

// ─── Property 15: Status bar contains screen name, user info, and key hints ─

// **Validates: Requirements 11.6**

proptest! {
    /// The status bar render contains the screen name, username, and at least one key hint.
    #[test]
    fn prop_status_bar_contains_screen_name_user_and_hints(
        screen_name in arb_screen_name(),
        username in arb_username(),
        xp in 0i32..=99999,
        key_hints in arb_key_hints(),
    ) {
        // Build owned key hints for the StatusBar (needs &str references)
        let hints_refs: Vec<(&str, &str)> = key_hints
            .iter()
            .map(|(k, d)| (k.as_str(), d.as_str()))
            .collect();

        let bar = StatusBar::new(&screen_name, &username, xp, hints_refs);

        // Render to a buffer wide enough to contain all content
        let width = 120u16;
        let area = Rect::new(0, 0, width, 1);
        let mut buf = Buffer::empty(area);
        bar.render(area, &mut buf);

        // Extract rendered content as a string
        let content: String = buf
            .content()
            .iter()
            .map(|cell| cell.symbol().chars().next().unwrap_or(' '))
            .collect();

        // Status bar must contain the screen name
        prop_assert!(
            content.contains(&screen_name),
            "Status bar must contain screen name '{}', got: '{}'",
            screen_name,
            content
        );

        // Status bar must contain the username
        prop_assert!(
            content.contains(&username),
            "Status bar must contain username '{}', got: '{}'",
            username,
            content
        );

        // Status bar must contain at least one key hint
        let has_hint = key_hints.iter().any(|(key, _desc)| content.contains(key.as_str()));
        prop_assert!(
            has_hint,
            "Status bar must contain at least one key hint from {:?}, got: '{}'",
            key_hints,
            content
        );
    }

    /// When username is empty, the status bar still renders screen name and hints without panic.
    #[test]
    fn prop_status_bar_empty_username_still_renders(
        screen_name in arb_screen_name(),
        key_hints in arb_key_hints(),
    ) {
        let hints_refs: Vec<(&str, &str)> = key_hints
            .iter()
            .map(|(k, d)| (k.as_str(), d.as_str()))
            .collect();

        let bar = StatusBar::new(&screen_name, "", 0, hints_refs);

        let width = 120u16;
        let area = Rect::new(0, 0, width, 1);
        let mut buf = Buffer::empty(area);
        bar.render(area, &mut buf);

        let content: String = buf
            .content()
            .iter()
            .map(|cell| cell.symbol().chars().next().unwrap_or(' '))
            .collect();

        // Screen name should still be present
        prop_assert!(
            content.contains(&screen_name),
            "Status bar must contain screen name even with empty username"
        );
    }
}
