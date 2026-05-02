use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use crate::error::AppError;
use entity::users;

/// Update the user's streak based on their last_active_at timestamp.
///
/// - If last_active_at is today: no change (already active today)
/// - If last_active_at is yesterday: increment streak_days
/// - If last_active_at is older or None: reset streak_days to 1
///
/// Updates last_active_at to now and returns the new streak_days value.
pub async fn update_streak(db: &DatabaseConnection, user_id: Uuid) -> Result<i32, AppError> {
    let user = users::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let now = Utc::now().fixed_offset();
    let today = now.date_naive();

    let new_streak = match user.last_active_at {
        Some(last_active) => {
            let last_date = last_active.date_naive();

            if last_date == today {
                // Already active today, no change
                return Ok(user.streak_days);
            }

            let yesterday = today.pred_opt().unwrap_or(today);

            if last_date == yesterday {
                // Consecutive day — increment streak
                user.streak_days + 1
            } else {
                // Gap detected — reset to 1
                1
            }
        }
        None => {
            // First activity ever — start streak at 1
            1
        }
    };

    // Update user record
    let mut active_user: users::ActiveModel = user.into();
    active_user.streak_days = Set(new_streak);
    active_user.last_active_at = Set(Some(now));
    active_user.updated_at = Set(now);
    active_user.update(db).await?;

    Ok(new_streak)
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    #[test]
    fn test_date_predecessor() {
        let today = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let yesterday = today.pred_opt().unwrap();
        assert_eq!(yesterday, NaiveDate::from_ymd_opt(2024, 3, 14).unwrap());
    }

    #[test]
    fn test_date_predecessor_month_boundary() {
        let today = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();
        let yesterday = today.pred_opt().unwrap();
        assert_eq!(yesterday, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());
    }
}
