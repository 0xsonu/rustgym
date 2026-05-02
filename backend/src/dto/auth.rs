use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── Request Types ───────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyEmailRequest {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

// ─── Response Types ──────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub role: String,
    pub xp: i32,
    pub level: i32,
    pub streak_days: i32,
    pub is_verified: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

// ─── Conversions ─────────────────────────────────────────────────────────────

impl From<&entity::users::Model> for UserResponse {
    fn from(user: &entity::users::Model) -> Self {
        let role = match &user.role {
            entity::users::Role::Student => "student",
            entity::users::Role::Mentor => "mentor",
            entity::users::Role::Admin => "admin",
        };

        Self {
            id: user.id,
            username: user.username.clone(),
            email: user.email.clone(),
            avatar_url: user.avatar_url.clone(),
            bio: user.bio.clone(),
            role: role.to_string(),
            xp: user.xp,
            level: user.level,
            streak_days: user.streak_days,
            is_verified: user.is_verified,
            created_at: user.created_at.to_rfc3339(),
        }
    }
}
