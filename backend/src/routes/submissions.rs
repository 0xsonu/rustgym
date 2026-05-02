use axum::{
    extract::{Path, Query, State},
    middleware,
    routing::get,
    Json, Router,
};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    dto::quest::{SubmissionListResponse, SubmissionSummary},
    error::AppError,
    middleware::auth::{auth_middleware, CurrentUser},
    AppState,
};

use entity::submissions;

/// Build the submissions router.
pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list_submissions))
        .route("/{id}", get(get_submission))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}

// ─── GET /api/v1/submissions/:id ─────────────────────────────────────────────

#[derive(Debug, serde::Serialize)]
struct SubmissionDetailResponse {
    id: Uuid,
    user_id: Uuid,
    task_id: Uuid,
    code: String,
    status: String,
    test_results: serde_json::Value,
    stdout: String,
    stderr: String,
    duration_ms: i32,
    memory_kb: i32,
    xp_awarded: i32,
    attempt_number: i32,
    created_at: String,
}

async fn get_submission(
    State(state): State<AppState>,
    _current_user: CurrentUser,
    Path(id): Path<Uuid>,
) -> Result<Json<SubmissionDetailResponse>, AppError> {
    let submission = submissions::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Submission not found".to_string()))?;

    Ok(Json(SubmissionDetailResponse {
        id: submission.id,
        user_id: submission.user_id,
        task_id: submission.task_id,
        code: submission.code,
        status: format!("{:?}", submission.status).to_lowercase(),
        test_results: submission.test_results,
        stdout: submission.stdout,
        stderr: submission.stderr,
        duration_ms: submission.duration_ms,
        memory_kb: submission.memory_kb,
        xp_awarded: submission.xp_awarded,
        attempt_number: submission.attempt_number,
        created_at: submission.created_at.to_rfc3339(),
    }))
}

// ─── GET /api/v1/submissions ─────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct PaginationParams {
    page: Option<u64>,
    per_page: Option<u64>,
}

async fn list_submissions(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(params): Query<PaginationParams>,
) -> Result<Json<SubmissionListResponse>, AppError> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);

    let user_id = Uuid::parse_str(current_user.user_id())
        .map_err(|_| AppError::Internal("Invalid user ID in token".to_string()))?;

    // Count total submissions for this user
    let total = submissions::Entity::find()
        .filter(submissions::Column::UserId.eq(user_id))
        .count(&state.db)
        .await?;

    // Fetch paginated submissions ordered by created_at DESC
    let subs = submissions::Entity::find()
        .filter(submissions::Column::UserId.eq(user_id))
        .order_by_desc(submissions::Column::CreatedAt)
        .offset((page.saturating_sub(1)) * per_page)
        .limit(per_page)
        .all(&state.db)
        .await?;

    let submissions_list: Vec<SubmissionSummary> = subs
        .into_iter()
        .map(|s| SubmissionSummary {
            id: s.id,
            code: s.code,
            status: format!("{:?}", s.status).to_lowercase(),
            test_results: s.test_results,
            stdout: s.stdout,
            stderr: s.stderr,
            duration_ms: s.duration_ms,
            memory_kb: s.memory_kb,
            xp_awarded: s.xp_awarded,
            attempt_number: s.attempt_number,
            created_at: s.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(SubmissionListResponse {
        submissions: submissions_list,
        page,
        per_page,
        total,
    }))
}
