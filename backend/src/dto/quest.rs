use serde::Serialize;
use uuid::Uuid;

// ─── Quest List ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct QuestListResponse {
    pub quests: Vec<QuestSummary>,
}

#[derive(Debug, Serialize)]
pub struct QuestSummary {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub order_index: i32,
    pub is_published: bool,
    pub prerequisite_quest_id: Option<Uuid>,
    pub level_count: i64,
    pub task_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_progress: Option<QuestProgress>,
}

#[derive(Debug, Serialize)]
pub struct QuestProgress {
    pub tasks_completed: i32,
    pub tasks_total: i32,
    pub is_completed: bool,
}

// ─── Quest Detail ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct QuestDetailResponse {
    pub quest: QuestSummary,
    pub levels: Vec<LevelSummary>,
}

#[derive(Debug, Serialize)]
pub struct LevelSummary {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub order_index: i32,
    pub task_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_progress: Option<LevelProgress>,
}

#[derive(Debug, Serialize)]
pub struct LevelProgress {
    pub tasks_completed: i32,
    pub tasks_total: i32,
    pub is_completed: bool,
}

// ─── Level Detail ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct LevelDetailResponse {
    pub level: LevelSummary,
    pub tasks: Vec<TaskSummary>,
}

#[derive(Debug, Serialize)]
pub struct TaskSummary {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub difficulty: String,
    pub xp_reward: i32,
    pub order_index: i32,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_progress: Option<TaskProgress>,
}

#[derive(Debug, Serialize)]
pub struct TaskProgress {
    pub status: String,
    pub attempts: i32,
    pub completed_at: Option<String>,
}

// ─── Task Detail ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct TaskDetailResponse {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub description_md: String,
    pub starter_code: String,
    pub difficulty: String,
    pub xp_reward: i32,
    pub tags: Vec<String>,
    pub hint_md: Option<String>,
    pub order_index: i32,
}

// ─── Submissions (placeholder) ───────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct SubmissionListResponse {
    pub submissions: Vec<SubmissionSummary>,
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
}

#[derive(Debug, Serialize)]
pub struct SubmissionSummary {
    pub id: Uuid,
    pub code: String,
    pub status: String,
    pub test_results: serde_json::Value,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: i32,
    pub memory_kb: i32,
    pub xp_awarded: i32,
    pub attempt_number: i32,
    pub created_at: String,
}
