use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Quests::Table)
                    .if_not_exists()
                    .col(uuid(Quests::Id).primary_key())
                    .col(string_len(Quests::Slug, 100).unique_key().not_null())
                    .col(string_len(Quests::Title, 200).not_null())
                    .col(text_null(Quests::Description))
                    .col(integer(Quests::OrderIndex).not_null())
                    .col(text_null(Quests::Icon))
                    .col(string_len_null(Quests::Color, 20))
                    .col(boolean(Quests::IsPublished).default(false).not_null())
                    .col(uuid_null(Quests::PrerequisiteQuestId))
                    .col(
                        timestamp_with_time_zone(Quests::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp_with_time_zone(Quests::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_quests_prerequisite_quest_id")
                            .from(Quests::Table, Quests::PrerequisiteQuestId)
                            .to(Quests::Table, Quests::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on order_index for ordered retrieval
        manager
            .create_index(
                Index::create()
                    .name("idx_quests_order_index")
                    .table(Quests::Table)
                    .col(Quests::OrderIndex)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Quests::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Quests {
    Table,
    Id,
    Slug,
    Title,
    Description,
    OrderIndex,
    Icon,
    Color,
    IsPublished,
    PrerequisiteQuestId,
    CreatedAt,
    UpdatedAt,
}
