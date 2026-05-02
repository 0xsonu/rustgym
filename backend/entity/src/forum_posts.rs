use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "forum_posts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub body_md: String,
    #[sea_orm(nullable)]
    pub task_id: Option<Uuid>,
    pub category: ForumCategory,
    #[sea_orm(default_value = "0")]
    pub votes: i32,
    #[sea_orm(default_value = "0")]
    pub views: i32,
    #[sea_orm(default_value = "0")]
    pub reply_count: i32,
    #[sea_orm(default_value = "false")]
    pub is_pinned: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "forum_category")]
pub enum ForumCategory {
    #[sea_orm(string_value = "general")]
    General,
    #[sea_orm(string_value = "task_help")]
    TaskHelp,
    #[sea_orm(string_value = "show_and_tell")]
    ShowAndTell,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::tasks::Entity",
        from = "Column::TaskId",
        to = "super::tasks::Column::Id"
    )]
    Task,
    #[sea_orm(has_many = "super::forum_replies::Entity")]
    Replies,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::tasks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl Related<super::forum_replies::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Replies.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
