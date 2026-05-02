pub use sea_orm_migration::prelude::*;

mod m20240101_000001_create_users_table;
mod m20240101_000002_create_refresh_tokens_table;
mod m20240101_000003_create_quests_table;
mod m20240101_000004_create_levels_table;
mod m20240101_000005_create_tasks_table;
mod m20240101_000006_create_progress_tables;
mod m20240101_000007_create_submissions_table;
mod m20240101_000008_create_achievements_table;
mod m20240101_000009_create_user_achievements_table;
mod m20240101_000010_seed_achievements;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_create_users_table::Migration),
            Box::new(m20240101_000002_create_refresh_tokens_table::Migration),
            Box::new(m20240101_000003_create_quests_table::Migration),
            Box::new(m20240101_000004_create_levels_table::Migration),
            Box::new(m20240101_000005_create_tasks_table::Migration),
            Box::new(m20240101_000006_create_progress_tables::Migration),
            Box::new(m20240101_000007_create_submissions_table::Migration),
            Box::new(m20240101_000008_create_achievements_table::Migration),
            Box::new(m20240101_000009_create_user_achievements_table::Migration),
            Box::new(m20240101_000010_seed_achievements::Migration),
        ]
    }
}
