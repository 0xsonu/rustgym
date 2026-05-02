use axum::{
    extract::{Path, State},
    middleware,
    routing::get,
    Json, Router,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::{
    dto::{
        auth::UserResponse,
        user::{PublicProfileResponse, UpdateProfileRequest},
    },
    error::AppError,
    middleware::auth::{auth_middleware, CurrentUser},
    AppState,
};

use entity::users;

/// Build the users router.
///
/// - `/me` routes require auth middleware
/// - `/:username` is public
pub fn router(state: AppState) -> Router<AppState> {
    let me_routes = Router::new()
        .route("/me", get(get_my_profile).put(update_my_profile))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware));

    Router::new()
        .merge(me_routes)
        .route("/{username}", get(get_public_profile))
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
