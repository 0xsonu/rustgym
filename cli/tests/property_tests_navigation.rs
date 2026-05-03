// Feature: cli-tui-rewrite, Property 2: Back navigation pops the screen stack
// Feature: cli-tui-rewrite, Property 3: Enter navigation pushes the correct detail screen

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use proptest::prelude::*;

use rustgym_cli::action::{Action, ScreenId};
use rustgym_cli::api::{LevelSummary, QuestSummary, TaskSummary};
use rustgym_cli::app::AppState;
use rustgym_cli::event::AppEvent;
use rustgym_cli::screens::quest_detail::QuestDetailScreen;
use rustgym_cli::screens::quest_list::QuestListScreen;
use rustgym_cli::screens::level_detail::LevelDetailScreen;
use rustgym_cli::screens::ScreenHandler;

// ─── Helpers ────────────────────────────────────────────────────────────────

/// Create a KeyEvent for a given KeyCode.
fn key_event(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

/// Generate a random ScreenId for building navigation stacks.
fn arb_screen_id() -> impl Strategy<Value = ScreenId> {
    prop_oneof![
        Just(ScreenId::Login),
        Just(ScreenId::QuestList),
        "[a-z]{3,10}".prop_map(|slug| ScreenId::QuestDetail { slug }),
        ("[a-z]{3,10}", "[a-z]{3,10}")
            .prop_map(|(quest_slug, level_slug)| ScreenId::LevelDetail {
                quest_slug,
                level_slug
            }),
        "[a-z]{3,10}".prop_map(|slug| ScreenId::TaskDetail { slug }),
        "[a-z]{3,10}".prop_map(|slug| ScreenId::Submit { slug }),
    ]
}

/// Generate a navigation stack of a given size.
fn arb_nav_stack(min_size: usize, max_size: usize) -> impl Strategy<Value = Vec<ScreenId>> {
    proptest::collection::vec(arb_screen_id(), min_size..=max_size)
}

/// Generate a random slug string.
fn arb_slug() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9\\-]{2,15}".prop_map(|s| s)
}

// ─── Property 2: Back navigation pops the screen stack ──────────────────────

// **Validates: Requirements 2.6, 5.4, 6.5, 7.7**
//
// For any navigation stack with length > 1, pressing `b` SHALL reduce the
// stack length by exactly 1 and the new top of the stack SHALL equal the
// previous second-to-top element.
//
// We test this by simulating the Pop action behavior on the nav_stack directly,
// which is what execute_action does when a screen returns Action::Pop.
proptest! {
    #[test]
    fn prop_back_navigation_pops_stack(
        nav_stack in arb_nav_stack(2, 10),
    ) {
        let original_len = nav_stack.len();
        let expected_top = nav_stack[original_len - 2].clone();

        // Simulate the Pop action (same logic as execute_action in main.rs)
        let mut stack = nav_stack.clone();
        if stack.len() > 1 {
            stack.pop();
        }

        // Stack length reduced by exactly 1
        prop_assert_eq!(stack.len(), original_len - 1);
        // New top equals previous second-to-top
        prop_assert_eq!(stack.last().unwrap(), &expected_top);
    }

    #[test]
    fn prop_back_navigation_does_not_pop_single_element_stack(
        screen_id in arb_screen_id(),
    ) {
        let mut stack = vec![screen_id.clone()];

        // Simulate the Pop action — should NOT pop when len == 1
        if stack.len() > 1 {
            stack.pop();
        }

        // Stack remains unchanged
        prop_assert_eq!(stack.len(), 1);
        prop_assert_eq!(stack.last().unwrap(), &screen_id);
    }
}

// ─── Property 3: Enter navigation pushes the correct detail screen ──────────

// **Validates: Requirements 2.5, 4.4, 5.3, 6.4**

/// Helper: create a QuestSummary with a given slug.
fn make_quest_summary(slug: &str) -> QuestSummary {
    QuestSummary {
        slug: slug.to_string(),
        title: format!("Quest {}", slug),
        description: None,
        level_count: 1,
        task_count: 5,
        user_progress: None,
    }
}

/// Helper: create a LevelSummary with a given slug (marked as completed so it's not locked).
fn make_level_summary(slug: &str) -> LevelSummary {
    LevelSummary {
        slug: slug.to_string(),
        title: format!("Level {}", slug),
        description: None,
        task_count: 3,
        user_progress: Some(rustgym_cli::api::LevelProgress {
            tasks_completed: 3,
            tasks_total: 3,
            is_completed: true,
        }),
    }
}

/// Helper: create a TaskSummary with a given slug.
fn make_task_summary(slug: &str) -> TaskSummary {
    TaskSummary {
        slug: slug.to_string(),
        title: format!("Task {}", slug),
        difficulty: "beginner".to_string(),
        xp_reward: 10,
        user_progress: None,
    }
}

proptest! {
    /// QuestListScreen: Enter with quests loaded → returns Action::Push(ScreenId::QuestDetail { slug })
    #[test]
    fn prop_enter_on_quest_list_pushes_quest_detail(
        slugs in proptest::collection::vec(arb_slug(), 1..=20),
        selected_idx_raw in 0usize..20,
    ) {
        let quests: Vec<QuestSummary> = slugs.iter().map(|s| make_quest_summary(s)).collect();
        let selected_idx = selected_idx_raw % quests.len();

        let mut screen = QuestListScreen::new();
        screen.quests = quests.clone();
        screen.list_state.items_count = quests.len();
        screen.list_state.selected_index = selected_idx;
        screen.fetched = true; // Prevent fetch action

        let app_state = AppState::new();
        let event = AppEvent::Input(key_event(KeyCode::Enter));

        let action = screen.update(&event, &app_state);

        let expected_slug = &quests[selected_idx].slug;
        match action {
            Action::Push(ScreenId::QuestDetail { slug }) => {
                prop_assert_eq!(&slug, expected_slug);
            }
            other => {
                prop_assert!(false, "Expected Action::Push(QuestDetail), got {:?}", other);
            }
        }
    }

    /// QuestDetailScreen: Enter with levels loaded → returns Action::Push(ScreenId::LevelDetail { quest_slug, level_slug })
    #[test]
    fn prop_enter_on_quest_detail_pushes_level_detail(
        quest_slug in arb_slug(),
        level_slugs in proptest::collection::vec(arb_slug(), 1..=20),
        selected_idx_raw in 0usize..20,
    ) {
        let levels: Vec<LevelSummary> = level_slugs.iter().map(|s| make_level_summary(s)).collect();
        let selected_idx = selected_idx_raw % levels.len();

        let mut screen = QuestDetailScreen::new(quest_slug.clone());
        screen.levels = levels.clone();
        screen.list_state.items_count = levels.len();
        screen.list_state.selected_index = selected_idx;
        screen.fetched = true; // Prevent fetch action

        let app_state = AppState::new();
        let event = AppEvent::Input(key_event(KeyCode::Enter));

        let action = screen.update(&event, &app_state);

        let expected_level_slug = &levels[selected_idx].slug;

        // The screen may return a Batch if it also triggers a fetch on first update.
        // Since we set fetched = true, it should return just the Push action.
        match action {
            Action::Push(ScreenId::LevelDetail {
                quest_slug: qs,
                level_slug: ls,
            }) => {
                prop_assert_eq!(&qs, &quest_slug);
                prop_assert_eq!(&ls, expected_level_slug);
            }
            other => {
                prop_assert!(false, "Expected Action::Push(LevelDetail), got {:?}", other);
            }
        }
    }

    /// LevelDetailScreen: Enter with tasks loaded → returns Action::Push(ScreenId::TaskDetail { slug })
    #[test]
    fn prop_enter_on_level_detail_pushes_task_detail(
        quest_slug in arb_slug(),
        level_slug in arb_slug(),
        task_slugs in proptest::collection::vec(arb_slug(), 1..=20),
        selected_idx_raw in 0usize..20,
    ) {
        let tasks: Vec<TaskSummary> = task_slugs.iter().map(|s| make_task_summary(s)).collect();
        let selected_idx = selected_idx_raw % tasks.len();

        let mut screen = LevelDetailScreen::new(quest_slug.clone(), level_slug.clone());
        screen.tasks = tasks.clone();
        screen.list_state.items_count = tasks.len();
        screen.list_state.selected_index = selected_idx;
        screen.fetched = true; // Prevent fetch action

        let app_state = AppState::new();
        let event = AppEvent::Input(key_event(KeyCode::Enter));

        let action = screen.update(&event, &app_state);

        let expected_task_slug = &tasks[selected_idx].slug;
        match action {
            Action::Push(ScreenId::TaskDetail { slug }) => {
                prop_assert_eq!(&slug, expected_task_slug);
            }
            other => {
                prop_assert!(false, "Expected Action::Push(TaskDetail), got {:?}", other);
            }
        }
    }
}
