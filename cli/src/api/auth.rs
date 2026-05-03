use serde::{Deserialize, Serialize};

use super::{ApiClient, ApiResponse, ApiResult};
use super::error::AppError;

// ─── Local Request/Response Types ───────────────────────────────────────────

/// JSON body for the login request.
#[derive(Serialize)]
struct LoginRequest {
    email: String,
    password: String,
}

/// User info nested within the login response.
#[derive(Debug, Clone, Deserialize)]
struct UserInfo {
    pub username: String,
    pub xp: i32,
    pub level: i32,
    pub streak_days: i32,
}

/// Deserialization target for the login endpoint response.
#[derive(Debug, Clone, Deserialize)]
struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserInfo,
}

// ─── Login Implementation ───────────────────────────────────────────────────

impl ApiClient {
    /// Initiate a login request in the background.
    ///
    /// Spawns a tokio task that POSTs credentials to `/api/v1/auth/login`
    /// and sends the result through the response channel.
    pub fn login(&self, email: String, password: String) {
        let client = self.client.clone();
        let base_url = self.base_url.clone();
        let tx = self.response_tx.clone();

        tokio::spawn(async move {
            let result = do_login(&client, &base_url, email, password).await;
            let _ = tx.send(result);
        });
    }
}

/// Perform the actual login HTTP request and map the response.
async fn do_login(
    client: &reqwest::Client,
    base_url: &str,
    email: String,
    password: String,
) -> ApiResult {
    let url = format!("{}/api/v1/auth/login", base_url);

    let response = client
        .post(&url)
        .json(&LoginRequest { email, password })
        .send()
        .await?;

    let status = response.status();

    if status == reqwest::StatusCode::UNAUTHORIZED {
        return Err(AppError::Unauthorized);
    }

    if status.is_server_error() {
        return Err(AppError::ServerError);
    }

    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(AppError::Other(body));
    }

    let login_resp: LoginResponse = response.json().await.map_err(|e| {
        AppError::Other(format!("Failed to parse login response: {}", e))
    })?;

    Ok(ApiResponse::LoginSuccess {
        access_token: login_resp.access_token,
        refresh_token: login_resp.refresh_token,
        username: login_resp.user.username,
        xp: login_resp.user.xp,
        level: login_resp.user.level,
        streak_days: login_resp.user.streak_days,
    })
}
