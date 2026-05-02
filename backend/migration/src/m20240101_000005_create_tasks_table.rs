use sea_orm_migration::{prelude::*, prelude::extension::postgres::Type, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the difficulty enum type
        manager
            .create_type(
                Type::create()
                    .as_enum(DifficultyEnum::Enum)
                    .values([
                        DifficultyEnum::Beginner,
                        DifficultyEnum::Easy,
                        DifficultyEnum::Medium,
                        DifficultyEnum::Hard,
                        DifficultyEnum::Advanced,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Tasks::Table)
                    .if_not_exists()
                    .col(uuid(Tasks::Id).primary_key())
                    .col(uuid(Tasks::LevelId).not_null())
                    .col(string_len(Tasks::Slug, 100).unique_key().not_null())
                    .col(string_len(Tasks::Title, 200).not_null())
                    .col(text(Tasks::DescriptionMd).not_null())
                    .col(text(Tasks::StarterCode).not_null())
                    .col(text(Tasks::SolutionCode).not_null())
                    .col(text(Tasks::TestCode).not_null())
                    .col(text(Tasks::CargoToml).not_null())
                    .col(
                        ColumnDef::new(Tasks::Difficulty)
                            .custom(DifficultyEnum::Enum)
                            .not_null()
                            .to_owned(),
                    )
                    .col(integer(Tasks::XpReward).not_null())
                    .col(integer(Tasks::OrderIndex).not_null())
                    .col(boolean(Tasks::IsPublished).default(false).not_null())
                    .col(
                        ColumnDef::new(Tasks::Tags)
                            .array(ColumnType::Text)
                            .default(Expr::cust("'{}'"))
                            .not_null()
                            .to_owned(),
                    )
                    .col(text_null(Tasks::HintMd))
                    .col(
                        ColumnDef::new(Tasks::SyntestRules)
                            .array(ColumnType::Text)
                            .default(Expr::cust("'{}'"))
                            .not_null()
                            .to_owned(),
                    )
                    .col(
                        timestamp_with_time_zone(Tasks::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp_with_time_zone(Tasks::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tasks_level_id")
                            .from(Tasks::Table, Tasks::LevelId)
                            .to(Levels::Table, Levels::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create composite index on (level_id, order_index) for ordered task retrieval
        manager
            .create_index(
                Index::create()
                    .name("idx_tasks_level_id_order_index")
                    .table(Tasks::Table)
                    .col(Tasks::LevelId)
                    .col(Tasks::OrderIndex)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tasks::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(DifficultyEnum::Enum).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Tasks {
    Table,
    Id,
    LevelId,
    Slug,
    Title,
    DescriptionMd,
    StarterCode,
    SolutionCode,
    TestCode,
    CargoToml,
    Difficulty,
    XpReward,
    OrderIndex,
    IsPublished,
    Tags,
    HintMd,
    SyntestRules,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Levels {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum DifficultyEnum {
    #[sea_orm(iden = "difficulty")]
    Enum,
    #[sea_orm(iden = "beginner")]
    Beginner,
    #[sea_orm(iden = "easy")]
    Easy,
    #[sea_orm(iden = "medium")]
    Medium,
    #[sea_orm(iden = "hard")]
    Hard,
    #[sea_orm(iden = "advanced")]
    Advanced,
}
