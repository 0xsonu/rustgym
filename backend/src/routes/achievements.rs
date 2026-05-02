use axum::{extract::State, middleware, routing::get, Json, Router};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{
    dto::gamification::{AchievementResponse, AchievementsListResponse},
    error::AppError,
    middleware::auth::{auth_middleware, CurrentUser},
    AppState,
};

use entity::{achievements, user_achievements};

/// Build the achievements router.
pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/me/achievements", get(get_my_achievements))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}

// ─── GET /api/v1/users/me/achievements ───────────────────────────────────────

async fn get_my_achievements(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> Result<Json<AchievementsListResponse>, AppError> {
    let user_id = Uuid::parse_str(current_user.user_id())
        .map_err(|_| AppError::Internal("Invalid user ID in token".to_string()))?;

    // Load all achievements
    let all_achievements = achievements::Entity::find().all(&state.db).await?;

    // Load user's earned achievements
    let earned_list = user_achievements::Entity::find()
        .filter(user_achievements::Column::UserId.eq(user_id))
        .all(&state.db)
        .await?;

    let earned_map: std::collections::HashMap<Uuid, chrono::DateTime<chrono::FixedOffset>> =
        earned_list
            .into_iter()
            .map(|ua| (ua.achievement_id, ua.earned_at))
            .collect();

    let total_available = all_achievements.len();
    let total_earned = earned_map.len();

    let achievements_response: Vec<AchievementResponse> = all_achievements
        .into_iter()
        .map(|a| {
            let earned_at = earned_map.get(&a.id);
            AchievementResponse {
                id: a.id,
                slug: a.slug,
                name: a.name,
                description: a.description,
                icon: a.icon,
                xp_reward: a.xp_reward,
                is_earned: earned_at.is_some(),
                earned_at: earned_at.map(|dt| dt.to_rfc3339()),
            }
        })
        .collect();

    Ok(Json(AchievementsListResponse {
        achievements: achievements_response,
        total_earned,
        total_available,
    }))
}
