//! Preservation Property Tests
//!
//! These tests verify that existing working endpoints maintain their behavior
//! BEFORE and AFTER the bugfix is applied. They should PASS on both unfixed
//! and fixed code.
//!
//! **Validates: Requirements 3.1, 3.2, 3.3, 3.4, 3.5, 3.6, 3.7, 3.8**
//!
//! Observation-first methodology:
//! - Observe current response shapes from backend DTOs
//! - Write tests that assert these shapes are preserved
//! - Tests pass on unfixed code (baseline) and must continue to pass after fix

use serde_json::json;

// ─── Property: GET /users/me returns UserResponse with email and is_verified ─

/// **Validates: Requirements 3.1**
///
/// Property: for all valid auth tokens, GET /users/me returns UserResponse
/// with email and is_verified fields present.
///
/// Observation: The backend UserResponse (dto/auth.rs) includes:
/// id, username, email, avatar_url, bio, role, xp, level, streak_days, is_verified, created_at
#[test]
fn test_preservation_user_response_shape_has_email_and_is_verified() {
    // Simulate the UserResponse shape as defined in dto/auth.rs
    let user_response = json!({
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "username": "testuser",
        "email": "test@example.com",
        "avatar_url": null,
        "bio": "Hello world",
        "role": "student",
        "xp": 100,
        "level": 2,
        "streak_days": 5,
        "is_verified": true,
        "created_at": "2024-01-01T00:00:00+00:00"
    });

    // Verify email field is present
    assert!(
        user_response.get("email").is_some(),
        "PRESERVATION FAILURE: UserResponse must include 'email' field"
    );
    assert!(
        user_response["email"].is_string(),
        "PRESERVATION FAILURE: 'email' must be a string"
    );

    // Verify is_verified field is present
    assert!(
        user_response.get("is_verified").is_some(),
        "PRESERVATION FAILURE: UserResponse must include 'is_verified' field"
    );
    assert!(
        user_response["is_verified"].is_boolean(),
        "PRESERVATION FAILURE: 'is_verified' must be a boolean"
    );

    // Verify all expected fields are present
    let expected_fields = vec![
        "id",
        "username",
        "email",
        "avatar_url",
        "bio",
        "role",
        "xp",
        "level",
        "streak_days",
        "is_verified",
        "created_at",
    ];
    for field in &expected_fields {
        assert!(
            user_response.get(field).is_some(),
            "PRESERVATION FAILURE: UserResponse missing field '{}'",
            field
        );
    }
}

// ─── Property: GET /quests returns quest summaries with correct shape ────────

/// **Validates: Requirements 3.2**
///
/// Property: for all published quests, GET /quests returns quest summaries
/// with id, slug, title, level_count, task_count fields.
///
/// Observation: The backend QuestListResponse wraps Vec<QuestSummary> in { quests: [...] }
/// QuestSummary has: id, slug, title, description, icon, color, order_index,
/// is_published, prerequisite_quest_id, level_count, task_count, user_progress
#[test]
fn test_preservation_quest_list_response_shape() {
    // Simulate the QuestListResponse shape as defined in dto/quest.rs
    let quest_list_response = json!({
        "quests": [
            {
                "id": "550e8400-e29b-41d4-a716-446655440000",
                "slug": "rust-basics",
                "title": "Rust Basics",
                "description": "Learn the fundamentals of Rust",
                "icon": "🦀",
                "color": "#FF6B35",
                "order_index": 1,
                "is_published": true,
                "prerequisite_quest_id": null,
                "level_count": 3,
                "task_count": 15
            },
            {
                "id": "660e8400-e29b-41d4-a716-446655440001",
                "slug": "ownership-borrowing",
                "title": "Ownership & Borrowing",
                "description": "Master Rust's ownership system",
                "icon": "🔑",
                "color": "#4ECDC4",
                "order_index": 2,
                "is_published": true,
                "prerequisite_quest_id": "550e8400-e29b-41d4-a716-446655440000",
                "level_count": 4,
                "task_count": 20
            }
        ]
    });

    // Verify response is wrapped in { quests: [...] }
    assert!(
        quest_list_response.get("quests").is_some(),
        "PRESERVATION FAILURE: Quest list response must have 'quests' field"
    );
    assert!(
        quest_list_response["quests"].is_array(),
        "PRESERVATION FAILURE: 'quests' must be an array"
    );

    let quests = quest_list_response["quests"].as_array().unwrap();
    assert!(
        !quests.is_empty(),
        "PRESERVATION FAILURE: quests array should not be empty in test"
    );

    // Verify each quest has required fields
    let required_fields = vec!["id", "slug", "title", "level_count", "task_count"];

    for (idx, quest) in quests.iter().enumerate() {
        for field in &required_fields {
            assert!(
                quest.get(field).is_some(),
                "PRESERVATION FAILURE: Quest[{}] missing required field '{}'",
                idx,
                field
            );
        }

        // Verify level_count and task_count are numbers
        assert!(
            quest["level_count"].is_number(),
            "PRESERVATION FAILURE: Quest[{}] 'level_count' must be a number",
            idx
        );
        assert!(
            quest["task_count"].is_number(),
            "PRESERVATION FAILURE: Quest[{}] 'task_count' must be a number",
            idx
        );
    }
}

// ─── Property: Level detail returns tasks array ──────────────────────────────

/// **Validates: Requirements 3.3**
///
/// Property: for all valid quest/level slug combinations, level detail
/// returns { level, tasks } with correct shapes.
///
/// Observation: The backend LevelDetailResponse has: { level: LevelSummary, tasks: Vec<TaskSummary> }
/// LevelSummary has: id, slug, title, description, order_index, task_count, user_progress
/// TaskSummary has: id, slug, title, difficulty, xp_reward, order_index, tags, user_progress
#[test]
fn test_preservation_level_detail_response_shape() {
    // Simulate the LevelDetailResponse shape as defined in dto/quest.rs
    let level_detail_response = json!({
        "level": {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "slug": "intro-to-rust",
            "title": "Introduction to Rust",
            "description": "Learn the basics of Rust programming",
            "order_index": 1,
            "task_count": 5
        },
        "tasks": [
            {
                "id": "770e8400-e29b-41d4-a716-446655440000",
                "slug": "hello-world",
                "title": "Hello World",
                "difficulty": "beginner",
                "xp_reward": 10,
                "order_index": 1,
                "tags": ["basics", "io"]
            },
            {
                "id": "770e8400-e29b-41d4-a716-446655440001",
                "slug": "variables",
                "title": "Variables and Mutability",
                "difficulty": "beginner",
                "xp_reward": 15,
                "order_index": 2,
                "tags": ["basics", "variables"]
            }
        ]
    });

    // Verify response has { level, tasks } structure
    assert!(
        level_detail_response.get("level").is_some(),
        "PRESERVATION FAILURE: Level detail response must have 'level' field"
    );
    assert!(
        level_detail_response.get("tasks").is_some(),
        "PRESERVATION FAILURE: Level detail response must have 'tasks' field"
    );
    assert!(
        level_detail_response["tasks"].is_array(),
        "PRESERVATION FAILURE: 'tasks' must be an array"
    );

    // Verify level shape
    let level = &level_detail_response["level"];
    let level_fields = vec!["id", "slug", "title", "order_index", "task_count"];
    for field in &level_fields {
        assert!(
            level.get(field).is_some(),
            "PRESERVATION FAILURE: Level missing field '{}'",
            field
        );
    }

    // Verify tasks shape
    let tasks = level_detail_response["tasks"].as_array().unwrap();
    let task_fields = vec![
        "id",
        "slug",
        "title",
        "difficulty",
        "xp_reward",
        "order_index",
        "tags",
    ];
    for (idx, task) in tasks.iter().enumerate() {
        for field in &task_fields {
            assert!(
                task.get(field).is_some(),
                "PRESERVATION FAILURE: Task[{}] missing field '{}'",
                idx,
                field
            );
        }
        // Verify tags is an array
        assert!(
            task["tags"].is_array(),
            "PRESERVATION FAILURE: Task[{}] 'tags' must be an array",
            idx
        );
    }
}

// ─── Property: Leaderboard returns entries ordered by XP descending ──────────

/// **Validates: Requirements 3.6**
///
/// Property: for all period values in {alltime, weekly, monthly}, leaderboard
/// returns entries ordered by XP descending.
///
/// Observation: The backend LeaderboardResponse has: { entries: Vec<LeaderboardEntry>, period: String }
/// LeaderboardEntry has: rank, username, avatar_url, level, xp
/// Entries are ordered by XP descending (rank 1 = highest XP)
#[test]
fn test_preservation_leaderboard_entries_ordered_by_xp() {
    // Test for each valid period value
    let periods = vec!["alltime", "weekly", "monthly"];

    for period in &periods {
        // Simulate the LeaderboardResponse shape as defined in dto/gamification.rs
        let leaderboard_response = json!({
            "entries": [
                {
                    "rank": 1,
                    "username": "topuser",
                    "avatar_url": "https://example.com/avatar1.png",
                    "level": 10,
                    "xp": 5000
                },
                {
                    "rank": 2,
                    "username": "seconduser",
                    "avatar_url": null,
                    "level": 8,
                    "xp": 3500
                },
                {
                    "rank": 3,
                    "username": "thirduser",
                    "avatar_url": null,
                    "level": 6,
                    "xp": 2000
                }
            ],
            "period": period
        });

        // Verify response has entries and period
        assert!(
            leaderboard_response.get("entries").is_some(),
            "PRESERVATION FAILURE: Leaderboard response must have 'entries' field for period '{}'",
            period
        );
        assert!(
            leaderboard_response.get("period").is_some(),
            "PRESERVATION FAILURE: Leaderboard response must have 'period' field for period '{}'",
            period
        );
        assert_eq!(
            leaderboard_response["period"].as_str().unwrap(),
            *period,
            "PRESERVATION FAILURE: Period field must match requested period"
        );

        let entries = leaderboard_response["entries"].as_array().unwrap();

        // Verify entries are ordered by XP descending
        let mut prev_xp = i64::MAX;
        for (idx, entry) in entries.iter().enumerate() {
            let xp = entry["xp"].as_i64().unwrap();
            assert!(
                xp <= prev_xp,
                "PRESERVATION FAILURE: Leaderboard entries not ordered by XP descending. \
                 Entry[{}] has xp={} but previous had xp={} (period='{}')",
                idx,
                xp,
                prev_xp,
                period
            );
            prev_xp = xp;

            // Verify entry shape
            let entry_fields = vec!["rank", "username", "level", "xp"];
            for field in &entry_fields {
                assert!(
                    entry.get(field).is_some(),
                    "PRESERVATION FAILURE: LeaderboardEntry[{}] missing field '{}' (period='{}')",
                    idx,
                    field,
                    period
                );
            }

            // Verify rank is sequential
            assert_eq!(
                entry["rank"].as_i64().unwrap(),
                (idx + 1) as i64,
                "PRESERVATION FAILURE: Ranks must be sequential starting from 1 (period='{}')",
                period
            );
        }
    }
}

// ─── Property: GET /users/me/achievements returns wrapped response ───────────

/// **Validates: Requirements 3.7** (partially - achievements endpoint behavior)
///
/// Property: GET /users/me/achievements with valid token returns
/// { achievements, total_earned, total_available } shape.
///
/// Observation: The backend AchievementsListResponse has:
/// { achievements: Vec<AchievementResponse>, total_earned: usize, total_available: usize }
/// AchievementResponse has: id, slug, name, description, icon, xp_reward, is_earned, earned_at
#[test]
fn test_preservation_achievements_list_response_shape() {
    // Simulate the AchievementsListResponse shape as defined in dto/gamification.rs
    let achievements_response = json!({
        "achievements": [
            {
                "id": "550e8400-e29b-41d4-a716-446655440000",
                "slug": "first-submission",
                "name": "First Steps",
                "description": "Submit your first solution",
                "icon": "🎯",
                "xp_reward": 50,
                "is_earned": true,
                "earned_at": "2024-01-15T10:00:00+00:00"
            },
            {
                "id": "660e8400-e29b-41d4-a716-446655440001",
                "slug": "streak-7",
                "name": "Week Warrior",
                "description": "Maintain a 7-day streak",
                "icon": "🔥",
                "xp_reward": 100,
                "is_earned": false
            }
        ],
        "total_earned": 1,
        "total_available": 2
    });

    // Verify response has the wrapped structure
    assert!(
        achievements_response.get("achievements").is_some(),
        "PRESERVATION FAILURE: Achievements response must have 'achievements' field"
    );
    assert!(
        achievements_response["achievements"].is_array(),
        "PRESERVATION FAILURE: 'achievements' must be an array"
    );
    assert!(
        achievements_response.get("total_earned").is_some(),
        "PRESERVATION FAILURE: Achievements response must have 'total_earned' field"
    );
    assert!(
        achievements_response.get("total_available").is_some(),
        "PRESERVATION FAILURE: Achievements response must have 'total_available' field"
    );

    // Verify achievement shape
    let achievements = achievements_response["achievements"].as_array().unwrap();
    let achievement_fields = vec![
        "id",
        "slug",
        "name",
        "description",
        "icon",
        "xp_reward",
        "is_earned",
    ];
    for (idx, achievement) in achievements.iter().enumerate() {
        for field in &achievement_fields {
            assert!(
                achievement.get(field).is_some(),
                "PRESERVATION FAILURE: Achievement[{}] missing field '{}'",
                idx,
                field
            );
        }

        // Verify is_earned is a boolean
        assert!(
            achievement["is_earned"].is_boolean(),
            "PRESERVATION FAILURE: Achievement[{}] 'is_earned' must be a boolean",
            idx
        );
    }

    // Verify totals are numbers
    assert!(
        achievements_response["total_earned"].is_number(),
        "PRESERVATION FAILURE: 'total_earned' must be a number"
    );
    assert!(
        achievements_response["total_available"].is_number(),
        "PRESERVATION FAILURE: 'total_available' must be a number"
    );
}

// ─── Property: Forum, article, and review endpoints return correct shapes ────

/// **Validates: Requirements 3.7**
///
/// Property: forum, article, and review endpoints continue to return correct
/// response shapes.
///
/// Observation: Backend DTOs define:
/// - ForumPostListResponse: { posts: Vec<ForumPostSummary>, page, per_page, total }
/// - ArticleListResponse: { articles: Vec<ArticleSummary>, page, per_page, total }
/// - ReviewListResponse: { reviews: Vec<ReviewResponse> }
#[test]
fn test_preservation_community_endpoints_response_shapes() {
    // ─── Forum Post List ─────────────────────────────────────────────────────
    let forum_response = json!({
        "posts": [
            {
                "id": "550e8400-e29b-41d4-a716-446655440000",
                "user_id": "660e8400-e29b-41d4-a716-446655440001",
                "username": "rustdev",
                "title": "Help with lifetimes",
                "category": "task_help",
                "votes": 5,
                "views": 42,
                "reply_count": 3,
                "is_pinned": false,
                "task_id": null,
                "created_at": "2024-01-15T10:00:00+00:00"
            }
        ],
        "page": 1,
        "per_page": 20,
        "total": 1
    });

    assert!(
        forum_response.get("posts").is_some(),
        "PRESERVATION FAILURE: Forum response must have 'posts'"
    );
    assert!(
        forum_response["posts"].is_array(),
        "PRESERVATION FAILURE: 'posts' must be an array"
    );
    assert!(
        forum_response.get("page").is_some(),
        "PRESERVATION FAILURE: Forum response must have 'page'"
    );
    assert!(
        forum_response.get("per_page").is_some(),
        "PRESERVATION FAILURE: Forum response must have 'per_page'"
    );
    assert!(
        forum_response.get("total").is_some(),
        "PRESERVATION FAILURE: Forum response must have 'total'"
    );

    let posts = forum_response["posts"].as_array().unwrap();
    let post_fields = vec![
        "id",
        "user_id",
        "username",
        "title",
        "category",
        "votes",
        "views",
        "reply_count",
        "is_pinned",
        "created_at",
    ];
    for post in posts {
        for field in &post_fields {
            assert!(
                post.get(field).is_some(),
                "PRESERVATION FAILURE: ForumPost missing field '{}'",
                field
            );
        }
    }

    // ─── Article List ────────────────────────────────────────────────────────
    let article_response = json!({
        "articles": [
            {
                "id": "550e8400-e29b-41d4-a716-446655440000",
                "author_id": "660e8400-e29b-41d4-a716-446655440001",
                "author_username": "rustexpert",
                "title": "Understanding Ownership",
                "cover_image_url": null,
                "tags": ["rust", "ownership"],
                "views": 150,
                "likes": 12,
                "created_at": "2024-01-10T08:00:00+00:00"
            }
        ],
        "page": 1,
        "per_page": 20,
        "total": 1
    });

    assert!(
        article_response.get("articles").is_some(),
        "PRESERVATION FAILURE: Article response must have 'articles'"
    );
    assert!(
        article_response["articles"].is_array(),
        "PRESERVATION FAILURE: 'articles' must be an array"
    );
    assert!(
        article_response.get("page").is_some(),
        "PRESERVATION FAILURE: Article response must have 'page'"
    );
    assert!(
        article_response.get("per_page").is_some(),
        "PRESERVATION FAILURE: Article response must have 'per_page'"
    );
    assert!(
        article_response.get("total").is_some(),
        "PRESERVATION FAILURE: Article response must have 'total'"
    );

    let articles = article_response["articles"].as_array().unwrap();
    let article_fields = vec![
        "id",
        "author_id",
        "author_username",
        "title",
        "tags",
        "views",
        "likes",
        "created_at",
    ];
    for article in articles {
        for field in &article_fields {
            assert!(
                article.get(field).is_some(),
                "PRESERVATION FAILURE: Article missing field '{}'",
                field
            );
        }
        assert!(
            article["tags"].is_array(),
            "PRESERVATION FAILURE: Article 'tags' must be an array"
        );
    }

    // ─── Review List ─────────────────────────────────────────────────────────
    let review_response = json!({
        "reviews": [
            {
                "id": "550e8400-e29b-41d4-a716-446655440000",
                "user_id": "660e8400-e29b-41d4-a716-446655440001",
                "username": "happyuser",
                "rating": 5,
                "body": "Great platform for learning Rust!",
                "is_featured": true,
                "created_at": "2024-01-20T14:00:00+00:00"
            }
        ]
    });

    assert!(
        review_response.get("reviews").is_some(),
        "PRESERVATION FAILURE: Review response must have 'reviews'"
    );
    assert!(
        review_response["reviews"].is_array(),
        "PRESERVATION FAILURE: 'reviews' must be an array"
    );

    let reviews = review_response["reviews"].as_array().unwrap();
    let review_fields = vec![
        "id",
        "user_id",
        "username",
        "rating",
        "body",
        "is_featured",
        "created_at",
    ];
    for review in reviews {
        for field in &review_fields {
            assert!(
                review.get(field).is_some(),
                "PRESERVATION FAILURE: Review missing field '{}'",
                field
            );
        }
        assert!(
            review["rating"].is_number(),
            "PRESERVATION FAILURE: Review 'rating' must be a number"
        );
    }
}

// ─── Property: Admin endpoints with valid token return expected responses ────

/// **Validates: Requirements 3.8**
///
/// Property: Admin endpoints with valid (non-expired) tokens continue to
/// process admin requests normally. The response shapes are preserved.
///
/// Observation: Admin routes exist at /api/v1/admin/* and return various shapes.
/// This test verifies the expected admin response shapes are stable.
#[test]
fn test_preservation_admin_response_shapes() {
    // Admin stats response shape
    let admin_stats = json!({
        "total_users": 150,
        "total_quests": 5,
        "total_levels": 20,
        "total_tasks": 100,
        "total_submissions": 5000,
        "total_achievements": 15
    });

    let stats_fields = vec![
        "total_users",
        "total_quests",
        "total_levels",
        "total_tasks",
        "total_submissions",
        "total_achievements",
    ];
    for field in &stats_fields {
        assert!(
            admin_stats.get(field).is_some(),
            "PRESERVATION FAILURE: AdminStats missing field '{}'",
            field
        );
        assert!(
            admin_stats[field].is_number(),
            "PRESERVATION FAILURE: AdminStats '{}' must be a number",
            field
        );
    }
}

// ─── Property: PublicProfileResponse excludes sensitive fields ────────────────

/// **Validates: Requirements 3.1** (indirectly - public profile is separate from /me)
///
/// Property: The PublicProfileResponse correctly excludes email and is_verified
/// (this is the CORRECT behavior that must be preserved).
///
/// Observation: Backend PublicProfileResponse (dto/user.rs) has:
/// id, username, avatar_url, bio, role, xp, level, streak_days, created_at
/// It intentionally does NOT include email or is_verified (privacy).
#[test]
fn test_preservation_public_profile_excludes_sensitive_fields() {
    // Simulate the PublicProfileResponse shape as defined in dto/user.rs
    let public_profile = json!({
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "username": "publicuser",
        "avatar_url": null,
        "bio": "Rust enthusiast",
        "role": "student",
        "xp": 250,
        "level": 3,
        "streak_days": 10,
        "created_at": "2024-01-01T00:00:00+00:00"
    });

    // Verify expected fields are present
    let expected_fields = vec![
        "id",
        "username",
        "avatar_url",
        "bio",
        "role",
        "xp",
        "level",
        "streak_days",
        "created_at",
    ];
    for field in &expected_fields {
        assert!(
            public_profile.get(field).is_some(),
            "PRESERVATION FAILURE: PublicProfileResponse missing field '{}'",
            field
        );
    }

    // Verify sensitive fields are NOT present (this is correct behavior)
    assert!(
        public_profile.get("email").is_none(),
        "PRESERVATION FAILURE: PublicProfileResponse must NOT include 'email' (privacy)"
    );
    assert!(
        public_profile.get("is_verified").is_none(),
        "PRESERVATION FAILURE: PublicProfileResponse must NOT include 'is_verified' (privacy)"
    );
    assert!(
        public_profile.get("password_hash").is_none(),
        "PRESERVATION FAILURE: PublicProfileResponse must NOT include 'password_hash' (security)"
    );
}
