pub mod auth;
pub mod error;
pub mod quests;
pub mod tasks;

use serde::Deserialize;
use tokio::sync::mpsc;

use self::error::AppError;

// ─── API Response Data Types ────────────────────────────────────────────────

/// Summary data for a quest as returned by the API list endpoint.
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct QuestSummary {
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub level_count: i64,
    pub task_count: i64,
    pub user_progress: Option<QuestProgress>,
}

/// Progress data for a quest.
#[derive(Debug, Clone, Deserialize)]
pub struct QuestProgress {
    pub tasks_completed: i32,
    pub tasks_total: i32,
    pub is_completed: bool,
}

/// Summary data for a level within a quest.
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct LevelSummary {
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub task_count: i64,
    pub user_progress: Option<LevelProgress>,
}

/// Progress data for a level.
#[derive(Debug, Clone, Deserialize)]
pub struct LevelProgress {
    pub tasks_completed: i32,
    pub tasks_total: i32,
    pub is_completed: bool,
}

/// Summary data for a task within a level.
#[derive(Debug, Clone, Deserialize)]
pub struct TaskSummary {
    pub slug: String,
    pub title: String,
    pub difficulty: String,
    pub xp_reward: i32,
    pub user_progress: Option<TaskProgress>,
}

/// Progress data for a task.
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct TaskProgress {
    pub status: String,
    pub attempts: i32,
}

/// Full detail data for a single task.
#[derive(Debug, Clone, Deserialize)]
pub struct TaskDetailData {
    pub slug: String,
    pub title: String,
    pub description_md: String,
    pub starter_code: String,
    pub difficulty: String,
    pub xp_reward: i32,
    pub hint_md: Option<String>,
}

/// Result of a code submission.
#[derive(Debug, Clone, Deserialize)]
pub struct SubmissionResult {
    pub status: String,
    pub test_results: serde_json::Value,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub duration_ms: i32,
    pub xp_awarded: i32,
    pub attempt_number: i32,
    pub leveled_up: bool,
    pub new_level: Option<i32>,
}

// ─── API Response Enum ──────────────────────────────────────────────────────

/// Enum representing all possible successful API responses.
/// Each variant corresponds to a specific endpoint's response shape.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ApiResponse {
    /// Successful login with tokens and user info.
    LoginSuccess {
        access_token: String,
        refresh_token: String,
        username: String,
        xp: i32,
        level: i32,
        streak_days: i32,
    },
    /// List of quests returned from GET /api/v1/quests.
    QuestList { quests: Vec<QuestSummary> },
    /// Detail for a single quest including its levels.
    QuestDetail {
        slug: String,
        title: String,
        levels: Vec<LevelSummary>,
    },
    /// Detail for a single level including its tasks.
    LevelDetail {
        slug: String,
        title: String,
        tasks: Vec<TaskSummary>,
    },
    /// Full task detail data.
    TaskDetail(TaskDetailData),
    /// Submission result after running tests.
    SubmitResult(SubmissionResult),
}

// ─── ApiResult Type Alias ───────────────────────────────────────────────────

/// Result type for API responses sent through the channel.
pub type ApiResult = Result<ApiResponse, AppError>;

// ─── ApiClient ──────────────────────────────────────────────────────────────

/// HTTP client for communicating with the RustGym backend API.
///
/// Each API method spawns a tokio task that sends the result through
/// `response_tx`. The main event loop receives results via the corresponding
/// receiver end of the channel.
pub struct ApiClient {
    /// The underlying HTTP client.
    client: reqwest::Client,
    /// Base URL of the API (e.g., "https://rustgym.dev").
    base_url: String,
    /// Optional bearer token for authenticated requests.
    token: Option<String>,
    /// Channel sender for delivering API results back to the event loop.
    response_tx: mpsc::UnboundedSender<ApiResult>,
}

impl ApiClient {
    /// Create a new ApiClient.
    ///
    /// # Arguments
    /// * `base_url` - The base URL of the RustGym API.
    /// * `token` - Optional authentication token.
    /// * `response_tx` - Channel sender for API results.
    pub fn new(
        base_url: String,
        token: Option<String>,
        response_tx: mpsc::UnboundedSender<ApiResult>,
    ) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
            token,
            response_tx,
        }
    }

    /// Update the authentication token (e.g., after login or session expiry).
    #[allow(dead_code)]
    pub fn set_token(&mut self, token: Option<String>) {
        self.token = token;
    }
}
