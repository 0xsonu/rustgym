pub mod auth;
pub mod quests;
pub mod submissions;
pub mod tasks;
pub mod users;

use axum::Router;

use crate::AppState;

/// Compose all API routes under /api/v1.
pub fn api_router(state: &AppState) -> Router<AppState> {
    Router::new()
        .nest("/api/v1/auth", auth::router())
        .nest("/api/v1/users", users::router(state.clone()))
        .nest("/api/v1/quests", quests::router())
        .nest("/api/v1/tasks", tasks::router(state.clone()))
        .nest("/api/v1/submissions", submissions::router(state.clone()))
}
