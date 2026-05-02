use sea_orm_migration::{prelude::extension::postgres::Type, prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the forum_category enum type
        manager
            .create_type(
                Type::create()
                    .as_enum(ForumCategoryEnum::Enum)
                    .values([
                        ForumCategoryEnum::General,
                        ForumCategoryEnum::TaskHelp,
                        ForumCategoryEnum::ShowAndTell,
                    ])
                    .to_owned(),
            )
            .await?;

        // Create forum_posts table
        manager
            .create_table(
                Table::create()
                    .table(ForumPosts::Table)
                    .if_not_exists()
                    .col(uuid(ForumPosts::Id).primary_key())
                    .col(uuid(ForumPosts::UserId).not_null())
                    .col(string_len(ForumPosts::Title, 300).not_null())
                    .col(text(ForumPosts::BodyMd).not_null())
                    .col(uuid_null(ForumPosts::TaskId))
                    .col(
                        ColumnDef::new(ForumPosts::Category)
                            .custom(ForumCategoryEnum::Enum)
                            .not_null()
                            .default("general")
                            .to_owned(),
                    )
                    .col(integer(ForumPosts::Votes).default(0).not_null())
                    .col(integer(ForumPosts::Views).default(0).not_null())
                    .col(integer(ForumPosts::ReplyCount).default(0).not_null())
                    .col(
                        boolean(ForumPosts::IsPinned)
                            .default(false)
                            .not_null()
                            .to_owned(),
                    )
                    .col(
                        timestamp_with_time_zone(ForumPosts::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp_with_time_zone(ForumPosts::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_forum_posts_user_id")
                            .from(ForumPosts::Table, ForumPosts::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_forum_posts_task_id")
                            .from(ForumPosts::Table, ForumPosts::TaskId)
                            .to(Tasks::Table, Tasks::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // Create forum_replies table
        manager
            .create_table(
                Table::create()
                    .table(ForumReplies::Table)
                    .if_not_exists()
                    .col(uuid(ForumReplies::Id).primary_key())
                    .col(uuid(ForumReplies::PostId).not_null())
                    .col(uuid(ForumReplies::UserId).not_null())
                    .col(text(ForumReplies::BodyMd).not_null())
                    .col(integer(ForumReplies::Votes).default(0).not_null())
                    .col(
                        boolean(ForumReplies::IsAccepted)
                            .default(false)
                            .not_null()
                            .to_owned(),
                    )
                    .col(
                        timestamp_with_time_zone(ForumReplies::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp_with_time_zone(ForumReplies::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_forum_replies_post_id")
                            .from(ForumReplies::Table, ForumReplies::PostId)
                            .to(ForumPosts::Table, ForumPosts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_forum_replies_user_id")
                            .from(ForumReplies::Table, ForumReplies::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create articles table
        manager
            .create_table(
                Table::create()
                    .table(Articles::Table)
                    .if_not_exists()
                    .col(uuid(Articles::Id).primary_key())
                    .col(uuid(Articles::AuthorId).not_null())
                    .col(string_len(Articles::Title, 300).not_null())
                    .col(text(Articles::BodyMd).not_null())
                    .col(text_null(Articles::CoverImageUrl))
                    .col(
                        ColumnDef::new(Articles::Tags)
                            .array(ColumnType::Text)
                            .default(Expr::cust("'{}'::text[]"))
                            .not_null()
                            .to_owned(),
                    )
                    .col(
                        boolean(Articles::IsPublished)
                            .default(false)
                            .not_null()
                            .to_owned(),
                    )
                    .col(integer(Articles::Views).default(0).not_null())
                    .col(integer(Articles::Likes).default(0).not_null())
                    .col(
                        timestamp_with_time_zone(Articles::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp_with_time_zone(Articles::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_articles_author_id")
                            .from(Articles::Table, Articles::AuthorId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create reviews table
        manager
            .create_table(
                Table::create()
                    .table(Reviews::Table)
                    .if_not_exists()
                    .col(uuid(Reviews::Id).primary_key())
                    .col(uuid(Reviews::UserId).not_null())
                    .col(integer(Reviews::Rating).not_null())
                    .col(text(Reviews::Body).not_null())
                    .col(
                        boolean(Reviews::IsFeatured)
                            .default(false)
                            .not_null()
                            .to_owned(),
                    )
                    .col(
                        timestamp_with_time_zone(Reviews::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reviews_user_id")
                            .from(Reviews::Table, Reviews::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Add check constraint for reviews rating (1-5)
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE reviews ADD CONSTRAINT chk_reviews_rating CHECK (rating >= 1 AND rating <= 5)",
            )
            .await?;

        // Indexes for forum_posts
        manager
            .create_index(
                Index::create()
                    .name("idx_forum_posts_user_id")
                    .table(ForumPosts::Table)
                    .col(ForumPosts::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_forum_posts_category")
                    .table(ForumPosts::Table)
                    .col(ForumPosts::Category)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_forum_posts_task_id")
                    .table(ForumPosts::Table)
                    .col(ForumPosts::TaskId)
                    .to_owned(),
            )
            .await?;

        // Indexes for forum_replies
        manager
            .create_index(
                Index::create()
                    .name("idx_forum_replies_post_id")
                    .table(ForumReplies::Table)
                    .col(ForumReplies::PostId)
                    .to_owned(),
            )
            .await?;

        // Indexes for articles
        manager
            .create_index(
                Index::create()
                    .name("idx_articles_author_id")
                    .table(Articles::Table)
                    .col(Articles::AuthorId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_articles_is_published")
                    .table(Articles::Table)
                    .col(Articles::IsPublished)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Reviews::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Articles::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ForumReplies::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ForumPosts::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(ForumCategoryEnum::Enum).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ForumPosts {
    Table,
    Id,
    UserId,
    Title,
    BodyMd,
    TaskId,
    Category,
    Votes,
    Views,
    ReplyCount,
    IsPinned,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum ForumReplies {
    Table,
    Id,
    PostId,
    UserId,
    BodyMd,
    Votes,
    IsAccepted,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Articles {
    Table,
    Id,
    AuthorId,
    Title,
    BodyMd,
    CoverImageUrl,
    Tags,
    IsPublished,
    Views,
    Likes,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Reviews {
    Table,
    Id,
    UserId,
    Rating,
    Body,
    IsFeatured,
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
enum ForumCategoryEnum {
    #[sea_orm(iden = "forum_category")]
    Enum,
    #[sea_orm(iden = "general")]
    General,
    #[sea_orm(iden = "task_help")]
    TaskHelp,
    #[sea_orm(iden = "show_and_tell")]
    ShowAndTell,
}
