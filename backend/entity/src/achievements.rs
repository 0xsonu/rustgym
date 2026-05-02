use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "achievement_condition_type"
)]
pub enum ConditionType {
    #[sea_orm(string_value = "tasks_completed")]
    TasksCompleted,
    #[sea_orm(string_value = "xp_earned")]
    XpEarned,
    #[sea_orm(string_value = "streak")]
    Streak,
    #[sea_orm(string_value = "quest_done")]
    QuestDone,
    #[sea_orm(string_value = "first_submit")]
    FirstSubmit,
    #[sea_orm(string_value = "first_pass")]
    FirstPass,
    #[sea_orm(string_value = "speed_pass")]
    SpeedPass,
    #[sea_orm(string_value = "forum_accepted")]
    ForumAccepted,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "achievements")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(100))", unique)]
    pub slug: String,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub icon: Option<String>,
    #[sea_orm(default_value = "0")]
    pub xp_reward: i32,
    pub condition_type: ConditionType,
    pub condition_value: i32,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub condition_meta: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_achievements::Entity")]
    UserAchievements,
}

impl Related<super::user_achievements::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserAchievements.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
