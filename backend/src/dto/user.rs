use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dto::auth::UserResponse;
use crate::dto::gamification::AchievementResponse;
use crate::dto::quest::QuestSummary;

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

// ─── Dashboard Types ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct RecentSubmissionResponse {
    pub id: Uuid,
    pub task_id: Uuid,
    pub task_slug: String,
    pub task_title: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct DashboardResponse {
    pub user: UserResponse,
    pub recent_submissions: Vec<RecentSubmissionResponse>,
    pub recent_achievements: Vec<AchievementResponse>,
    pub active_quest: Option<QuestSummary>,
    pub leaderboard_rank: Option<i64>,
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
