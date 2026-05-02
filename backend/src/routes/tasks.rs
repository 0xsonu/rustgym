use axum::{
    extract::{Path, Query, State},
    middleware,
    routing::get,
    Json, Router,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{
    dto::quest::{SubmissionListResponse, TaskDetailResponse},
    error::AppError,
    middleware::auth::{auth_middleware, CurrentUser},
    AppState,
};

use entity::tasks;

/// Build the tasks router.
pub fn router(state: AppState) -> Router<AppState> {
    let submissions_route = Router::new()
        .route("/{slug}/submissions", get(get_task_submissions))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware));

    Router::new()
        .route("/{slug}", get(get_task_detail))
        .merge(submissions_route)
}

// ─── GET /api/v1/tasks/:slug ─────────────────────────────────────────────────

async fn get_task_detail(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<TaskDetailResponse>, AppError> {
    let task = tasks::Entity::find()
        .filter(tasks::Column::Slug.eq(&slug))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Task not found".to_string()))?;

    // NEVER expose solution_code or test_code
    Ok(Json(TaskDetailResponse {
        id: task.id,
        slug: task.slug,
        title: task.title,
        description_md: task.description_md,
        starter_code: task.starter_code,
        difficulty: format!("{:?}", task.difficulty).to_lowercase(),
        xp_reward: task.xp_reward,
        tags: task.tags,
        hint_md: task.hint_md,
        order_index: task.order_index,
    }))
}

// ─── GET /api/v1/tasks/:slug/submissions ─────────────────────────────────────

#[derive(Debug, Deserialize)]
struct PaginationParams {
    page: Option<u64>,
    per_page: Option<u64>,
}

async fn get_task_submissions(
    State(_state): State<AppState>,
    _current_user: CurrentUser,
    Path(_slug): Path<String>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<SubmissionListResponse>, AppError> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);

    // Placeholder: submissions table doesn't exist yet, return empty list
    Ok(Json(SubmissionListResponse {
        submissions: vec![],
        page,
        per_page,
        total: 0,
    }))
}
