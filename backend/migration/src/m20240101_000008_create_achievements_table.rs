use sea_orm_migration::{prelude::extension::postgres::Type, prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the achievement_condition_type enum
        manager
            .create_type(
                Type::create()
                    .as_enum(AchievementConditionTypeEnum::Enum)
                    .values([
                        AchievementConditionTypeEnum::TasksCompleted,
                        AchievementConditionTypeEnum::XpEarned,
                        AchievementConditionTypeEnum::Streak,
                        AchievementConditionTypeEnum::QuestDone,
                        AchievementConditionTypeEnum::FirstSubmit,
                        AchievementConditionTypeEnum::FirstPass,
                        AchievementConditionTypeEnum::SpeedPass,
                        AchievementConditionTypeEnum::ForumAccepted,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Achievements::Table)
                    .if_not_exists()
                    .col(uuid(Achievements::Id).primary_key())
                    .col(string_len(Achievements::Slug, 100).unique_key().not_null())
                    .col(string_len(Achievements::Name, 100).not_null())
                    .col(text_null(Achievements::Description))
                    .col(text_null(Achievements::Icon))
                    .col(integer(Achievements::XpReward).default(0).not_null())
                    .col(
                        ColumnDef::new(Achievements::ConditionType)
                            .custom(AchievementConditionTypeEnum::Enum)
                            .not_null()
                            .to_owned(),
                    )
                    .col(integer(Achievements::ConditionValue).not_null())
                    .col(
                        ColumnDef::new(Achievements::ConditionMeta)
                            .json_binary()
                            .null()
                            .to_owned(),
                    )
                    .col(
                        timestamp_with_time_zone(Achievements::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp_with_time_zone(Achievements::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Achievements::Table).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(AchievementConditionTypeEnum::Enum)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Achievements {
    Table,
    Id,
    Slug,
    Name,
    Description,
    Icon,
    XpReward,
    ConditionType,
    ConditionValue,
    ConditionMeta,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum AchievementConditionTypeEnum {
    #[sea_orm(iden = "achievement_condition_type")]
    Enum,
    #[sea_orm(iden = "tasks_completed")]
    TasksCompleted,
    #[sea_orm(iden = "xp_earned")]
    XpEarned,
    #[sea_orm(iden = "streak")]
    Streak,
    #[sea_orm(iden = "quest_done")]
    QuestDone,
    #[sea_orm(iden = "first_submit")]
    FirstSubmit,
    #[sea_orm(iden = "first_pass")]
    FirstPass,
    #[sea_orm(iden = "speed_pass")]
    SpeedPass,
    #[sea_orm(iden = "forum_accepted")]
    ForumAccepted,
}
