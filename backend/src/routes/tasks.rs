use axum::{
    extract::{Path, Query, State},
    middleware,
    routing::{get, post},
    Json, Router,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    dto::quest::{SubmissionListResponse, SubmissionSummary, TaskDetailResponse},
    error::AppError,
    middleware::auth::{auth_middleware, CurrentUser},
    services::{
        achievement_service::{self, AchievementAwarded, SubmissionContext},
        progress_service,
        runner_client::{PlaygroundResponse, RunnerClient},
        streak_service, xp_service,
    },
    AppState,
};

use entity::{submissions, tasks};

/// Build the tasks router.
pub fn router(state: AppState) -> Router<AppState> {
    let auth_routes = Router::new()
        .route("/{slug}/submissions", get(get_task_submissions))
        .route("/{slug}/submit", post(submit_task))
        .route("/{slug}/run", post(run_playground))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware));

    Router::new()
        .route("/{slug}", get(get_task_detail))
        .merge(auth_routes)
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
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(slug): Path<String>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<SubmissionListResponse>, AppError> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);

    let user_id = Uuid::parse_str(current_user.user_id())
        .map_err(|_| AppError::Internal("Invalid user ID in token".to_string()))?;

    // Find the task by slug
    let task = tasks::Entity::find()
        .filter(tasks::Column::Slug.eq(&slug))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Task not found".to_string()))?;

    // Count total submissions for this user+task
    let total = submissions::Entity::find()
        .filter(submissions::Column::UserId.eq(user_id))
        .filter(submissions::Column::TaskId.eq(task.id))
        .count(&state.db)
        .await?;

    // Fetch paginated submissions
    let subs = submissions::Entity::find()
        .filter(submissions::Column::UserId.eq(user_id))
        .filter(submissions::Column::TaskId.eq(task.id))
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

// ─── POST /api/v1/tasks/:slug/submit ─────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct SubmitRequest {
    code: String,
}

#[derive(Debug, serde::Serialize)]
struct SubmitResponse {
    id: Uuid,
    status: String,
    test_results: serde_json::Value,
    stdout: String,
    stderr: String,
    duration_ms: i32,
    memory_kb: i32,
    xp_awarded: i32,
    attempt_number: i32,
    leveled_up: bool,
    new_level: Option<i32>,
    new_xp: Option<i32>,
    new_streak: Option<i32>,
    achievements_awarded: Vec<AchievementAwarded>,
    created_at: String,
}

async fn submit_task(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(slug): Path<String>,
    Json(body): Json<SubmitRequest>,
) -> Result<Json<SubmitResponse>, AppError> {
    let user_id = Uuid::parse_str(current_user.user_id())
        .map_err(|_| AppError::Internal("Invalid user ID in token".to_string()))?;

    // Find the task by slug
    let task = tasks::Entity::find()
        .filter(tasks::Column::Slug.eq(&slug))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Task not found".to_string()))?;

    // Call runner service
    let runner = RunnerClient::new(&state.config.runner_url);
    let syntest_rules = if task.syntest_rules.is_empty() {
        None
    } else {
        Some(task.syntest_rules.clone())
    };

    let run_result = runner
        .run_test(&body.code, &task.test_code, &task.cargo_toml, syntest_rules)
        .await?;

    // Calculate attempt number
    let attempt_count = submissions::Entity::find()
        .filter(submissions::Column::UserId.eq(user_id))
        .filter(submissions::Column::TaskId.eq(task.id))
        .count(&state.db)
        .await?;
    let attempt_number = (attempt_count + 1) as i32;

    // Map runner status to submission status
    let submission_status = match run_result.status.as_str() {
        "passed" => submissions::SubmissionStatus::Passed,
        "failed" => submissions::SubmissionStatus::Failed,
        "timeout" => submissions::SubmissionStatus::Timeout,
        _ => submissions::SubmissionStatus::Error,
    };

    // Calculate XP awarded (only on first pass)
    let xp_awarded = if matches!(submission_status, submissions::SubmissionStatus::Passed) {
        // Check if user has already passed this task
        let has_passed = submissions::Entity::find()
            .filter(submissions::Column::UserId.eq(user_id))
            .filter(submissions::Column::TaskId.eq(task.id))
            .filter(submissions::Column::Status.eq(submissions::SubmissionStatus::Passed))
            .count(&state.db)
            .await?;

        if has_passed == 0 {
            task.xp_reward
        } else {
            0
        }
    } else {
        0
    };

    let now = chrono::Utc::now().fixed_offset();
    let submission_id = Uuid::new_v4();

    let test_results_json = serde_json::to_value(&run_result.tests)
        .unwrap_or_else(|_| serde_json::Value::Array(vec![]));

    // Record submission in DB
    let new_submission = submissions::ActiveModel {
        id: Set(submission_id),
        user_id: Set(user_id),
        task_id: Set(task.id),
        code: Set(body.code),
        status: Set(submission_status.clone()),
        test_results: Set(test_results_json.clone()),
        stdout: Set(run_result.stdout.clone()),
        stderr: Set(run_result.stderr.clone()),
        duration_ms: Set(run_result.duration_ms as i32),
        memory_kb: Set(run_result.memory_kb as i32),
        xp_awarded: Set(xp_awarded),
        attempt_number: Set(attempt_number),
        created_at: Set(now),
    };

    new_submission.insert(&state.db).await?;

    // Award XP and update progress if this is the first pass
    let mut leveled_up = false;
    let mut new_level = None;
    let mut new_xp = None;

    if xp_awarded > 0 {
        let xp_result = xp_service::award_xp(&state.db, user_id, xp_awarded).await?;
        leveled_up = xp_result.leveled_up;
        new_level = Some(xp_result.new_level);
        new_xp = Some(xp_result.new_xp);

        progress_service::update_progress(&state.db, user_id, task.id).await?;
    }

    // Update streak on any successful submission
    let new_streak = if matches!(submission_status, submissions::SubmissionStatus::Passed) {
        Some(streak_service::update_streak(&state.db, user_id).await?)
    } else {
        None
    };

    // Evaluate achievements
    let submission_ctx = SubmissionContext {
        task_id: task.id,
        status: submission_status.clone(),
        attempt_number,
        duration_ms: run_result.duration_ms as i32,
    };
    let achievements_awarded =
        achievement_service::evaluate_achievements(&state.db, user_id, &submission_ctx).await?;

    Ok(Json(SubmitResponse {
        id: submission_id,
        status: format!("{:?}", submission_status).to_lowercase(),
        test_results: test_results_json,
        stdout: run_result.stdout,
        stderr: run_result.stderr,
        duration_ms: run_result.duration_ms as i32,
        memory_kb: run_result.memory_kb as i32,
        xp_awarded,
        attempt_number,
        leveled_up,
        new_level,
        new_xp,
        new_streak,
        achievements_awarded,
        created_at: now.to_rfc3339(),
    }))
}

// ─── POST /api/v1/tasks/:slug/run ────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct RunRequest {
    code: String,
}

#[derive(Debug, serde::Serialize)]
struct RunResponse {
    status: String,
    stdout: String,
    stderr: String,
    duration_ms: i64,
}

async fn run_playground(
    State(state): State<AppState>,
    _current_user: CurrentUser,
    Path(_slug): Path<String>,
    Json(body): Json<RunRequest>,
) -> Result<Json<RunResponse>, AppError> {
    let runner = RunnerClient::new(&state.config.runner_url);

    let result: PlaygroundResponse = runner.run_playground(&body.code).await?;

    Ok(Json(RunResponse {
        status: result.status,
        stdout: result.stdout,
        stderr: result.stderr,
        duration_ms: result.duration_ms,
    }))
}
