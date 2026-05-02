use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(Debug, Deserialize)]
struct RunTestRequest {
    /// Base64-encoded user code
    code: String,
    /// Base64-encoded test code
    test_code: String,
    /// Base64-encoded Cargo.toml
    cargo_toml: String,
}

#[derive(Debug, Deserialize)]
struct PlaygroundRequest {
    /// Base64-encoded user code
    code: String,
}

#[derive(Debug, Serialize)]
struct RunResponse {
    status: String,
    stdout: String,
    stderr: String,
    duration_ms: u64,
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "rustgym-runner"
    }))
}

async fn run_test(Json(_payload): Json<RunTestRequest>) -> Json<Value> {
    // TODO: Implement Docker-based test execution
    Json(json!({
        "status": "not_implemented",
        "message": "Test execution not yet implemented"
    }))
}

async fn run_playground(Json(_payload): Json<PlaygroundRequest>) -> Json<Value> {
    // TODO: Implement Docker-based playground execution
    Json(json!({
        "status": "not_implemented",
        "message": "Playground execution not yet implemented"
    }))
}

#[tokio::main]
async fn main() {
    // Set up tracing subscriber
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/run/test", post(run_test))
        .route("/run/playground", post(run_playground));

    let addr = "0.0.0.0:3001";
    tracing::info!("RustGym runner service starting on {}", addr);

    let listener = TcpListener::bind(addr).await.expect("Failed to bind address");
    axum::serve(listener, app).await.expect("Server error");
}
