use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    middleware,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use uuid::Uuid;

use crate::{
    dto::admin::{
        AchievementResponse, AdminStatsResponse, AdminSubmissionListResponse,
        AdminSubmissionResponse, AdminUserListResponse, AdminUserResponse,
        CreateAchievementRequest, CreateLevelRequest, CreateQuestRequest, CreateTaskRequest,
        LevelResponse, PopularTask, QuestResponse, SubmissionSearchParams, TaskResponse,
        UpdateAchievementRequest, UpdateLevelRequest, UpdateQuestRequest, UpdateTaskRequest,
        UpdateUserRoleRequest, UserSearchParams,
    },
    error::AppError,
    middleware::auth::{auth_middleware, CurrentUser},
    AppState,
};

use entity::{achievements, levels, quests, refresh_tokens, submissions, tasks, users};

/// Build the admin router. All routes require admin role.
pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/stats", get(get_stats))
        // Quest CRUD
        .route("/quests", get(list_quests))
        .route("/quests", post(create_quest))
        .route("/quests/:id", get(get_quest))
        .route("/quests/:id", put(update_quest))
        .route("/quests/:id", delete(delete_quest))
        // Level CRUD
        .route("/levels", get(list_levels))
        .route("/levels", post(create_level))
        .route("/levels/:id", get(get_level))
        .route("/levels/:id", put(update_level))
        .route("/levels/:id", delete(delete_level))
        // Task CRUD
        .route("/tasks", get(list_tasks))
        .route("/tasks", post(create_task))
        .route("/tasks/:id", get(get_task))
        .route("/tasks/:id", put(update_task))
        .route("/tasks/:id", delete(delete_task))
        // Achievement CRUD
        .route("/achievements", get(list_achievements))
        .route("/achievements", post(create_achievement))
        .route("/achievements/:id", get(get_achievement))
        .route("/achievements/:id", put(update_achievement))
        // User management
        .route("/users", get(list_users))
        .route("/users/:id", put(update_user_role))
        .route("/users/:id", delete(ban_user))
        // Submissions
        .route("/submissions", get(list_submissions))
        .route_layer(middleware::from_fn(require_admin))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}

// ─── Admin Role Guard ────────────────────────────────────────────────────────

async fn require_admin(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let current_user = request
        .extensions()
        .get::<CurrentUser>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if current_user.role() != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}

// ─── GET /api/v1/admin/stats ─────────────────────────────────────────────────

async fn get_stats(
    State(state): State<AppState>,
    _user: CurrentUser,
) -> Result<Json<AdminStatsResponse>, AppError> {
    let total_users = users::Entity::find().count(&state.db).await?;

    let today_start = Utc::now()
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .fixed_offset();

    let submissions_today = submissions::Entity::find()
        .filter(submissions::Column::CreatedAt.gte(today_start))
        .count(&state.db)
        .await?;

    let passed_today = submissions::Entity::find()
        .filter(submissions::Column::CreatedAt.gte(today_start))
        .filter(submissions::Column::Status.eq(submissions::SubmissionStatus::Passed))
        .count(&state.db)
        .await?;

    let pass_rate = if submissions_today > 0 {
        (passed_today as f64 / submissions_today as f64) * 100.0
    } else {
        0.0
    };

    // Popular tasks: top 5 by submission count
    let all_tasks = tasks::Entity::find()
        .order_by_asc(tasks::Column::Title)
        .limit(100)
        .all(&state.db)
        .await?;

    let mut popular_tasks: Vec<PopularTask> = Vec::new();
    for task in all_tasks.into_iter().take(5) {
        let count = submissions::Entity::find()
            .filter(submissions::Column::TaskId.eq(task.id))
            .count(&state.db)
            .await?;
        popular_tasks.push(PopularTask {
            id: task.id,
            slug: task.slug,
            title: task.title,
            submission_count: count,
        });
    }

    popular_tasks.sort_by_key(|t| std::cmp::Reverse(t.submission_count));
    popular_tasks.truncate(5);

    Ok(Json(AdminStatsResponse {
        total_users,
        submissions_today,
        pass_rate,
        popular_tasks,
    }))
}

// ─── Quest CRUD ──────────────────────────────────────────────────────────────

async fn list_quests(
    State(state): State<AppState>,
    _user: CurrentUser,
) -> Result<Json<Vec<QuestResponse>>, AppError> {
    let all = quests::Entity::find()
        .order_by_asc(quests::Column::OrderIndex)
        .all(&state.db)
        .await?;

    let response: Vec<QuestResponse> = all
        .into_iter()
        .map(|q| QuestResponse {
            id: q.id,
            slug: q.slug,
            title: q.title,
            description: q.description,
            order_index: q.order_index,
            icon: q.icon,
            color: q.color,
            is_published: q.is_published,
            prerequisite_quest_id: q.prerequisite_quest_id,
            created_at: q.created_at.to_rfc3339(),
            updated_at: q.updated_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(response))
}

async fn create_quest(
    State(state): State<AppState>,
    _user: CurrentUser,
    Json(body): Json<CreateQuestRequest>,
) -> Result<(StatusCode, Json<QuestResponse>), AppError> {
    let now = Utc::now().fixed_offset();
    let id = Uuid::new_v4();

    let model = quests::ActiveModel {
        id: Set(id),
        slug: Set(body.slug),
        title: Set(body.title),
        description: Set(body.description),
        order_index: Set(body.order_index),
        icon: Set(body.icon),
        color: Set(body.color),
        is_published: Set(body.is_published.unwrap_or(false)),
        prerequisite_quest_id: Set(body.prerequisite_quest_id),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let quest = model.insert(&state.db).await?;

    Ok((
        StatusCode::CREATED,
        Json(QuestResponse {
            id: quest.id,
            slug: quest.slug,
            title: quest.title,
            description: quest.description,
            order_index: quest.order_index,
            icon: quest.icon,
            color: quest.color,
            is_published: quest.is_published,
            prerequisite_quest_id: quest.prerequisite_quest_id,
            created_at: quest.created_at.to_rfc3339(),
            updated_at: quest.updated_at.to_rfc3339(),
        }),
    ))
}

async fn get_quest(
    State(state): State<AppState>,
    _user: CurrentUser,
    Path(id): Path<Uuid>,
) -> Result<Json<QuestResponse>, AppError> {
    let quest = quests::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Quest not found".to_string()))?;

    Ok(Json(QuestResponse {
        id: quest.id,
        slug: quest.slug,
        title: quest.title,
        description: quest.description,
        order_index: quest.order_index,
        icon: quest.icon,
        color: quest.color,
        is_published: quest.is_published,
        prerequisite_quest_id: quest.prerequisite_quest_id,
        created_at: quest.created_at.to_rfc3339(),
        updated_at: quest.updated_at.to_rfc3339(),
    }))
}

async fn update_quest(
    State(state): State<AppState>,
    _user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateQuestRequest>,
) -> Result<Json<QuestResponse>, AppError> {
    let existing = quests::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Quest not found".to_string()))?;

    let now = Utc::now().fixed_offset();

    let mut model: quests::ActiveModel = existing.into();
    if let Some(slug) = body.slug {
        model.slug = Set(slug);
    }
    if let Some(title) = body.title {
        model.title = Set(title);
    }
    if body.description.is_some() {
        model.description = Set(body.description);
    }
    if let Some(order_index) = body.order_index {
        model.order_index = Set(order_index);
    }
    if body.icon.is_some() {
        model.icon = Set(body.icon);
    }
    if body.color.is_some() {
        model.color = Set(body.color);
    }
    if let Some(is_published) = body.is_published {
        model.is_published = Set(is_published);
    }
    if body.prerequisite_quest_id.is_some() {
        model.prerequisite_quest_id = Set(body.prerequisite_quest_id);
    }
    model.updated_at = Set(now);

    let quest = model.update(&state.db).await?;

    Ok(Json(QuestResponse {
        id: quest.id,
        slug: quest.slug,
        title: quest.title,
        description: quest.description,
        order_index: quest.order_index,
        icon: quest.icon,
        color: quest.color,
        is_published: quest.is_published,
        prerequisite_quest_id: quest.prerequisite_quest_id,
        created_at: quest.created_at.to_rfc3339(),
        updated_at: quest.updated_at.to_rfc3339(),
    }))
}

async fn delete_quest(
    State(state): State<AppState>,
    _user: CurrentUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = quests::Entity::delete_by_id(id).exec(&state.db).await?;
    if result.rows_affected == 0 {
        return Err(AppError::NotFound("Quest not found".to_string()));
    }
    Ok(StatusCode::NO_CONTENT)
}

// ─── Level CRUD ──────────────────────────────────────────────────────────────

async fn list_levels(
    State(state): State<AppState>,
    _user: CurrentUser,
) -> Result<Json<Vec<LevelResponse>>, AppError> {
    let all = levels::Entity::find()
        .order_by_asc(levels::Column::OrderIndex)
        .all(&state.db)
        .await?;

    let response: Vec<LevelResponse> = all
        .into_iter()
        .map(|l| LevelResponse {
            id: l.id,
            quest_id: l.quest_id,
            slug: l.slug,
            title: l.title,
            description: l.description,
            order_index: l.order_index,
            is_published: l.is_published,
            created_at: l.created_at.to_rfc3339(),
            updated_at: l.updated_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(response))
}

async fn create_level(
    State(state): State<AppState>,
    _user: CurrentUser,
    Json(body): Json<CreateLevelRequest>,
) -> Result<(StatusCode, Json<LevelResponse>), AppError> {
    let now = Utc::now().fixed_offset();
    let id = Uuid::new_v4();

    let model = levels::ActiveModel {
        id: Set(id),
        quest_id: Set(body.quest_id),
        slug: Set(body.slug),
        title: Set(body.title),
        description: Set(body.description),
        order_index: Set(body.order_index),
        is_published: Set(body.is_published.unwrap_or(false)),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let level = model.insert(&state.db).await?;

    Ok((
        StatusCode::CREATED,
        Json(LevelResponse {
            id: level.id,
            quest_id: level.quest_id,
            slug: level.slug,
            title: level.title,
            description: level.description,
            order_index: level.order_index,
            is_published: level.is_published,
            created_at: level.created_at.to_rfc3339(),
            updated_at: level.updated_at.to_rfc3339(),
        }),
    ))
}

async fn get_level(
    State(state): State<AppState>,
    _user: CurrentUser,
    Path(id): Path<Uuid>,
) -> Result<Json<LevelResponse>, AppError> {
    let level = levels::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Level not found".to_string()))?;

    Ok(Json(LevelResponse {
        id: level.id,
        quest_id: level.quest_id,
        slug: level.slug,
        title: level.title,
        description: level.description,
        order_index: level.order_index,
        is_published: level.is_published,
        created_at: level.created_at.to_rfc3339(),
        updated_at: level.updated_at.to_rfc3339(),
    }))
}

async fn update_level(
    State(state): State<AppState>,
    _user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateLevelRequest>,
) -> Result<Json<LevelResponse>, AppError> {
    let existing = levels::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Level not found".to_string()))?;

    let now = Utc::now().fixed_offset();

    let mut model: levels::ActiveModel = existing.into();
    if let Some(quest_id) = body.quest_id {
        model.quest_id = Set(quest_id);
    }
    if let Some(slug) = body.slug {
        model.slug = Set(slug);
    }
    if let Some(title) = body.title {
        model.title = Set(title);
    }
    if body.description.is_some() {
        model.description = Set(body.description);
    }
    if let Some(order_index) = body.order_index {
        model.order_index = Set(order_index);
    }
    if let Some(is_published) = body.is_published {
        model.is_published = Set(is_published);
    }
    model.updated_at = Set(now);

    let level = model.update(&state.db).await?;

    Ok(Json(LevelResponse {
        id: level.id,
        quest_id: level.quest_id,
        slug: level.slug,
        title: level.title,
        description: level.description,
        order_index: level.order_index,
        is_published: level.is_published,
        created_at: level.created_at.to_rfc3339(),
        updated_at: level.updated_at.to_rfc3339(),
    }))
}

async fn delete_level(
    State(state): State<AppState>,
    _user: CurrentUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = levels::Entity::delete_by_id(id).exec(&state.db).await?;
    if result.rows_affected == 0 {
        return Err(AppError::NotFound("Level not found".to_string()));
    }
    Ok(StatusCode::NO_CONTENT)
}

// ─── Task CRUD ───────────────────────────────────────────────────────────────

async fn list_tasks(
    State(state): State<AppState>,
    _user: CurrentUser,
) -> Result<Json<Vec<TaskResponse>>, AppError> {
    let all = tasks::Entity::find()
        .order_by_asc(tasks::Column::OrderIndex)
        .all(&state.db)
        .await?;

    let response: Vec<TaskResponse> = all
        .into_iter()
        .map(|t| TaskResponse {
            id: t.id,
            level_id: t.level_id,
            slug: t.slug,
            title: t.title,
            description_md: t.description_md,
            starter_code: t.starter_code,
            solution_code: t.solution_code,
            test_code: t.test_code,
            cargo_toml: t.cargo_toml,
            difficulty: format!("{:?}", t.difficulty).to_lowercase(),
            xp_reward: t.xp_reward,
            order_index: t.order_index,
            is_published: t.is_published,
            tags: t.tags,
            hint_md: t.hint_md,
            syntest_rules: t.syntest_rules,
            created_at: t.created_at.to_rfc3339(),
            updated_at: t.updated_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(response))
}

fn parse_difficulty(s: &str) -> Result<tasks::Difficulty, AppError> {
    match s.to_lowercase().as_str() {
        "beginner" => Ok(tasks::Difficulty::Beginner),
        "easy" => Ok(tasks::Difficulty::Easy),
        "medium" => Ok(tasks::Difficulty::Medium),
        "hard" => Ok(tasks::Difficulty::Hard),
        "advanced" => Ok(tasks::Difficulty::Advanced),
        _ => Err(AppError::BadRequest(format!("Invalid difficulty: {}", s))),
    }
}

async fn create_task(
    State(state): State<AppState>,
    _user: CurrentUser,
    Json(body): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<TaskResponse>), AppError> {
    let now = Utc::now().fixed_offset();
    let id = Uuid::new_v4();
    let difficulty = parse_difficulty(&body.difficulty)?;

    let model = tasks::ActiveModel {
        id: Set(id),
        level_id: Set(body.level_id),
        slug: Set(body.slug),
        title: Set(body.title),
        description_md: Set(body.description_md),
        starter_code: Set(body.starter_code),
        solution_code: Set(body.solution_code),
        test_code: Set(body.test_code),
        cargo_toml: Set(body.cargo_toml),
        difficulty: Set(difficulty),
        xp_reward: Set(body.xp_reward),
        order_index: Set(body.order_index),
        is_published: Set(body.is_published.unwrap_or(false)),
        tags: Set(body.tags.unwrap_or_default()),
        hint_md: Set(body.hint_md),
        syntest_rules: Set(body.syntest_rules.unwrap_or_default()),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let task = model.insert(&state.db).await?;

    Ok((
        StatusCode::CREATED,
        Json(TaskResponse {
            id: task.id,
            level_id: task.level_id,
            slug: task.slug,
            title: task.title,
            description_md: task.description_md,
            starter_code: task.starter_code,
            solution_code: task.solution_code,
            test_code: task.test_code,
            cargo_toml: task.cargo_toml,
            difficulty: format!("{:?}", task.difficulty).to_lowercase(),
            xp_reward: task.xp_reward,
            order_index: task.order_index,
            is_published: task.is_published,
            tags: task.tags,
            hint_md: task.hint_md,
            syntest_rules: task.syntest_rules,
            created_at: task.created_at.to_rfc3339(),
            updated_at: task.updated_at.to_rfc3339(),
        }),
    ))
}

async fn get_task(
    State(state): State<AppState>,
    _user: CurrentUser,
    Path(id): Path<Uuid>,
) -> Result<Json<TaskResponse>, AppError> {
    let task = tasks::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Task not found".to_string()))?;

    Ok(Json(TaskResponse {
        id: task.id,
        level_id: task.level_id,
        slug: task.slug,
        title: task.title,
        description_md: task.description_md,
        starter_code: task.starter_code,
        solution_code: task.solution_code,
        test_code: task.test_code,
        cargo_toml: task.cargo_toml,
        difficulty: format!("{:?}", task.difficulty).to_lowercase(),
        xp_reward: task.xp_reward,
        order_index: task.order_index,
        is_published: task.is_published,
        tags: task.tags,
        hint_md: task.hint_md,
        syntest_rules: task.syntest_rules,
        created_at: task.created_at.to_rfc3339(),
        updated_at: task.updated_at.to_rfc3339(),
    }))
}

async fn update_task(
    State(state): State<AppState>,
    _user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateTaskRequest>,
) -> Result<Json<TaskResponse>, AppError> {
    let existing = tasks::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Task not found".to_string()))?;

    let now = Utc::now().fixed_offset();

    let mut model: tasks::ActiveModel = existing.into();
    if let Some(level_id) = body.level_id {
        model.level_id = Set(level_id);
    }
    if let Some(slug) = body.slug {
        model.slug = Set(slug);
    }
    if let Some(title) = body.title {
        model.title = Set(title);
    }
    if let Some(description_md) = body.description_md {
        model.description_md = Set(description_md);
    }
    if let Some(starter_code) = body.starter_code {
        model.starter_code = Set(starter_code);
    }
    if let Some(solution_code) = body.solution_code {
        model.solution_code = Set(solution_code);
    }
    if let Some(test_code) = body.test_code {
        model.test_code = Set(test_code);
    }
    if let Some(cargo_toml) = body.cargo_toml {
        model.cargo_toml = Set(cargo_toml);
    }
    if let Some(difficulty) = body.difficulty {
        model.difficulty = Set(parse_difficulty(&difficulty)?);
    }
    if let Some(xp_reward) = body.xp_reward {
        model.xp_reward = Set(xp_reward);
    }
    if let Some(order_index) = body.order_index {
        model.order_index = Set(order_index);
    }
    if let Some(is_published) = body.is_published {
        model.is_published = Set(is_published);
    }
    if let Some(tags) = body.tags {
        model.tags = Set(tags);
    }
    if body.hint_md.is_some() {
        model.hint_md = Set(body.hint_md);
    }
    if let Some(syntest_rules) = body.syntest_rules {
        model.syntest_rules = Set(syntest_rules);
    }
    model.updated_at = Set(now);

    let task = model.update(&state.db).await?;

    Ok(Json(TaskResponse {
        id: task.id,
        level_id: task.level_id,
        slug: task.slug,
        title: task.title,
        description_md: task.description_md,
        starter_code: task.starter_code,
        solution_code: task.solution_code,
        test_code: task.test_code,
        cargo_toml: task.cargo_toml,
        difficulty: format!("{:?}", task.difficulty).to_lowercase(),
        xp_reward: task.xp_reward,
        order_index: task.order_index,
        is_published: task.is_published,
        tags: task.tags,
        hint_md: task.hint_md,
        syntest_rules: task.syntest_rules,
        created_at: task.created_at.to_rfc3339(),
        updated_at: task.updated_at.to_rfc3339(),
    }))
}

async fn delete_task(
    State(state): State<AppState>,
    _user: CurrentUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = tasks::Entity::delete_by_id(id).exec(&state.db).await?;
    if result.rows_affected == 0 {
        return Err(AppError::NotFound("Task not found".to_string()));
    }
    Ok(StatusCode::NO_CONTENT)
}

// ─── Achievement CRUD ────────────────────────────────────────────────────────

fn parse_condition_type(s: &str) -> Result<achievements::ConditionType, AppError> {
    match s {
        "tasks_completed" => Ok(achievements::ConditionType::TasksCompleted),
        "xp_earned" => Ok(achievements::ConditionType::XpEarned),
        "streak" => Ok(achievements::ConditionType::Streak),
        "quest_done" => Ok(achievements::ConditionType::QuestDone),
        "first_submit" => Ok(achievements::ConditionType::FirstSubmit),
        "first_pass" => Ok(achievements::ConditionType::FirstPass),
        "speed_pass" => Ok(achievements::ConditionType::SpeedPass),
        "forum_accepted" => Ok(achievements::ConditionType::ForumAccepted),
        _ => Err(AppError::BadRequest(format!(
            "Invalid condition_type: {}",
            s
        ))),
    }
}

async fn list_achievements(
    State(state): State<AppState>,
    _user: CurrentUser,
) -> Result<Json<Vec<AchievementResponse>>, AppError> {
    let all = achievements::Entity::find()
        .order_by_asc(achievements::Column::Name)
        .all(&state.db)
        .await?;

    let response: Vec<AchievementResponse> = all
        .into_iter()
        .map(|a| AchievementResponse {
            id: a.id,
            slug: a.slug,
            name: a.name,
            description: a.description,
            icon: a.icon,
            xp_reward: a.xp_reward,
            condition_type: format!("{:?}", a.condition_type).to_lowercase(),
            condition_value: a.condition_value,
            condition_meta: a.condition_meta,
            created_at: a.created_at.to_rfc3339(),
            updated_at: a.updated_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(response))
}

async fn create_achievement(
    State(state): State<AppState>,
    _user: CurrentUser,
    Json(body): Json<CreateAchievementRequest>,
) -> Result<(StatusCode, Json<AchievementResponse>), AppError> {
    let now = Utc::now().fixed_offset();
    let id = Uuid::new_v4();
    let condition_type = parse_condition_type(&body.condition_type)?;

    let model = achievements::ActiveModel {
        id: Set(id),
        slug: Set(body.slug),
        name: Set(body.name),
        description: Set(body.description),
        icon: Set(body.icon),
        xp_reward: Set(body.xp_reward.unwrap_or(0)),
        condition_type: Set(condition_type),
        condition_value: Set(body.condition_value),
        condition_meta: Set(body.condition_meta),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let achievement = model.insert(&state.db).await?;

    Ok((
        StatusCode::CREATED,
        Json(AchievementResponse {
            id: achievement.id,
            slug: achievement.slug,
            name: achievement.name,
            description: achievement.description,
            icon: achievement.icon,
            xp_reward: achievement.xp_reward,
            condition_type: format!("{:?}", achievement.condition_type).to_lowercase(),
            condition_value: achievement.condition_value,
            condition_meta: achievement.condition_meta,
            created_at: achievement.created_at.to_rfc3339(),
            updated_at: achievement.updated_at.to_rfc3339(),
        }),
    ))
}

async fn get_achievement(
    State(state): State<AppState>,
    _user: CurrentUser,
    Path(id): Path<Uuid>,
) -> Result<Json<AchievementResponse>, AppError> {
    let achievement = achievements::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Achievement not found".to_string()))?;

    Ok(Json(AchievementResponse {
        id: achievement.id,
        slug: achievement.slug,
        name: achievement.name,
        description: achievement.description,
        icon: achievement.icon,
        xp_reward: achievement.xp_reward,
        condition_type: format!("{:?}", achievement.condition_type).to_lowercase(),
        condition_value: achievement.condition_value,
        condition_meta: achievement.condition_meta,
        created_at: achievement.created_at.to_rfc3339(),
        updated_at: achievement.updated_at.to_rfc3339(),
    }))
}

async fn update_achievement(
    State(state): State<AppState>,
    _user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateAchievementRequest>,
) -> Result<Json<AchievementResponse>, AppError> {
    let existing = achievements::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Achievement not found".to_string()))?;

    let now = Utc::now().fixed_offset();

    let mut model: achievements::ActiveModel = existing.into();
    if let Some(slug) = body.slug {
        model.slug = Set(slug);
    }
    if let Some(name) = body.name {
        model.name = Set(name);
    }
    if body.description.is_some() {
        model.description = Set(body.description);
    }
    if body.icon.is_some() {
        model.icon = Set(body.icon);
    }
    if let Some(xp_reward) = body.xp_reward {
        model.xp_reward = Set(xp_reward);
    }
    if let Some(condition_type) = body.condition_type {
        model.condition_type = Set(parse_condition_type(&condition_type)?);
    }
    if let Some(condition_value) = body.condition_value {
        model.condition_value = Set(condition_value);
    }
    if body.condition_meta.is_some() {
        model.condition_meta = Set(body.condition_meta);
    }
    model.updated_at = Set(now);

    let achievement = model.update(&state.db).await?;

    Ok(Json(AchievementResponse {
        id: achievement.id,
        slug: achievement.slug,
        name: achievement.name,
        description: achievement.description,
        icon: achievement.icon,
        xp_reward: achievement.xp_reward,
        condition_type: format!("{:?}", achievement.condition_type).to_lowercase(),
        condition_value: achievement.condition_value,
        condition_meta: achievement.condition_meta,
        created_at: achievement.created_at.to_rfc3339(),
        updated_at: achievement.updated_at.to_rfc3339(),
    }))
}

// ─── User Management ─────────────────────────────────────────────────────────

async fn list_users(
    State(state): State<AppState>,
    _user: CurrentUser,
    Query(params): Query<UserSearchParams>,
) -> Result<Json<AdminUserListResponse>, AppError> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);

    let mut condition = Condition::all();
    if let Some(search) = &params.search {
        if !search.is_empty() {
            condition = condition.add(
                Condition::any()
                    .add(users::Column::Username.contains(search))
                    .add(users::Column::Email.contains(search)),
            );
        }
    }

    let total = users::Entity::find()
        .filter(condition.clone())
        .count(&state.db)
        .await?;

    let all = users::Entity::find()
        .filter(condition)
        .order_by_desc(users::Column::CreatedAt)
        .offset((page.saturating_sub(1)) * per_page)
        .limit(per_page)
        .all(&state.db)
        .await?;

    let users_list: Vec<AdminUserResponse> = all
        .into_iter()
        .map(|u| AdminUserResponse {
            id: u.id,
            username: u.username,
            email: u.email,
            role: format!("{:?}", u.role).to_lowercase(),
            xp: u.xp,
            level: u.level,
            streak_days: u.streak_days,
            is_verified: u.is_verified,
            is_banned: u.is_banned,
            created_at: u.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(AdminUserListResponse {
        users: users_list,
        page,
        per_page,
        total,
    }))
}

async fn update_user_role(
    State(state): State<AppState>,
    _user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateUserRoleRequest>,
) -> Result<Json<AdminUserResponse>, AppError> {
    let existing = users::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let role = match body.role.as_str() {
        "student" => users::Role::Student,
        "mentor" => users::Role::Mentor,
        "admin" => users::Role::Admin,
        _ => return Err(AppError::BadRequest("Invalid role".to_string())),
    };

    let now = Utc::now().fixed_offset();
    let mut model: users::ActiveModel = existing.into();
    model.role = Set(role);
    model.updated_at = Set(now);

    let user = model.update(&state.db).await?;

    Ok(Json(AdminUserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        role: format!("{:?}", user.role).to_lowercase(),
        xp: user.xp,
        level: user.level,
        streak_days: user.streak_days,
        is_verified: user.is_verified,
        is_banned: user.is_banned,
        created_at: user.created_at.to_rfc3339(),
    }))
}

async fn ban_user(
    State(state): State<AppState>,
    _user: CurrentUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let existing = users::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let now = Utc::now().fixed_offset();

    // Set is_banned = true
    let mut model: users::ActiveModel = existing.into();
    model.is_banned = Set(true);
    model.updated_at = Set(now);
    model.update(&state.db).await?;

    // Revoke all refresh tokens for this user
    refresh_tokens::Entity::delete_many()
        .filter(refresh_tokens::Column::UserId.eq(id))
        .exec(&state.db)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

// ─── Submissions ─────────────────────────────────────────────────────────────

async fn list_submissions(
    State(state): State<AppState>,
    _user: CurrentUser,
    Query(params): Query<SubmissionSearchParams>,
) -> Result<Json<AdminSubmissionListResponse>, AppError> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);

    let total = submissions::Entity::find().count(&state.db).await?;

    let all = submissions::Entity::find()
        .order_by_desc(submissions::Column::CreatedAt)
        .offset((page.saturating_sub(1)) * per_page)
        .limit(per_page)
        .all(&state.db)
        .await?;

    let submissions_list: Vec<AdminSubmissionResponse> = all
        .into_iter()
        .map(|s| AdminSubmissionResponse {
            id: s.id,
            user_id: s.user_id,
            task_id: s.task_id,
            status: format!("{:?}", s.status).to_lowercase(),
            duration_ms: s.duration_ms,
            memory_kb: s.memory_kb,
            xp_awarded: s.xp_awarded,
            attempt_number: s.attempt_number,
            created_at: s.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(AdminSubmissionListResponse {
        submissions: submissions_list,
        page,
        per_page,
        total,
    }))
}
