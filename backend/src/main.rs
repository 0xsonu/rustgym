mod config;
mod error;
mod services;

use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::config::Config;

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "rustgym-backend"
    }))
}

#[tokio::main]
async fn main() {
    // Load .env file (ignore error if not present)
    dotenvy::dotenv().ok();

    // Set up tracing subscriber
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();

    let app = Router::new().route("/api/v1/health", get(health_check));

    let addr = format!("0.0.0.0:{}", config.server_port);
    tracing::info!("RustGym backend starting on {}", addr);

    let listener = TcpListener::bind(&addr).await.expect("Failed to bind address");
    axum::serve(listener, app).await.expect("Server error");
}
