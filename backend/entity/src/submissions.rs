use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "submission_status")]
pub enum SubmissionStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "running")]
    Running,
    #[sea_orm(string_value = "passed")]
    Passed,
    #[sea_orm(string_value = "failed")]
    Failed,
    #[sea_orm(string_value = "error")]
    Error,
    #[sea_orm(string_value = "timeout")]
    Timeout,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "submissions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub task_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub code: String,
    pub status: SubmissionStatus,
    #[sea_orm(column_type = "JsonBinary")]
    pub test_results: serde_json::Value,
    #[sea_orm(column_type = "Text")]
    pub stdout: String,
    #[sea_orm(column_type = "Text")]
    pub stderr: String,
    pub duration_ms: i32,
    pub memory_kb: i32,
    pub xp_awarded: i32,
    pub attempt_number: i32,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
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

impl ActiveModelBehavior for ActiveModel {}
