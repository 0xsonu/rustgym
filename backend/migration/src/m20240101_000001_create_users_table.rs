use sea_orm_migration::{prelude::*, prelude::extension::postgres::Type, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the user_role enum type
        manager
            .create_type(
                Type::create()
                    .as_enum(UserRole::Enum)
                    .values([UserRole::Student, UserRole::Mentor, UserRole::Admin])
                    .to_owned(),
            )
            .await?;

        // Create the users table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(uuid(Users::Id).primary_key())
                    .col(string_len(Users::Username, 32).unique_key().not_null())
                    .col(string_len(Users::Email, 255).unique_key().not_null())
                    .col(text(Users::PasswordHash).not_null())
                    .col(text_null(Users::AvatarUrl))
                    .col(text_null(Users::Bio))
                    .col(
                        ColumnDef::new(Users::Role)
                            .custom(UserRole::Enum)
                            .default("student")
                            .not_null()
                            .to_owned(),
                    )
                    .col(integer(Users::Xp).default(0).not_null())
                    .col(integer(Users::Level).default(1).not_null())
                    .col(integer(Users::StreakDays).default(0).not_null())
                    .col(timestamp_with_time_zone_null(Users::LastActiveAt))
                    .col(boolean(Users::IsVerified).default(false).not_null())
                    .col(boolean(Users::IsBanned).default(false).not_null())
                    .col(
                        timestamp_with_time_zone(Users::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp_with_time_zone(Users::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique index on username
        manager
            .create_index(
                Index::create()
                    .name("idx_users_username")
                    .table(Users::Table)
                    .col(Users::Username)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create unique index on email
        manager
            .create_index(
                Index::create()
                    .name("idx_users_email")
                    .table(Users::Table)
                    .col(Users::Email)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(UserRole::Enum).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    AvatarUrl,
    Bio,
    Role,
    Xp,
    Level,
    StreakDays,
    LastActiveAt,
    IsVerified,
    IsBanned,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum UserRole {
    #[sea_orm(iden = "user_role")]
    Enum,
    #[sea_orm(iden = "student")]
    Student,
    #[sea_orm(iden = "mentor")]
    Mentor,
    #[sea_orm(iden = "admin")]
    Admin,
}
