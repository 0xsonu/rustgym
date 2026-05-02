use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserAchievements::Table)
                    .if_not_exists()
                    .col(uuid(UserAchievements::Id).primary_key())
                    .col(uuid(UserAchievements::UserId).not_null())
                    .col(uuid(UserAchievements::AchievementId).not_null())
                    .col(
                        timestamp_with_time_zone(UserAchievements::EarnedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_achievements_user_id")
                            .from(UserAchievements::Table, UserAchievements::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_achievements_achievement_id")
                            .from(UserAchievements::Table, UserAchievements::AchievementId)
                            .to(Achievements::Table, Achievements::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique constraint on (user_id, achievement_id)
        manager
            .create_index(
                Index::create()
                    .name("idx_user_achievements_unique")
                    .table(UserAchievements::Table)
                    .col(UserAchievements::UserId)
                    .col(UserAchievements::AchievementId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Index for querying user's achievements
        manager
            .create_index(
                Index::create()
                    .name("idx_user_achievements_user_id")
                    .table(UserAchievements::Table)
                    .col(UserAchievements::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserAchievements::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum UserAchievements {
    Table,
    Id,
    UserId,
    AchievementId,
    EarnedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Achievements {
    Table,
    Id,
}
