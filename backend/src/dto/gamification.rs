use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── Achievements ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct AchievementResponse {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub xp_reward: i32,
    pub is_earned: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earned_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AchievementsListResponse {
    pub achievements: Vec<AchievementResponse>,
    pub total_earned: usize,
    pub total_available: usize,
}

// ─── Leaderboard ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub rank: i64,
    pub username: String,
    pub avatar_url: Option<String>,
    pub level: i32,
    pub xp: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardResponse {
    pub entries: Vec<LeaderboardEntry>,
    pub period: String,
}
