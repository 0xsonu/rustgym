use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String,
    #[sea_orm(nullable)]
    pub avatar_url: Option<String>,
    #[sea_orm(nullable)]
    pub bio: Option<String>,
    pub role: Role,
    #[sea_orm(default_value = "0")]
    pub xp: i32,
    #[sea_orm(default_value = "1")]
    pub level: i32,
    #[sea_orm(default_value = "0")]
    pub streak_days: i32,
    #[sea_orm(nullable)]
    pub last_active_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[sea_orm(default_value = "false")]
    pub is_verified: bool,
    #[sea_orm(default_value = "false")]
    pub is_banned: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_role")]
pub enum Role {
    #[sea_orm(string_value = "student")]
    Student,
    #[sea_orm(string_value = "mentor")]
    Mentor,
    #[sea_orm(string_value = "admin")]
    Admin,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::refresh_tokens::Entity")]
    RefreshTokens,
}

impl Related<super::refresh_tokens::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefreshTokens.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
