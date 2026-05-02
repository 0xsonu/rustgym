use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "levels")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub quest_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(100))", unique)]
    pub slug: String,
    #[sea_orm(column_type = "String(StringLen::N(200))")]
    pub title: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub order_index: i32,
    #[sea_orm(default_value = "false")]
    pub is_published: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::quests::Entity",
        from = "Column::QuestId",
        to = "super::quests::Column::Id"
    )]
    Quest,
    #[sea_orm(has_many = "super::tasks::Entity")]
    Tasks,
}

impl Related<super::quests::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Quest.def()
    }
}

impl Related<super::tasks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tasks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
