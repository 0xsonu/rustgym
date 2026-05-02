use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "difficulty")]
pub enum Difficulty {
    #[sea_orm(string_value = "beginner")]
    Beginner,
    #[sea_orm(string_value = "easy")]
    Easy,
    #[sea_orm(string_value = "medium")]
    Medium,
    #[sea_orm(string_value = "hard")]
    Hard,
    #[sea_orm(string_value = "advanced")]
    Advanced,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub level_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(100))", unique)]
    pub slug: String,
    #[sea_orm(column_type = "String(StringLen::N(200))")]
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub description_md: String,
    #[sea_orm(column_type = "Text")]
    pub starter_code: String,
    #[sea_orm(column_type = "Text")]
    pub solution_code: String,
    #[sea_orm(column_type = "Text")]
    pub test_code: String,
    #[sea_orm(column_type = "Text")]
    pub cargo_toml: String,
    pub difficulty: Difficulty,
    pub xp_reward: i32,
    pub order_index: i32,
    #[sea_orm(default_value = "false")]
    pub is_published: bool,
    #[sea_orm(column_type = "Array(RcOrArc::new(ColumnType::Text))")]
    pub tags: Vec<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub hint_md: Option<String>,
    #[sea_orm(column_type = "Array(RcOrArc::new(ColumnType::Text))")]
    pub syntest_rules: Vec<String>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::levels::Entity",
        from = "Column::LevelId",
        to = "super::levels::Column::Id"
    )]
    Level,
}

impl Related<super::levels::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Level.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
