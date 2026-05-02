use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};
use serde::Serialize;
use uuid::Uuid;

use chrono::Timelike;

use crate::error::AppError;
use entity::{
    achievements::{self, ConditionType},
    submissions, user_achievements, user_quest_progress, user_task_progress, users,
};

/// Context about the current submission, used to evaluate achievements.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SubmissionContext {
    pub task_id: Uuid,
    pub status: submissions::SubmissionStatus,
    pub attempt_number: i32,
    pub duration_ms: i32,
}

/// Represents a newly awarded achievement.
#[derive(Debug, Serialize, Clone)]
pub struct AchievementAwarded {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub xp_reward: i32,
}

/// Evaluate all achievements for a user after a submission.
///
/// Checks each achievement's condition against the user's current state.
/// Awards any newly met achievements (inserts into user_achievements).
/// Returns a list of newly awarded achievements for frontend notification.
pub async fn evaluate_achievements(
    db: &DatabaseConnection,
    user_id: Uuid,
    submission: &SubmissionContext,
) -> Result<Vec<AchievementAwarded>, AppError> {
    // Load all achievements
    let all_achievements = achievements::Entity::find().all(db).await?;

    // Load user's already-earned achievement IDs
    let earned: Vec<Uuid> = user_achievements::Entity::find()
        .filter(user_achievements::Column::UserId.eq(user_id))
        .all(db)
        .await?
        .into_iter()
        .map(|ua| ua.achievement_id)
        .collect();

    // Load user for XP/streak checks
    let user = users::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let mut newly_awarded = Vec::new();

    for achievement in &all_achievements {
        // Skip already earned
        if earned.contains(&achievement.id) {
            continue;
        }

        let condition_met = check_condition(db, user_id, &user, achievement, submission).await?;

        if condition_met {
            // Award the achievement
            let now = chrono::Utc::now().fixed_offset();
            let ua = user_achievements::ActiveModel {
                id: Set(Uuid::new_v4()),
                user_id: Set(user_id),
                achievement_id: Set(achievement.id),
                earned_at: Set(now),
            };
            ua.insert(db).await?;

            newly_awarded.push(AchievementAwarded {
                id: achievement.id,
                slug: achievement.slug.clone(),
                name: achievement.name.clone(),
                description: achievement.description.clone(),
                icon: achievement.icon.clone(),
                xp_reward: achievement.xp_reward,
            });
        }
    }

    Ok(newly_awarded)
}

/// Check if a specific achievement's condition is met.
async fn check_condition(
    db: &DatabaseConnection,
    user_id: Uuid,
    user: &users::Model,
    achievement: &achievements::Model,
    submission: &SubmissionContext,
) -> Result<bool, AppError> {
    match achievement.condition_type {
        ConditionType::TasksCompleted => {
            let completed_count = user_task_progress::Entity::find()
                .filter(user_task_progress::Column::UserId.eq(user_id))
                .filter(
                    user_task_progress::Column::Status
                        .eq(user_task_progress::ProgressStatus::Completed),
                )
                .count(db)
                .await? as i32;

            Ok(completed_count >= achievement.condition_value)
        }

        ConditionType::XpEarned => Ok(user.xp >= achievement.condition_value),

        ConditionType::Streak => Ok(user.streak_days >= achievement.condition_value),

        ConditionType::QuestDone => {
            // Check condition_meta for specific quest_slug
            if let Some(ref meta) = achievement.condition_meta {
                if let Some(quest_slug) = meta.get("quest_slug").and_then(|v| v.as_str()) {
                    // Check if the specific quest is completed
                    let quest = entity::quests::Entity::find()
                        .filter(entity::quests::Column::Slug.eq(quest_slug))
                        .one(db)
                        .await?;

                    if let Some(quest) = quest {
                        let progress = user_quest_progress::Entity::find()
                            .filter(user_quest_progress::Column::UserId.eq(user_id))
                            .filter(user_quest_progress::Column::QuestId.eq(quest.id))
                            .filter(user_quest_progress::Column::IsCompleted.eq(true))
                            .one(db)
                            .await?;

                        return Ok(progress.is_some());
                    }
                    return Ok(false);
                }
            }

            // No specific quest — check total completed quests
            let completed_quests = user_quest_progress::Entity::find()
                .filter(user_quest_progress::Column::UserId.eq(user_id))
                .filter(user_quest_progress::Column::IsCompleted.eq(true))
                .count(db)
                .await? as i32;

            Ok(completed_quests >= achievement.condition_value)
        }

        ConditionType::FirstSubmit => {
            // Check if this is the user's first submission ever
            let submission_count = submissions::Entity::find()
                .filter(submissions::Column::UserId.eq(user_id))
                .count(db)
                .await?;

            // Check condition_meta for special conditions (e.g., before_hour)
            if let Some(ref meta) = achievement.condition_meta {
                if let Some(before_hour) = meta.get("before_hour").and_then(|v| v.as_i64()) {
                    let now = chrono::Utc::now();
                    let hour = now.hour() as i64;
                    return Ok(hour < before_hour);
                }
            }

            Ok(submission_count >= achievement.condition_value as u64)
        }

        ConditionType::FirstPass => {
            // Check condition_meta for first_attempt requirement
            if let Some(ref meta) = achievement.condition_meta {
                if meta.get("first_attempt").and_then(|v| v.as_bool()) == Some(true) {
                    // Must pass on first attempt
                    return Ok(
                        matches!(submission.status, submissions::SubmissionStatus::Passed)
                            && submission.attempt_number == 1,
                    );
                }
            }

            // Generic first pass — user has at least one passed submission
            let passed_count = submissions::Entity::find()
                .filter(submissions::Column::UserId.eq(user_id))
                .filter(submissions::Column::Status.eq(submissions::SubmissionStatus::Passed))
                .count(db)
                .await?;

            Ok(passed_count >= achievement.condition_value as u64)
        }

        ConditionType::SpeedPass => {
            // Check if the current submission passed within the time limit (condition_value in seconds)
            if matches!(submission.status, submissions::SubmissionStatus::Passed) {
                let max_duration_seconds = achievement.condition_value;
                let duration_seconds = submission.duration_ms / 1000;
                return Ok(duration_seconds < max_duration_seconds);
            }
            Ok(false)
        }

        ConditionType::ForumAccepted => {
            // Forum feature not yet implemented — always false for now
            Ok(false)
        }
    }
}
