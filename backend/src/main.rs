mod config;
mod dto;
mod error;
mod routes;
mod services;

use std::sync::Arc;

use axum::{routing::get, Json, Router};
use sea_orm::Database;
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::config::Config;

/// Shared application state available to all route handlers.
#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub config: Arc<Config>,
}

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

    // Connect to the database
    let db = Database::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Connected to database");

    let state = AppState {
        db,
        config: Arc::new(config.clone()),
    };

    let app = Router::new()
        .route("/api/v1/health", get(health_check))
        .merge(routes::api_router())
        .with_state(state);

    let addr = format!("0.0.0.0:{}", config.server_port);
    tracing::info!("RustGym backend starting on {}", addr);

    let listener = TcpListener::bind(&addr).await.expect("Failed to bind address");
    axum::serve(listener, app).await.expect("Server error");
}
