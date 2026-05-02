use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

// ─── Request/Response Types ──────────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct RunTestRequest {
    code_b64: String,
    test_b64: String,
    cargo_toml_b64: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    syntest_rules: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct PlaygroundRequest {
    code_b64: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RunTestResponse {
    pub status: String,
    pub stdout: String,
    pub stderr: String,
    pub tests: Vec<TestResult>,
    pub duration_ms: i64,
    pub memory_kb: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlaygroundResponse {
    pub status: String,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: i64,
}

// ─── Runner Client ───────────────────────────────────────────────────────────

/// HTTP client for communicating with the Runner service.
pub struct RunnerClient {
    client: reqwest::Client,
    base_url: String,
}

impl RunnerClient {
    pub fn new(runner_url: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: runner_url.trim_end_matches('/').to_string(),
        }
    }

    /// Call the runner's /run/test endpoint.
    /// Base64-encodes all payloads before sending.
    pub async fn run_test(
        &self,
        code: &str,
        test_code: &str,
        cargo_toml: &str,
        syntest_rules: Option<Vec<String>>,
    ) -> Result<RunTestResponse, AppError> {
        let request = RunTestRequest {
            code_b64: BASE64.encode(code),
            test_b64: BASE64.encode(test_code),
            cargo_toml_b64: BASE64.encode(cargo_toml),
            syntest_rules,
        };

        let url = format!("{}/run/test", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Runner service request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!(
                "Runner service returned {}: {}",
                status, body
            )));
        }

        response
            .json::<RunTestResponse>()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to parse runner response: {}", e)))
    }

    /// Call the runner's /run/playground endpoint.
    /// Base64-encodes the code before sending.
    pub async fn run_playground(&self, code: &str) -> Result<PlaygroundResponse, AppError> {
        let request = PlaygroundRequest {
            code_b64: BASE64.encode(code),
        };

        let url = format!("{}/run/playground", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Runner service request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!(
                "Runner service returned {}: {}",
                status, body
            )));
        }

        response
            .json::<PlaygroundResponse>()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to parse runner response: {}", e)))
    }
}
