use sea_orm_migration::{prelude::extension::postgres::Type, prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the progress_status enum type
        manager
            .create_type(
                Type::create()
                    .as_enum(ProgressStatusEnum::Enum)
                    .values([
                        ProgressStatusEnum::NotStarted,
                        ProgressStatusEnum::InProgress,
                        ProgressStatusEnum::Completed,
                    ])
                    .to_owned(),
            )
            .await?;

        // Create user_task_progress table
        manager
            .create_table(
                Table::create()
                    .table(UserTaskProgress::Table)
                    .if_not_exists()
                    .col(uuid(UserTaskProgress::Id).primary_key())
                    .col(uuid(UserTaskProgress::UserId).not_null())
                    .col(uuid(UserTaskProgress::TaskId).not_null())
                    .col(
                        ColumnDef::new(UserTaskProgress::Status)
                            .custom(ProgressStatusEnum::Enum)
                            .default("not_started")
                            .not_null()
                            .to_owned(),
                    )
                    .col(uuid_null(UserTaskProgress::BestSubmissionId))
                    .col(integer(UserTaskProgress::Attempts).default(0).not_null())
                    .col(timestamp_with_time_zone_null(UserTaskProgress::CompletedAt))
                    .col(
                        timestamp_with_time_zone(UserTaskProgress::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp_with_time_zone(UserTaskProgress::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_task_progress_user_id")
                            .from(UserTaskProgress::Table, UserTaskProgress::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_task_progress_task_id")
                            .from(UserTaskProgress::Table, UserTaskProgress::TaskId)
                            .to(Tasks::Table, Tasks::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique constraint on (user_id, task_id)
        manager
            .create_index(
                Index::create()
                    .name("idx_user_task_progress_user_task_unique")
                    .table(UserTaskProgress::Table)
                    .col(UserTaskProgress::UserId)
                    .col(UserTaskProgress::TaskId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create user_level_progress table
        manager
            .create_table(
                Table::create()
                    .table(UserLevelProgress::Table)
                    .if_not_exists()
                    .col(uuid(UserLevelProgress::Id).primary_key())
                    .col(uuid(UserLevelProgress::UserId).not_null())
                    .col(uuid(UserLevelProgress::LevelId).not_null())
                    .col(
                        integer(UserLevelProgress::TasksCompleted)
                            .default(0)
                            .not_null(),
                    )
                    .col(integer(UserLevelProgress::TasksTotal).not_null())
                    .col(
                        boolean(UserLevelProgress::IsCompleted)
                            .default(false)
                            .not_null(),
                    )
                    .col(timestamp_with_time_zone_null(
                        UserLevelProgress::CompletedAt,
                    ))
                    .col(
                        timestamp_with_time_zone(UserLevelProgress::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp_with_time_zone(UserLevelProgress::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_level_progress_user_id")
                            .from(UserLevelProgress::Table, UserLevelProgress::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_level_progress_level_id")
                            .from(UserLevelProgress::Table, UserLevelProgress::LevelId)
                            .to(Levels::Table, Levels::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique constraint on (user_id, level_id)
        manager
            .create_index(
                Index::create()
                    .name("idx_user_level_progress_user_level_unique")
                    .table(UserLevelProgress::Table)
                    .col(UserLevelProgress::UserId)
                    .col(UserLevelProgress::LevelId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create user_quest_progress table
        manager
            .create_table(
                Table::create()
                    .table(UserQuestProgress::Table)
                    .if_not_exists()
                    .col(uuid(UserQuestProgress::Id).primary_key())
                    .col(uuid(UserQuestProgress::UserId).not_null())
                    .col(uuid(UserQuestProgress::QuestId).not_null())
                    .col(
                        integer(UserQuestProgress::LevelsCompleted)
                            .default(0)
                            .not_null(),
                    )
                    .col(integer(UserQuestProgress::LevelsTotal).not_null())
                    .col(
                        integer(UserQuestProgress::TasksCompleted)
                            .default(0)
                            .not_null(),
                    )
                    .col(integer(UserQuestProgress::TasksTotal).not_null())
                    .col(
                        boolean(UserQuestProgress::IsCompleted)
                            .default(false)
                            .not_null(),
                    )
                    .col(timestamp_with_time_zone_null(
                        UserQuestProgress::CompletedAt,
                    ))
                    .col(
                        timestamp_with_time_zone(UserQuestProgress::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp_with_time_zone(UserQuestProgress::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_quest_progress_user_id")
                            .from(UserQuestProgress::Table, UserQuestProgress::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_quest_progress_quest_id")
                            .from(UserQuestProgress::Table, UserQuestProgress::QuestId)
                            .to(Quests::Table, Quests::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique constraint on (user_id, quest_id)
        manager
            .create_index(
                Index::create()
                    .name("idx_user_quest_progress_user_quest_unique")
                    .table(UserQuestProgress::Table)
                    .col(UserQuestProgress::UserId)
                    .col(UserQuestProgress::QuestId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserQuestProgress::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserLevelProgress::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserTaskProgress::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(ProgressStatusEnum::Enum).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum UserTaskProgress {
    Table,
    Id,
    UserId,
    TaskId,
    Status,
    BestSubmissionId,
    Attempts,
    CompletedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum UserLevelProgress {
    Table,
    Id,
    UserId,
    LevelId,
    TasksCompleted,
    TasksTotal,
    IsCompleted,
    CompletedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum UserQuestProgress {
    Table,
    Id,
    UserId,
    QuestId,
    LevelsCompleted,
    LevelsTotal,
    TasksCompleted,
    TasksTotal,
    IsCompleted,
    CompletedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Tasks {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Levels {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Quests {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum ProgressStatusEnum {
    #[sea_orm(iden = "progress_status")]
    Enum,
    #[sea_orm(iden = "not_started")]
    NotStarted,
    #[sea_orm(iden = "in_progress")]
    InProgress,
    #[sea_orm(iden = "completed")]
    Completed,
}
