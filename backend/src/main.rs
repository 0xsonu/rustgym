mod config;
mod dto;
mod error;
mod middleware;
mod routes;
mod services;

use std::sync::Arc;

use axum::{routing::get, Json, Router};
use sea_orm::Database;
use sea_orm_migration::MigratorTrait as _;
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::config::Config;

/// Shared application state available to all route handlers.
#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub config: Arc<Config>,
    pub redis: Option<deadpool_redis::Pool>,
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

    // Run migrations
    migration::Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");
    tracing::info!("Migrations applied successfully");

    // Try to connect to Redis (graceful degradation if unavailable)
    let redis = match create_redis_pool(&config.redis_url) {
        Ok(pool) => {
            // Verify the connection works
            match pool.get().await {
                Ok(_) => {
                    tracing::info!("Connected to Redis");
                    Some(pool)
                }
                Err(e) => {
                    tracing::warn!(
                        "Redis connection failed: {}. Rate limiting will be disabled.",
                        e
                    );
                    None
                }
            }
        }
        Err(e) => {
            tracing::warn!(
                "Failed to create Redis pool: {}. Rate limiting will be disabled.",
                e
            );
            None
        }
    };

    let state = AppState {
        db,
        config: Arc::new(config.clone()),
        redis,
    };

    // Build CORS layer
    let cors = middleware::cors::cors_layer(&config);

    let app = Router::new()
        .route("/api/v1/health", get(health_check))
        .merge(routes::api_router(&state))
        .layer(cors)
        .with_state(state);

    let addr = format!("0.0.0.0:{}", config.server_port);
    tracing::info!("RustGym backend starting on {}", addr);

    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");
    axum::serve(listener, app).await.expect("Server error");
}

/// Create a Redis connection pool from the given URL.
fn create_redis_pool(
    redis_url: &str,
) -> Result<deadpool_redis::Pool, deadpool_redis::CreatePoolError> {
    let cfg = deadpool_redis::Config::from_url(redis_url);
    cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1))
}
