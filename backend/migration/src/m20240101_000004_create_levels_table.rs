use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Levels::Table)
                    .if_not_exists()
                    .col(uuid(Levels::Id).primary_key())
                    .col(uuid(Levels::QuestId).not_null())
                    .col(string_len(Levels::Slug, 100).unique_key().not_null())
                    .col(string_len(Levels::Title, 200).not_null())
                    .col(text_null(Levels::Description))
                    .col(integer(Levels::OrderIndex).not_null())
                    .col(boolean(Levels::IsPublished).default(false).not_null())
                    .col(
                        timestamp_with_time_zone(Levels::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp_with_time_zone(Levels::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_levels_quest_id")
                            .from(Levels::Table, Levels::QuestId)
                            .to(Quests::Table, Quests::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create composite index on (quest_id, order_index) for ordered level retrieval
        manager
            .create_index(
                Index::create()
                    .name("idx_levels_quest_id_order_index")
                    .table(Levels::Table)
                    .col(Levels::QuestId)
                    .col(Levels::OrderIndex)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Levels::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Levels {
    Table,
    Id,
    QuestId,
    Slug,
    Title,
    Description,
    OrderIndex,
    IsPublished,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Quests {
    Table,
    Id,
}
