use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user_quest_progress")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub quest_id: Uuid,
    #[sea_orm(default_value = "0")]
    pub levels_completed: i32,
    pub levels_total: i32,
    #[sea_orm(default_value = "0")]
    pub tasks_completed: i32,
    pub tasks_total: i32,
    #[sea_orm(default_value = "false")]
    pub is_completed: bool,
    #[sea_orm(nullable)]
    pub completed_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
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
        belongs_to = "super::quests::Entity",
        from = "Column::QuestId",
        to = "super::quests::Column::Id"
    )]
    Quest,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::quests::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Quest.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
