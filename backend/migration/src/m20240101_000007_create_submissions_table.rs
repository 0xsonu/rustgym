use sea_orm_migration::{prelude::extension::postgres::Type, prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the submission_status enum type
        manager
            .create_type(
                Type::create()
                    .as_enum(SubmissionStatusEnum::Enum)
                    .values([
                        SubmissionStatusEnum::Pending,
                        SubmissionStatusEnum::Running,
                        SubmissionStatusEnum::Passed,
                        SubmissionStatusEnum::Failed,
                        SubmissionStatusEnum::Error,
                        SubmissionStatusEnum::Timeout,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Submissions::Table)
                    .if_not_exists()
                    .col(uuid(Submissions::Id).primary_key())
                    .col(uuid(Submissions::UserId).not_null())
                    .col(uuid(Submissions::TaskId).not_null())
                    .col(text(Submissions::Code).not_null())
                    .col(
                        ColumnDef::new(Submissions::Status)
                            .custom(SubmissionStatusEnum::Enum)
                            .not_null()
                            .to_owned(),
                    )
                    .col(
                        ColumnDef::new(Submissions::TestResults)
                            .json_binary()
                            .default(Expr::cust("'[]'::jsonb"))
                            .not_null()
                            .to_owned(),
                    )
                    .col(text(Submissions::Stdout).default("").not_null())
                    .col(text(Submissions::Stderr).default("").not_null())
                    .col(integer(Submissions::DurationMs).default(0).not_null())
                    .col(integer(Submissions::MemoryKb).default(0).not_null())
                    .col(integer(Submissions::XpAwarded).default(0).not_null())
                    .col(integer(Submissions::AttemptNumber).not_null())
                    .col(
                        timestamp_with_time_zone(Submissions::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submissions_user_id")
                            .from(Submissions::Table, Submissions::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submissions_task_id")
                            .from(Submissions::Table, Submissions::TaskId)
                            .to(Tasks::Table, Tasks::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Index: (user_id, task_id, created_at DESC) for submission history
        manager
            .create_index(
                Index::create()
                    .name("idx_submissions_user_task_created")
                    .table(Submissions::Table)
                    .col(Submissions::UserId)
                    .col(Submissions::TaskId)
                    .col((Submissions::CreatedAt, IndexOrder::Desc))
                    .to_owned(),
            )
            .await?;

        // Index: (user_id, created_at DESC) for recent submissions
        manager
            .create_index(
                Index::create()
                    .name("idx_submissions_user_created")
                    .table(Submissions::Table)
                    .col(Submissions::UserId)
                    .col((Submissions::CreatedAt, IndexOrder::Desc))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Submissions::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(SubmissionStatusEnum::Enum).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Submissions {
    Table,
    Id,
    UserId,
    TaskId,
    Code,
    Status,
    TestResults,
    Stdout,
    Stderr,
    DurationMs,
    MemoryKb,
    XpAwarded,
    AttemptNumber,
    CreatedAt,
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
enum SubmissionStatusEnum {
    #[sea_orm(iden = "submission_status")]
    Enum,
    #[sea_orm(iden = "pending")]
    Pending,
    #[sea_orm(iden = "running")]
    Running,
    #[sea_orm(iden = "passed")]
    Passed,
    #[sea_orm(iden = "failed")]
    Failed,
    #[sea_orm(iden = "error")]
    Error,
    #[sea_orm(iden = "timeout")]
    Timeout,
}
