use serde::Serialize;

use super::{ApiClient, ApiResponse, ApiResult, SubmissionResult, TaskDetailData};
use super::error::AppError;

// ─── Local Request Types ────────────────────────────────────────────────────

/// JSON body for the submit request.
#[derive(Serialize)]
struct SubmitRequest {
    code: String,
}

// ─── Task Detail & Submit Implementations ───────────────────────────────────

impl ApiClient {
    /// Fetch full detail for a single task.
    ///
    /// Spawns a tokio task that GETs `/api/v1/tasks/{slug}` and sends the
    /// result through the response channel.
    pub fn fetch_task_detail(&self, slug: String) {
        let client = self.client.clone();
        let base_url = self.base_url.clone();
        let token = self.token.clone();
        let tx = self.response_tx.clone();

        tokio::spawn(async move {
            let result = do_fetch_task_detail(&client, &base_url, token.as_deref(), &slug).await;
            let _ = tx.send(result);
        });
    }

    /// Submit a solution for a task.
    ///
    /// Spawns a tokio task that POSTs to `/api/v1/tasks/{slug}/submit` with
    /// the provided code and sends the result through the response channel.
    pub fn submit_solution(&self, slug: String, code: String) {
        let client = self.client.clone();
        let base_url = self.base_url.clone();
        let token = self.token.clone();
        let tx = self.response_tx.clone();

        tokio::spawn(async move {
            let result =
                do_submit_solution(&client, &base_url, token.as_deref(), &slug, code).await;
            let _ = tx.send(result);
        });
    }
}

// ─── Helper Functions ───────────────────────────────────────────────────────

/// Map an HTTP status code to the appropriate AppError.
fn map_status_error(status: reqwest::StatusCode) -> Option<AppError> {
    if status == reqwest::StatusCode::UNAUTHORIZED {
        Some(AppError::Unauthorized)
    } else if status == reqwest::StatusCode::NOT_FOUND {
        Some(AppError::NotFound)
    } else if status.is_server_error() {
        Some(AppError::ServerError)
    } else {
        None
    }
}

/// Perform the GET /api/v1/tasks/{slug} request.
async fn do_fetch_task_detail(
    client: &reqwest::Client,
    base_url: &str,
    token: Option<&str>,
    slug: &str,
) -> ApiResult {
    let url = format!("{}/api/v1/tasks/{}", base_url, slug);

    let mut request = client.get(&url);
    if let Some(t) = token {
        request = request.header("Authorization", format!("Bearer {}", t));
    }

    let response = request.send().await?;
    let status = response.status();

    if let Some(err) = map_status_error(status) {
        return Err(err);
    }

    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(AppError::Other(body));
    }

    let detail: TaskDetailData = response.json().await.map_err(|e| {
        AppError::Other(format!("Failed to parse task detail response: {}", e))
    })?;

    Ok(ApiResponse::TaskDetail(detail))
}

/// Perform the POST /api/v1/tasks/{slug}/submit request.
async fn do_submit_solution(
    client: &reqwest::Client,
    base_url: &str,
    token: Option<&str>,
    slug: &str,
    code: String,
) -> ApiResult {
    let url = format!("{}/api/v1/tasks/{}/submit", base_url, slug);

    let mut request = client.post(&url).json(&SubmitRequest { code });
    if let Some(t) = token {
        request = request.header("Authorization", format!("Bearer {}", t));
    }

    let response = request.send().await?;
    let status = response.status();

    if let Some(err) = map_status_error(status) {
        return Err(err);
    }

    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(AppError::Other(body));
    }

    let result: SubmissionResult = response.json().await.map_err(|e| {
        AppError::Other(format!("Failed to parse submission response: {}", e))
    })?;

    Ok(ApiResponse::SubmitResult(result))
}
