use axum::{
    extract::{Path, State},
    middleware,
    routing::get,
    Json, Router,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use uuid::Uuid;

use crate::{
    dto::{
        auth::UserResponse,
        gamification::AchievementResponse,
        quest::QuestSummary,
        user::{
            DashboardResponse, PublicProfileResponse, RecentSubmissionResponse,
            UpdateProfileRequest,
        },
    },
    error::AppError,
    middleware::auth::{auth_middleware, CurrentUser},
    AppState,
};

use entity::{
    achievements, levels, quests, submissions, tasks, user_achievements, user_quest_progress, users,
};

/// Build the users router.
///
/// - `/me` routes require auth middleware
/// - `/:username` is public
pub fn router(state: AppState) -> Router<AppState> {
    let me_routes = Router::new()
        .route("/me", get(get_my_profile).put(update_my_profile))
        .route("/me/dashboard", get(get_my_dashboard))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware));

    Router::new()
        .merge(me_routes)
        .route("/:username", get(get_public_profile))
}

// ─── GET /api/v1/users/me ────────────────────────────────────────────────────

async fn get_my_profile(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> Result<Json<UserResponse>, AppError> {
    let user_id = Uuid::parse_str(current_user.user_id())
        .map_err(|_| AppError::Internal("Invalid user ID in token".to_string()))?;

    let user = users::Entity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(UserResponse::from(&user)))
}

// ─── PUT /api/v1/users/me ────────────────────────────────────────────────────

async fn update_my_profile(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(body): Json<UpdateProfileRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user_id = Uuid::parse_str(current_user.user_id())
        .map_err(|_| AppError::Internal("Invalid user ID in token".to_string()))?;

    // Validate username if provided
    if let Some(ref username) = body.username {
        if username.len() < 3 || username.len() > 32 {
            return Err(AppError::Validation(
                "Username must be between 3 and 32 characters".to_string(),
            ));
        }

        // Check uniqueness (exclude current user)
        let existing = users::Entity::find()
            .filter(users::Column::Username.eq(username.as_str()))
            .filter(users::Column::Id.ne(user_id))
            .one(&state.db)
            .await?;

        if existing.is_some() {
            return Err(AppError::Conflict("Username is already taken".to_string()));
        }
    }

    // Find the current user
    let user = users::Entity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    // Build active model and update only provided fields
    let mut active_user: users::ActiveModel = user.into();

    if let Some(username) = body.username {
        active_user.username = Set(username);
    }

    if let Some(bio) = body.bio {
        active_user.bio = Set(Some(bio));
    }

    if let Some(avatar_url) = body.avatar_url {
        active_user.avatar_url = Set(Some(avatar_url));
    }

    active_user.updated_at = Set(Utc::now().fixed_offset());

    let updated_user = active_user.update(&state.db).await?;

    Ok(Json(UserResponse::from(&updated_user)))
}

// ─── GET /api/v1/users/:username ─────────────────────────────────────────────

async fn get_public_profile(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<PublicProfileResponse>, AppError> {
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(&username))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(PublicProfileResponse::from(&user)))
}

// ─── GET /api/v1/users/me/dashboard ──────────────────────────────────────────

async fn get_my_dashboard(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> Result<Json<DashboardResponse>, AppError> {
    let user_id = Uuid::parse_str(current_user.user_id())
        .map_err(|_| AppError::Internal("Invalid user ID in token".to_string()))?;

    // Fetch user
    let user = users::Entity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    // Fetch recent submissions (last 5) with task info
    let recent_subs = submissions::Entity::find()
        .filter(submissions::Column::UserId.eq(user_id))
        .order_by_desc(submissions::Column::CreatedAt)
        .limit(5)
        .all(&state.db)
        .await?;

    let mut recent_submissions = Vec::with_capacity(recent_subs.len());
    for sub in &recent_subs {
        let task = tasks::Entity::find_by_id(sub.task_id)
            .one(&state.db)
            .await?;
        let (task_slug, task_title) = match task {
            Some(t) => (t.slug, t.title),
            None => ("unknown".to_string(), "Unknown Task".to_string()),
        };
        recent_submissions.push(RecentSubmissionResponse {
            id: sub.id,
            task_id: sub.task_id,
            task_slug,
            task_title,
            status: format!("{:?}", sub.status).to_lowercase(),
            created_at: sub.created_at.to_rfc3339(),
        });
    }

    // Fetch recent achievements (last 5)
    let recent_earned = user_achievements::Entity::find()
        .filter(user_achievements::Column::UserId.eq(user_id))
        .order_by_desc(user_achievements::Column::EarnedAt)
        .limit(5)
        .all(&state.db)
        .await?;

    let mut recent_achievements = Vec::with_capacity(recent_earned.len());
    for ua in &recent_earned {
        if let Some(achievement) = achievements::Entity::find_by_id(ua.achievement_id)
            .one(&state.db)
            .await?
        {
            recent_achievements.push(AchievementResponse {
                id: achievement.id,
                slug: achievement.slug,
                name: achievement.name,
                description: achievement.description,
                icon: achievement.icon,
                xp_reward: achievement.xp_reward,
                is_earned: true,
                earned_at: Some(ua.earned_at.to_rfc3339()),
            });
        }
    }

    // Fetch active quest (first incomplete quest progress)
    let active_quest_progress = user_quest_progress::Entity::find()
        .filter(user_quest_progress::Column::UserId.eq(user_id))
        .filter(user_quest_progress::Column::IsCompleted.eq(false))
        .one(&state.db)
        .await?;

    let active_quest = if let Some(progress) = active_quest_progress {
        if let Some(quest) = quests::Entity::find_by_id(progress.quest_id)
            .one(&state.db)
            .await?
        {
            // Count levels and tasks for the quest
            let quest_levels = levels::Entity::find()
                .filter(levels::Column::QuestId.eq(quest.id))
                .filter(levels::Column::IsPublished.eq(true))
                .all(&state.db)
                .await?;

            let level_count = quest_levels.len() as i64;
            let mut task_count: i64 = 0;
            for level in &quest_levels {
                let count = tasks::Entity::find()
                    .filter(tasks::Column::LevelId.eq(level.id))
                    .filter(tasks::Column::IsPublished.eq(true))
                    .all(&state.db)
                    .await?
                    .len() as i64;
                task_count += count;
            }

            Some(QuestSummary {
                id: quest.id,
                slug: quest.slug,
                title: quest.title,
                description: quest.description,
                icon: quest.icon,
                color: quest.color,
                order_index: quest.order_index,
                is_published: quest.is_published,
                prerequisite_quest_id: quest.prerequisite_quest_id,
                level_count,
                task_count,
                user_progress: Some(crate::dto::quest::QuestProgress {
                    tasks_completed: progress.tasks_completed,
                    tasks_total: progress.tasks_total,
                    is_completed: progress.is_completed,
                }),
            })
        } else {
            None
        }
    } else {
        None
    };

    // Compute leaderboard rank
    let users_above = users::Entity::find()
        .filter(users::Column::Xp.gt(user.xp))
        .all(&state.db)
        .await?
        .len() as i64;
    let leaderboard_rank = Some(users_above + 1);

    Ok(Json(DashboardResponse {
        user: UserResponse::from(&user),
        recent_submissions,
        recent_achievements,
        active_quest,
        leaderboard_rank,
    }))
}
