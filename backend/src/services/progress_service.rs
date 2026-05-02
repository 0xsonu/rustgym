use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};
use uuid::Uuid;

use crate::error::AppError;
use entity::{levels, tasks, user_level_progress, user_quest_progress, user_task_progress};

/// Mark a task as completed and update level/quest progress.
pub async fn update_progress(
    db: &DatabaseConnection,
    user_id: Uuid,
    task_id: Uuid,
) -> Result<(), AppError> {
    let now = chrono::Utc::now().fixed_offset();

    // 1. Upsert user_task_progress to 'completed' status
    let existing_task_progress = user_task_progress::Entity::find()
        .filter(user_task_progress::Column::UserId.eq(user_id))
        .filter(user_task_progress::Column::TaskId.eq(task_id))
        .one(db)
        .await?;

    match existing_task_progress {
        Some(progress) => {
            let mut active: user_task_progress::ActiveModel = progress.into();
            active.status = Set(user_task_progress::ProgressStatus::Completed);
            active.completed_at = Set(Some(now));
            active.updated_at = Set(now);
            active.update(db).await?;
        }
        None => {
            let new_progress = user_task_progress::ActiveModel {
                id: Set(Uuid::new_v4()),
                user_id: Set(user_id),
                task_id: Set(task_id),
                status: Set(user_task_progress::ProgressStatus::Completed),
                best_submission_id: Set(None),
                attempts: Set(1),
                completed_at: Set(Some(now)),
                created_at: Set(now),
                updated_at: Set(now),
            };
            new_progress.insert(db).await?;
        }
    }

    // 2. Find the level for this task
    let task = tasks::Entity::find_by_id(task_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Task not found".to_string()))?;

    let level_id = task.level_id;

    // Count total tasks in this level
    let tasks_total = tasks::Entity::find()
        .filter(tasks::Column::LevelId.eq(level_id))
        .count(db)
        .await? as i32;

    // Count completed tasks in this level for this user
    let level_tasks: Vec<Uuid> = tasks::Entity::find()
        .filter(tasks::Column::LevelId.eq(level_id))
        .all(db)
        .await?
        .into_iter()
        .map(|t| t.id)
        .collect();

    let tasks_completed = user_task_progress::Entity::find()
        .filter(user_task_progress::Column::UserId.eq(user_id))
        .filter(user_task_progress::Column::TaskId.is_in(level_tasks))
        .filter(
            user_task_progress::Column::Status.eq(user_task_progress::ProgressStatus::Completed),
        )
        .count(db)
        .await? as i32;

    let level_is_completed = tasks_completed >= tasks_total;

    // 3. Upsert user_level_progress
    let existing_level_progress = user_level_progress::Entity::find()
        .filter(user_level_progress::Column::UserId.eq(user_id))
        .filter(user_level_progress::Column::LevelId.eq(level_id))
        .one(db)
        .await?;

    match existing_level_progress {
        Some(progress) => {
            let mut active: user_level_progress::ActiveModel = progress.into();
            active.tasks_completed = Set(tasks_completed);
            active.is_completed = Set(level_is_completed);
            if level_is_completed {
                active.completed_at = Set(Some(now));
            }
            active.updated_at = Set(now);
            active.update(db).await?;
        }
        None => {
            let new_progress = user_level_progress::ActiveModel {
                id: Set(Uuid::new_v4()),
                user_id: Set(user_id),
                level_id: Set(level_id),
                tasks_completed: Set(tasks_completed),
                tasks_total: Set(tasks_total),
                is_completed: Set(level_is_completed),
                completed_at: Set(if level_is_completed { Some(now) } else { None }),
                created_at: Set(now),
                updated_at: Set(now),
            };
            new_progress.insert(db).await?;
        }
    }

    // 4. Find the quest for this level, count completed levels in that quest
    let level = levels::Entity::find_by_id(level_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Level not found".to_string()))?;

    let quest_id = level.quest_id;

    // Count total levels in this quest
    let levels_total = levels::Entity::find()
        .filter(levels::Column::QuestId.eq(quest_id))
        .count(db)
        .await? as i32;

    // Get all level IDs in this quest
    let quest_level_ids: Vec<Uuid> = levels::Entity::find()
        .filter(levels::Column::QuestId.eq(quest_id))
        .all(db)
        .await?
        .into_iter()
        .map(|l| l.id)
        .collect();

    // Count completed levels for this user in this quest
    let levels_completed = user_level_progress::Entity::find()
        .filter(user_level_progress::Column::UserId.eq(user_id))
        .filter(user_level_progress::Column::LevelId.is_in(quest_level_ids.clone()))
        .filter(user_level_progress::Column::IsCompleted.eq(true))
        .count(db)
        .await? as i32;

    // Count total tasks in the quest
    let quest_tasks_total = tasks::Entity::find()
        .filter(tasks::Column::LevelId.is_in(quest_level_ids.clone()))
        .count(db)
        .await? as i32;

    // Count completed tasks in the quest for this user
    let quest_task_ids: Vec<Uuid> = tasks::Entity::find()
        .filter(tasks::Column::LevelId.is_in(quest_level_ids))
        .all(db)
        .await?
        .into_iter()
        .map(|t| t.id)
        .collect();

    let quest_tasks_completed = user_task_progress::Entity::find()
        .filter(user_task_progress::Column::UserId.eq(user_id))
        .filter(user_task_progress::Column::TaskId.is_in(quest_task_ids))
        .filter(
            user_task_progress::Column::Status.eq(user_task_progress::ProgressStatus::Completed),
        )
        .count(db)
        .await? as i32;

    let quest_is_completed = levels_completed >= levels_total;

    // 5. Upsert user_quest_progress
    let existing_quest_progress = user_quest_progress::Entity::find()
        .filter(user_quest_progress::Column::UserId.eq(user_id))
        .filter(user_quest_progress::Column::QuestId.eq(quest_id))
        .one(db)
        .await?;

    match existing_quest_progress {
        Some(progress) => {
            let mut active: user_quest_progress::ActiveModel = progress.into();
            active.levels_completed = Set(levels_completed);
            active.tasks_completed = Set(quest_tasks_completed);
            active.is_completed = Set(quest_is_completed);
            if quest_is_completed {
                active.completed_at = Set(Some(now));
            }
            active.updated_at = Set(now);
            active.update(db).await?;
        }
        None => {
            let new_progress = user_quest_progress::ActiveModel {
                id: Set(Uuid::new_v4()),
                user_id: Set(user_id),
                quest_id: Set(quest_id),
                levels_completed: Set(levels_completed),
                levels_total: Set(levels_total),
                tasks_completed: Set(quest_tasks_completed),
                tasks_total: Set(quest_tasks_total),
                is_completed: Set(quest_is_completed),
                completed_at: Set(if quest_is_completed { Some(now) } else { None }),
                created_at: Set(now),
                updated_at: Set(now),
            };
            new_progress.insert(db).await?;
        }
    }

    Ok(())
}
