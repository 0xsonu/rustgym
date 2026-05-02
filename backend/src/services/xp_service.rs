use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::Serialize;
use uuid::Uuid;

use crate::error::AppError;
use entity::users;

/// Result of awarding XP to a user.
#[derive(Debug, Serialize)]
pub struct XpAwardResult {
    pub new_xp: i32,
    pub new_level: i32,
    pub leveled_up: bool,
}

/// Calculate the level for a given XP amount.
/// Level N requires N × 1000 cumulative XP (max level 100).
/// - Level 1: 0–999 XP
/// - Level 2: 1000–1999 XP
/// - Level N: (N-1)*1000 to N*1000-1 XP
/// - Level 100: 99000+ XP (max)
pub fn calculate_level(xp: i32) -> i32 {
    let level = (xp / 1000) + 1;
    level.min(100)
}

/// Award XP to a user, recalculate their level, and update the user record.
/// This function should be called only after confirming this is the first pass (idempotent).
pub async fn award_xp(
    db: &DatabaseConnection,
    user_id: Uuid,
    xp_amount: i32,
) -> Result<XpAwardResult, AppError> {
    // Fetch the current user
    let user = users::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let old_level = user.level;
    let new_xp = user.xp + xp_amount;
    let new_level = calculate_level(new_xp);
    let leveled_up = new_level > old_level;

    // Update user record
    let now = chrono::Utc::now().fixed_offset();
    let mut active_user: users::ActiveModel = user.into();
    active_user.xp = Set(new_xp);
    active_user.level = Set(new_level);
    active_user.updated_at = Set(now);
    active_user.update(db).await?;

    Ok(XpAwardResult {
        new_xp,
        new_level,
        leveled_up,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_level_zero_xp() {
        assert_eq!(calculate_level(0), 1);
    }

    #[test]
    fn test_calculate_level_boundary() {
        assert_eq!(calculate_level(999), 1);
        assert_eq!(calculate_level(1000), 2);
        assert_eq!(calculate_level(1999), 2);
        assert_eq!(calculate_level(2000), 3);
    }

    #[test]
    fn test_calculate_level_max() {
        assert_eq!(calculate_level(99000), 100);
        assert_eq!(calculate_level(150000), 100);
    }

    #[test]
    fn test_calculate_level_mid_range() {
        assert_eq!(calculate_level(5500), 6);
        assert_eq!(calculate_level(49999), 50);
        assert_eq!(calculate_level(50000), 51);
    }
}
