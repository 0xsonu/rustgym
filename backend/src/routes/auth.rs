use axum::{extract::State, routing::post, Json, Router};
use chrono::{Duration, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::{
    dto::auth::{
        AuthResponse, ForgotPasswordRequest, LoginRequest, LogoutRequest, MessageResponse,
        RefreshRequest, RegisterRequest, ResetPasswordRequest, UserResponse, VerifyEmailRequest,
    },
    error::AppError,
    services::{
        auth_service::{
            generate_access_token, generate_refresh_token, hash_password, hash_refresh_token,
            verify_password,
        },
        email_service::{send_password_reset_email, send_verification_email},
    },
    AppState,
};

use entity::{refresh_tokens, users};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
        .route("/verify-email", post(verify_email))
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password", post(reset_password))
}

// ─── POST /api/v1/auth/register ──────────────────────────────────────────────

async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Validate input
    if body.username.len() < 3 || body.username.len() > 32 {
        return Err(AppError::Validation(
            "Username must be between 3 and 32 characters".to_string(),
        ));
    }

    if body.password.len() < 8 {
        return Err(AppError::Validation(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    // Basic email validation
    if !body.email.contains('@') || !body.email.contains('.') {
        return Err(AppError::Validation("Invalid email address".to_string()));
    }

    // Check uniqueness of email
    let existing_email = users::Entity::find()
        .filter(users::Column::Email.eq(&body.email))
        .one(&state.db)
        .await?;

    if existing_email.is_some() {
        return Err(AppError::Conflict(
            "Email is already registered".to_string(),
        ));
    }

    // Check uniqueness of username
    let existing_username = users::Entity::find()
        .filter(users::Column::Username.eq(&body.username))
        .one(&state.db)
        .await?;

    if existing_username.is_some() {
        return Err(AppError::Conflict(
            "Username is already taken".to_string(),
        ));
    }

    // Hash password
    let password_hash = hash_password(&body.password)?;

    // Create user record
    let now = Utc::now().fixed_offset();
    let user_id = Uuid::new_v4();

    let new_user = users::ActiveModel {
        id: Set(user_id),
        username: Set(body.username.clone()),
        email: Set(body.email.clone()),
        password_hash: Set(password_hash),
        avatar_url: Set(None),
        bio: Set(None),
        role: Set(users::Role::Student),
        xp: Set(0),
        level: Set(1),
        streak_days: Set(0),
        last_active_at: Set(None),
        is_verified: Set(false),
        is_banned: Set(false),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let user = new_user.insert(&state.db).await?;

    // Generate tokens
    let role_str = match &user.role {
        users::Role::Student => "student",
        users::Role::Mentor => "mentor",
        users::Role::Admin => "admin",
    };

    let access_token =
        generate_access_token(user.id, &user.username, role_str, &state.config.jwt_secret)?;
    let refresh_token = generate_refresh_token();
    let refresh_token_hash = hash_refresh_token(&refresh_token);

    // Store refresh token hash in DB (30-day expiry)
    let refresh_record = refresh_tokens::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user.id),
        token_hash: Set(refresh_token_hash),
        expires_at: Set((Utc::now() + Duration::days(30)).fixed_offset()),
        revoked_at: Set(None),
        created_at: Set(now),
    };

    refresh_record.insert(&state.db).await?;

    // Send verification email (stub)
    let verification_token = Uuid::new_v4().to_string();
    send_verification_email(&user.email, &verification_token).await;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        user: UserResponse::from(&user),
    }))
}

// ─── POST /api/v1/auth/login ─────────────────────────────────────────────────

async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Find user by email
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(&body.email))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    // Verify password — don't reveal which field is wrong
    let valid = verify_password(&body.password, &user.password_hash)?;
    if !valid {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    // Generate tokens
    let role_str = match &user.role {
        users::Role::Student => "student",
        users::Role::Mentor => "mentor",
        users::Role::Admin => "admin",
    };

    let access_token =
        generate_access_token(user.id, &user.username, role_str, &state.config.jwt_secret)?;
    let refresh_token = generate_refresh_token();
    let refresh_token_hash = hash_refresh_token(&refresh_token);

    // Store refresh token hash in DB
    let now = Utc::now().fixed_offset();
    let refresh_record = refresh_tokens::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user.id),
        token_hash: Set(refresh_token_hash),
        expires_at: Set((Utc::now() + Duration::days(30)).fixed_offset()),
        revoked_at: Set(None),
        created_at: Set(now),
    };

    refresh_record.insert(&state.db).await?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        user: UserResponse::from(&user),
    }))
}

// ─── POST /api/v1/auth/refresh ───────────────────────────────────────────────

async fn refresh(
    State(state): State<AppState>,
    Json(body): Json<RefreshRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let token_hash = hash_refresh_token(&body.refresh_token);

    // Find matching non-revoked record
    let token_record = refresh_tokens::Entity::find()
        .filter(refresh_tokens::Column::TokenHash.eq(&token_hash))
        .filter(refresh_tokens::Column::RevokedAt.is_null())
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid refresh token".to_string()))?;

    // Check expiry
    let now = Utc::now().fixed_offset();
    if token_record.expires_at < now {
        return Err(AppError::Unauthorized(
            "Refresh token has expired".to_string(),
        ));
    }

    // Revoke old refresh token
    let mut active_token: refresh_tokens::ActiveModel = token_record.clone().into();
    active_token.revoked_at = Set(Some(now));
    active_token.update(&state.db).await?;

    // Find the user
    let user = users::Entity::find_by_id(token_record.user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Internal("User not found for token".to_string()))?;

    // Generate new tokens
    let role_str = match &user.role {
        users::Role::Student => "student",
        users::Role::Mentor => "mentor",
        users::Role::Admin => "admin",
    };

    let access_token =
        generate_access_token(user.id, &user.username, role_str, &state.config.jwt_secret)?;
    let new_refresh_token = generate_refresh_token();
    let new_refresh_token_hash = hash_refresh_token(&new_refresh_token);

    // Store new refresh token hash
    let new_refresh_record = refresh_tokens::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user.id),
        token_hash: Set(new_refresh_token_hash),
        expires_at: Set((Utc::now() + Duration::days(30)).fixed_offset()),
        revoked_at: Set(None),
        created_at: Set(now),
    };

    new_refresh_record.insert(&state.db).await?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token: new_refresh_token,
        user: UserResponse::from(&user),
    }))
}

// ─── POST /api/v1/auth/logout ────────────────────────────────────────────────

async fn logout(
    State(state): State<AppState>,
    Json(body): Json<LogoutRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    let token_hash = hash_refresh_token(&body.refresh_token);

    // Find matching record
    let token_record = refresh_tokens::Entity::find()
        .filter(refresh_tokens::Column::TokenHash.eq(&token_hash))
        .filter(refresh_tokens::Column::RevokedAt.is_null())
        .one(&state.db)
        .await?;

    if let Some(record) = token_record {
        // Set revoked_at to now
        let now = Utc::now().fixed_offset();
        let mut active_token: refresh_tokens::ActiveModel = record.into();
        active_token.revoked_at = Set(Some(now));
        active_token.update(&state.db).await?;
    }

    Ok(Json(MessageResponse {
        message: "Logged out successfully".to_string(),
    }))
}

// ─── POST /api/v1/auth/verify-email ──────────────────────────────────────────

async fn verify_email(
    State(state): State<AppState>,
    Json(body): Json<VerifyEmailRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    // Validate token (for now, just check it's not empty — real implementation later)
    if body.token.is_empty() {
        return Err(AppError::BadRequest(
            "Verification token is required".to_string(),
        ));
    }

    // For now, we treat the token as a user_id (stub implementation)
    // In a real implementation, this would look up a verification token table
    let user_id = Uuid::parse_str(&body.token)
        .map_err(|_| AppError::BadRequest("Invalid verification token".to_string()))?;

    let user = users::Entity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::BadRequest("Invalid verification token".to_string()))?;

    // Set user's is_verified to true
    let mut active_user: users::ActiveModel = user.into();
    active_user.is_verified = Set(true);
    active_user.updated_at = Set(Utc::now().fixed_offset());
    active_user.update(&state.db).await?;

    Ok(Json(MessageResponse {
        message: "Email verified successfully".to_string(),
    }))
}

// ─── POST /api/v1/auth/forgot-password ───────────────────────────────────────

async fn forgot_password(
    State(state): State<AppState>,
    Json(body): Json<ForgotPasswordRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    // Find user by email (if not found, still return 200 to prevent enumeration)
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(&body.email))
        .one(&state.db)
        .await?;

    if let Some(user) = user {
        // Generate reset token
        let reset_token = Uuid::new_v4().to_string();

        // Send password reset email (stub)
        send_password_reset_email(&user.email, &reset_token).await;
    }

    // Always return success to prevent email enumeration
    Ok(Json(MessageResponse {
        message: "If an account with that email exists, a password reset link has been sent"
            .to_string(),
    }))
}

// ─── POST /api/v1/auth/reset-password ────────────────────────────────────────

async fn reset_password(
    State(state): State<AppState>,
    Json(body): Json<ResetPasswordRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    // Validate new password
    if body.new_password.len() < 8 {
        return Err(AppError::Validation(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    // Validate token and find user
    // For now, treat the token as a user_id (stub implementation)
    let user_id = Uuid::parse_str(&body.token)
        .map_err(|_| AppError::BadRequest("Invalid reset token".to_string()))?;

    let user = users::Entity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::BadRequest("Invalid reset token".to_string()))?;

    // Update password hash
    let new_password_hash = hash_password(&body.new_password)?;
    let now = Utc::now().fixed_offset();

    let mut active_user: users::ActiveModel = user.clone().into();
    active_user.password_hash = Set(new_password_hash);
    active_user.updated_at = Set(now);
    active_user.update(&state.db).await?;

    // Revoke all refresh tokens for that user
    let active_tokens = refresh_tokens::Entity::find()
        .filter(refresh_tokens::Column::UserId.eq(user.id))
        .filter(refresh_tokens::Column::RevokedAt.is_null())
        .all(&state.db)
        .await?;

    for token in active_tokens {
        let mut active_token: refresh_tokens::ActiveModel = token.into();
        active_token.revoked_at = Set(Some(now));
        active_token.update(&state.db).await?;
    }

    Ok(Json(MessageResponse {
        message: "Password reset successfully".to_string(),
    }))
}
