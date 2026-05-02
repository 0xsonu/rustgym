use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── Admin Stats ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct AdminStatsResponse {
    pub total_users: u64,
    pub submissions_today: u64,
    pub pass_rate: f64,
    pub popular_tasks: Vec<PopularTask>,
}

#[derive(Debug, Serialize)]
pub struct PopularTask {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub submission_count: u64,
}

// ─── Quest CRUD ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateQuestRequest {
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub order_index: i32,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub is_published: Option<bool>,
    pub prerequisite_quest_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuestRequest {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub order_index: Option<i32>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub is_published: Option<bool>,
    pub prerequisite_quest_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct QuestResponse {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub order_index: i32,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub is_published: bool,
    pub prerequisite_quest_id: Option<Uuid>,
    pub created_at: String,
    pub updated_at: String,
}

// ─── Level CRUD ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateLevelRequest {
    pub quest_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub order_index: i32,
    pub is_published: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLevelRequest {
    pub quest_id: Option<Uuid>,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub order_index: Option<i32>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct LevelResponse {
    pub id: Uuid,
    pub quest_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub order_index: i32,
    pub is_published: bool,
    pub created_at: String,
    pub updated_at: String,
}

// ─── Task CRUD ───────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub level_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description_md: String,
    pub starter_code: String,
    pub solution_code: String,
    pub test_code: String,
    pub cargo_toml: String,
    pub difficulty: String,
    pub xp_reward: i32,
    pub order_index: i32,
    pub is_published: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub hint_md: Option<String>,
    pub syntest_rules: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub level_id: Option<Uuid>,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description_md: Option<String>,
    pub starter_code: Option<String>,
    pub solution_code: Option<String>,
    pub test_code: Option<String>,
    pub cargo_toml: Option<String>,
    pub difficulty: Option<String>,
    pub xp_reward: Option<i32>,
    pub order_index: Option<i32>,
    pub is_published: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub hint_md: Option<String>,
    pub syntest_rules: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub id: Uuid,
    pub level_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description_md: String,
    pub starter_code: String,
    pub solution_code: String,
    pub test_code: String,
    pub cargo_toml: String,
    pub difficulty: String,
    pub xp_reward: i32,
    pub order_index: i32,
    pub is_published: bool,
    pub tags: Vec<String>,
    pub hint_md: Option<String>,
    pub syntest_rules: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ─── Achievement CRUD ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateAchievementRequest {
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub xp_reward: Option<i32>,
    pub condition_type: String,
    pub condition_value: i32,
    pub condition_meta: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAchievementRequest {
    pub slug: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub xp_reward: Option<i32>,
    pub condition_type: Option<String>,
    pub condition_value: Option<i32>,
    pub condition_meta: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct AchievementResponse {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub xp_reward: i32,
    pub condition_type: String,
    pub condition_value: i32,
    pub condition_meta: Option<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

// ─── User Management ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct UserSearchParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AdminUserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
    pub xp: i32,
    pub level: i32,
    pub streak_days: i32,
    pub is_verified: bool,
    pub is_banned: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct AdminUserListResponse {
    pub users: Vec<AdminUserResponse>,
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRoleRequest {
    pub role: String,
}

// ─── Submission List ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SubmissionSearchParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct AdminSubmissionResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub task_id: Uuid,
    pub status: String,
    pub duration_ms: i32,
    pub memory_kb: i32,
    pub xp_awarded: i32,
    pub attempt_number: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct AdminSubmissionListResponse {
    pub submissions: Vec<AdminSubmissionResponse>,
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
}
