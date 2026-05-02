use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── Request Types ───────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub username: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}

// ─── Response Types ──────────────────────────────────────────────────────────

/// Public profile response — excludes email and password_hash.
#[derive(Debug, Serialize)]
pub struct PublicProfileResponse {
    pub id: Uuid,
    pub username: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub role: String,
    pub xp: i32,
    pub level: i32,
    pub streak_days: i32,
    pub created_at: String,
}

// ─── Conversions ─────────────────────────────────────────────────────────────

impl From<&entity::users::Model> for PublicProfileResponse {
    fn from(user: &entity::users::Model) -> Self {
        let role = match &user.role {
            entity::users::Role::Student => "student",
            entity::users::Role::Mentor => "mentor",
            entity::users::Role::Admin => "admin",
        };

        Self {
            id: user.id,
            username: user.username.clone(),
            avatar_url: user.avatar_url.clone(),
            bio: user.bio.clone(),
            role: role.to_string(),
            xp: user.xp,
            level: user.level,
            streak_days: user.streak_days,
            created_at: user.created_at.to_rfc3339(),
        }
    }
}
