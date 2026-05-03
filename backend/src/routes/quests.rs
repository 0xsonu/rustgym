use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use crate::{
    dto::quest::{
        LevelDetailResponse, LevelProgress, LevelSummary, QuestDetailResponse, QuestListResponse,
        QuestProgress, QuestSummary, TaskProgress, TaskSummary,
    },
    error::AppError,
    middleware::auth::OptionalCurrentUser,
    AppState,
};

use entity::{levels, quests, tasks, user_level_progress, user_quest_progress, user_task_progress};

/// Build the quests router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_quests))
        .route("/:slug", get(get_quest_detail))
        .route("/:quest_slug/levels/:level_slug", get(get_level_detail))
}

// ─── GET /api/v1/quests ──────────────────────────────────────────────────────

async fn list_quests(
    State(state): State<AppState>,
    user: OptionalCurrentUser,
) -> Result<Json<QuestListResponse>, AppError> {
    // Fetch all published quests ordered by order_index
    let all_quests = quests::Entity::find()
        .filter(quests::Column::IsPublished.eq(true))
        .order_by_asc(quests::Column::OrderIndex)
        .all(&state.db)
        .await?;

    let user_id: Option<Uuid> = user
        .0
        .as_ref()
        .and_then(|u| Uuid::parse_str(u.user_id()).ok());

    // Fetch user progress if authenticated
    let user_quest_progresses = if let Some(uid) = user_id {
        user_quest_progress::Entity::find()
            .filter(user_quest_progress::Column::UserId.eq(uid))
            .all(&state.db)
            .await?
    } else {
        vec![]
    };

    let mut quest_summaries = Vec::with_capacity(all_quests.len());

    for quest in &all_quests {
        // Count levels for this quest
        let level_count = levels::Entity::find()
            .filter(levels::Column::QuestId.eq(quest.id))
            .filter(levels::Column::IsPublished.eq(true))
            .all(&state.db)
            .await?
            .len() as i64;

        // Count tasks for this quest (through levels)
        let quest_levels = levels::Entity::find()
            .filter(levels::Column::QuestId.eq(quest.id))
            .all(&state.db)
            .await?;

        let mut task_count: i64 = 0;
        for level in &quest_levels {
            let count = tasks::Entity::find()
                .filter(tasks::Column::LevelId.eq(level.id))
                .filter(tasks::Column::IsPublished.eq(true))
                .all(&state.db)
                .await?
                .len() as i64;
            task_count += count;
        }

        // Find user progress for this quest
        let progress = user_quest_progresses
            .iter()
            .find(|p| p.quest_id == quest.id)
            .map(|p| QuestProgress {
                tasks_completed: p.tasks_completed,
                tasks_total: p.tasks_total,
                is_completed: p.is_completed,
            });

        quest_summaries.push(QuestSummary {
            id: quest.id,
            slug: quest.slug.clone(),
            title: quest.title.clone(),
            description: quest.description.clone(),
            icon: quest.icon.clone(),
            color: quest.color.clone(),
            order_index: quest.order_index,
            is_published: quest.is_published,
            prerequisite_quest_id: quest.prerequisite_quest_id,
            level_count,
            task_count,
            user_progress: progress,
        });
    }

    Ok(Json(QuestListResponse {
        quests: quest_summaries,
    }))
}

// ─── GET /api/v1/quests/:slug ────────────────────────────────────────────────

async fn get_quest_detail(
    State(state): State<AppState>,
    user: OptionalCurrentUser,
    Path(slug): Path<String>,
) -> Result<Json<QuestDetailResponse>, AppError> {
    // Find quest by slug
    let quest = quests::Entity::find()
        .filter(quests::Column::Slug.eq(&slug))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Quest not found".to_string()))?;

    let user_id: Option<Uuid> = user
        .0
        .as_ref()
        .and_then(|u| Uuid::parse_str(u.user_id()).ok());

    // Fetch levels for this quest
    let quest_levels = levels::Entity::find()
        .filter(levels::Column::QuestId.eq(quest.id))
        .filter(levels::Column::IsPublished.eq(true))
        .order_by_asc(levels::Column::OrderIndex)
        .all(&state.db)
        .await?;

    // Fetch user level progress if authenticated
    let user_level_progresses = if let Some(uid) = user_id {
        user_level_progress::Entity::find()
            .filter(user_level_progress::Column::UserId.eq(uid))
            .all(&state.db)
            .await?
    } else {
        vec![]
    };

    // Build level summaries
    let mut level_summaries = Vec::with_capacity(quest_levels.len());
    let mut total_task_count: i64 = 0;

    for level in &quest_levels {
        let task_count = tasks::Entity::find()
            .filter(tasks::Column::LevelId.eq(level.id))
            .filter(tasks::Column::IsPublished.eq(true))
            .all(&state.db)
            .await?
            .len() as i64;

        total_task_count += task_count;

        let progress = user_level_progresses
            .iter()
            .find(|p| p.level_id == level.id)
            .map(|p| LevelProgress {
                tasks_completed: p.tasks_completed,
                tasks_total: p.tasks_total,
                is_completed: p.is_completed,
            });

        level_summaries.push(LevelSummary {
            id: level.id,
            quest_id: quest.id,
            slug: level.slug.clone(),
            title: level.title.clone(),
            description: level.description.clone(),
            order_index: level.order_index,
            task_count,
            user_progress: progress,
        });
    }

    // Build quest summary
    let user_quest_prog = if let Some(uid) = user_id {
        user_quest_progress::Entity::find()
            .filter(user_quest_progress::Column::UserId.eq(uid))
            .filter(user_quest_progress::Column::QuestId.eq(quest.id))
            .one(&state.db)
            .await?
    } else {
        None
    };

    let quest_progress = user_quest_prog.map(|p| QuestProgress {
        tasks_completed: p.tasks_completed,
        tasks_total: p.tasks_total,
        is_completed: p.is_completed,
    });

    let quest_summary = QuestSummary {
        id: quest.id,
        slug: quest.slug.clone(),
        title: quest.title.clone(),
        description: quest.description.clone(),
        icon: quest.icon.clone(),
        color: quest.color.clone(),
        order_index: quest.order_index,
        is_published: quest.is_published,
        prerequisite_quest_id: quest.prerequisite_quest_id,
        level_count: quest_levels.len() as i64,
        task_count: total_task_count,
        user_progress: quest_progress,
    };

    Ok(Json(QuestDetailResponse {
        quest: quest_summary,
        levels: level_summaries,
    }))
}

// ─── GET /api/v1/quests/:quest_slug/levels/:level_slug ───────────────────────

async fn get_level_detail(
    State(state): State<AppState>,
    user: OptionalCurrentUser,
    Path((quest_slug, level_slug)): Path<(String, String)>,
) -> Result<Json<LevelDetailResponse>, AppError> {
    // Find quest by slug first
    let quest = quests::Entity::find()
        .filter(quests::Column::Slug.eq(&quest_slug))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Quest not found".to_string()))?;

    // Find level by slug within the quest
    let level = levels::Entity::find()
        .filter(levels::Column::QuestId.eq(quest.id))
        .filter(levels::Column::Slug.eq(&level_slug))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Level not found".to_string()))?;

    let user_id: Option<Uuid> = user
        .0
        .as_ref()
        .and_then(|u| Uuid::parse_str(u.user_id()).ok());

    // Fetch tasks for this level
    let level_tasks = tasks::Entity::find()
        .filter(tasks::Column::LevelId.eq(level.id))
        .filter(tasks::Column::IsPublished.eq(true))
        .order_by_asc(tasks::Column::OrderIndex)
        .all(&state.db)
        .await?;

    // Fetch user task progress if authenticated
    let user_task_progresses = if let Some(uid) = user_id {
        user_task_progress::Entity::find()
            .filter(user_task_progress::Column::UserId.eq(uid))
            .all(&state.db)
            .await?
    } else {
        vec![]
    };

    // Build task summaries
    let task_summaries: Vec<TaskSummary> = level_tasks
        .iter()
        .map(|task| {
            let progress = user_task_progresses
                .iter()
                .find(|p| p.task_id == task.id)
                .map(|p| TaskProgress {
                    status: format!("{:?}", p.status).to_lowercase(),
                    attempts: p.attempts,
                    completed_at: p.completed_at.map(|dt| dt.to_rfc3339()),
                });

            TaskSummary {
                id: task.id,
                slug: task.slug.clone(),
                title: task.title.clone(),
                difficulty: format!("{:?}", task.difficulty).to_lowercase(),
                xp_reward: task.xp_reward,
                order_index: task.order_index,
                tags: task.tags.clone(),
                user_progress: progress,
            }
        })
        .collect();

    // Build level summary with progress
    let level_progress = if let Some(uid) = user_id {
        user_level_progress::Entity::find()
            .filter(user_level_progress::Column::UserId.eq(uid))
            .filter(user_level_progress::Column::LevelId.eq(level.id))
            .one(&state.db)
            .await?
            .map(|p| LevelProgress {
                tasks_completed: p.tasks_completed,
                tasks_total: p.tasks_total,
                is_completed: p.is_completed,
            })
    } else {
        None
    };

    let level_summary = LevelSummary {
        id: level.id,
        quest_id: quest.id,
        slug: level.slug.clone(),
        title: level.title.clone(),
        description: level.description.clone(),
        order_index: level.order_index,
        task_count: level_tasks.len() as i64,
        user_progress: level_progress,
    };

    Ok(Json(LevelDetailResponse {
        level: level_summary,
        tasks: task_summaries,
    }))
}
