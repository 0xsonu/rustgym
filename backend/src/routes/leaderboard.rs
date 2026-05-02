use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::Deserialize;

use crate::{
    dto::gamification::{LeaderboardEntry, LeaderboardResponse},
    error::AppError,
    AppState,
};

use entity::users;

/// Build the leaderboard router.
pub fn router() -> Router<AppState> {
    Router::new().route("/", get(get_leaderboard))
}

#[derive(Debug, Deserialize)]
struct LeaderboardParams {
    period: Option<String>,
}

// ─── GET /api/v1/users/leaderboard ───────────────────────────────────────────

async fn get_leaderboard(
    State(state): State<AppState>,
    Query(params): Query<LeaderboardParams>,
) -> Result<Json<LeaderboardResponse>, AppError> {
    let period = params.period.unwrap_or_else(|| "alltime".to_string());

    // Validate period
    if !["alltime", "weekly", "monthly"].contains(&period.as_str()) {
        return Err(AppError::BadRequest(
            "Invalid period. Must be one of: alltime, weekly, monthly".to_string(),
        ));
    }

    // Determine cache TTL based on period
    let cache_ttl_seconds: u64 = match period.as_str() {
        "alltime" => 300, // 5 minutes
        _ => 120,         // 2 minutes for weekly/monthly
    };

    let cache_key = format!("leaderboard:{}", period);

    // Try to read from Redis cache
    if let Some(ref redis_pool) = state.redis {
        if let Ok(mut conn) = redis_pool.get().await {
            use deadpool_redis::redis::AsyncCommands;
            if let Ok(Some(cached_json)) = conn.get::<_, Option<String>>(&cache_key).await {
                if let Ok(response) = serde_json::from_str::<LeaderboardResponse>(&cached_json) {
                    return Ok(Json(response));
                }
            }
        }
    }

    // Query DB for leaderboard
    let entries = fetch_leaderboard_from_db(&state, &period).await?;

    let response = LeaderboardResponse {
        entries,
        period: period.clone(),
    };

    // Cache in Redis
    if let Some(ref redis_pool) = state.redis {
        if let Ok(mut conn) = redis_pool.get().await {
            use deadpool_redis::redis::AsyncCommands;
            if let Ok(json) = serde_json::to_string(&response) {
                let _: Result<(), _> = conn
                    .set_ex::<_, _, ()>(&cache_key, &json, cache_ttl_seconds)
                    .await;
            }
        }
    }

    Ok(Json(response))
}

async fn fetch_leaderboard_from_db(
    state: &AppState,
    period: &str,
) -> Result<Vec<LeaderboardEntry>, AppError> {
    // For all periods, we rank by XP.
    // For weekly/monthly, we filter by recent activity via last_active_at.
    let users_list: Vec<users::Model> = match period {
        "weekly" => {
            let week_ago = chrono::Utc::now().fixed_offset() - chrono::Duration::days(7);
            users::Entity::find()
                .filter(users::Column::LastActiveAt.gte(week_ago))
                .order_by_desc(users::Column::Xp)
                .limit(100)
                .all(&state.db)
                .await?
        }
        "monthly" => {
            let month_ago = chrono::Utc::now().fixed_offset() - chrono::Duration::days(30);
            users::Entity::find()
                .filter(users::Column::LastActiveAt.gte(month_ago))
                .order_by_desc(users::Column::Xp)
                .limit(100)
                .all(&state.db)
                .await?
        }
        _ => {
            // alltime
            users::Entity::find()
                .order_by_desc(users::Column::Xp)
                .limit(100)
                .all(&state.db)
                .await?
        }
    };

    let entries: Vec<LeaderboardEntry> = users_list
        .into_iter()
        .enumerate()
        .map(|(idx, u)| LeaderboardEntry {
            rank: (idx + 1) as i64,
            username: u.username,
            avatar_url: u.avatar_url,
            level: u.level,
            xp: u.xp,
        })
        .collect();

    Ok(entries)
}
