use serde::Deserialize;

use super::{ApiClient, ApiResponse, ApiResult, LevelSummary, QuestSummary, TaskSummary};
use super::error::AppError;

// ─── Local Response Types ───────────────────────────────────────────────────

/// Deserialization target for GET /api/v1/quests.
#[derive(Debug, Clone, Deserialize)]
struct QuestsListResponse {
    pub quests: Vec<QuestSummary>,
}

/// Deserialization target for GET /api/v1/quests/{slug}.
/// Backend returns: { quest: QuestSummary, levels: [...] }
#[derive(Debug, Clone, Deserialize)]
struct QuestDetailResponse {
    pub quest: QuestSummaryInner,
    pub levels: Vec<LevelSummary>,
}

/// The quest object nested inside QuestDetailResponse.
#[derive(Debug, Clone, Deserialize)]
struct QuestSummaryInner {
    pub slug: String,
    pub title: String,
}

/// Deserialization target for GET /api/v1/quests/{quest_slug}/levels/{level_slug}.
/// Backend returns: { level: LevelSummary, tasks: [...] }
#[derive(Debug, Clone, Deserialize)]
struct LevelDetailResponse {
    pub level: LevelSummaryInner,
    pub tasks: Vec<TaskSummary>,
}

/// The level object nested inside LevelDetailResponse.
#[derive(Debug, Clone, Deserialize)]
struct LevelSummaryInner {
    pub slug: String,
    pub title: String,
}

// ─── Quest/Level Fetch Implementations ──────────────────────────────────────

impl ApiClient {
    /// Fetch the list of all quests.
    ///
    /// Spawns a tokio task that GETs `/api/v1/quests` and sends the result
    /// through the response channel.
    pub fn fetch_quests(&self) {
        let client = self.client.clone();
        let base_url = self.base_url.clone();
        let token = self.token.clone();
        let tx = self.response_tx.clone();

        tokio::spawn(async move {
            let result = do_fetch_quests(&client, &base_url, token.as_deref()).await;
            let _ = tx.send(result);
        });
    }

    /// Fetch detail for a single quest including its levels.
    ///
    /// Spawns a tokio task that GETs `/api/v1/quests/{slug}` and sends the
    /// result through the response channel.
    pub fn fetch_quest_detail(&self, slug: String) {
        let client = self.client.clone();
        let base_url = self.base_url.clone();
        let token = self.token.clone();
        let tx = self.response_tx.clone();

        tokio::spawn(async move {
            let result = do_fetch_quest_detail(&client, &base_url, token.as_deref(), &slug).await;
            let _ = tx.send(result);
        });
    }

    /// Fetch detail for a single level including its tasks.
    ///
    /// Spawns a tokio task that GETs `/api/v1/quests/{quest_slug}/levels/{level_slug}`
    /// and sends the result through the response channel.
    pub fn fetch_level_detail(&self, quest_slug: String, level_slug: String) {
        let client = self.client.clone();
        let base_url = self.base_url.clone();
        let token = self.token.clone();
        let tx = self.response_tx.clone();

        tokio::spawn(async move {
            let result = do_fetch_level_detail(
                &client,
                &base_url,
                token.as_deref(),
                &quest_slug,
                &level_slug,
            )
            .await;
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
    } else if !status.is_success() {
        None // Caller handles other non-success statuses
    } else {
        None
    }
}

/// Perform the GET /api/v1/quests request.
async fn do_fetch_quests(
    client: &reqwest::Client,
    base_url: &str,
    token: Option<&str>,
) -> ApiResult {
    let url = format!("{}/api/v1/quests", base_url);

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

    let list_resp: QuestsListResponse = response.json().await.map_err(|e| {
        AppError::Other(format!("Failed to parse quests response: {}", e))
    })?;

    Ok(ApiResponse::QuestList {
        quests: list_resp.quests,
    })
}

/// Perform the GET /api/v1/quests/{slug} request.
async fn do_fetch_quest_detail(
    client: &reqwest::Client,
    base_url: &str,
    token: Option<&str>,
    slug: &str,
) -> ApiResult {
    let url = format!("{}/api/v1/quests/{}", base_url, slug);

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

    let detail_resp: QuestDetailResponse = response.json().await.map_err(|e| {
        AppError::Other(format!("Failed to parse quest detail response: {}", e))
    })?;

    Ok(ApiResponse::QuestDetail {
        slug: detail_resp.quest.slug,
        title: detail_resp.quest.title,
        levels: detail_resp.levels,
    })
}

/// Perform the GET /api/v1/quests/{quest_slug}/levels/{level_slug} request.
async fn do_fetch_level_detail(
    client: &reqwest::Client,
    base_url: &str,
    token: Option<&str>,
    quest_slug: &str,
    level_slug: &str,
) -> ApiResult {
    let url = format!(
        "{}/api/v1/quests/{}/levels/{}",
        base_url, quest_slug, level_slug
    );

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

    let detail_resp: LevelDetailResponse = response.json().await.map_err(|e| {
        AppError::Other(format!("Failed to parse level detail response: {}", e))
    })?;

    Ok(ApiResponse::LevelDetail {
        slug: detail_resp.level.slug,
        title: detail_resp.level.title,
        tasks: detail_resp.tasks,
    })
}
