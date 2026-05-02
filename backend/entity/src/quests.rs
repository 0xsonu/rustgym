use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "quests")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(100))", unique)]
    pub slug: String,
    #[sea_orm(column_type = "String(StringLen::N(200))")]
    pub title: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub order_index: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub icon: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(20))", nullable)]
    pub color: Option<String>,
    #[sea_orm(default_value = "false")]
    pub is_published: bool,
    #[sea_orm(nullable)]
    pub prerequisite_quest_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::levels::Entity")]
    Levels,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::PrerequisiteQuestId",
        to = "Column::Id"
    )]
    PrerequisiteQuest,
}

impl Related<super::levels::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Levels.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
