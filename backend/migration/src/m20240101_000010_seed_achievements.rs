use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Seed 20 achievements
        let achievements = vec![
            (
                "first-blood",
                "First Blood",
                "Submit your first solution",
                25,
                "first_submit",
                1,
                None,
            ),
            (
                "rustacean-born",
                "Rustacean Born",
                "Pass your first task",
                50,
                "first_pass",
                1,
                None,
            ),
            (
                "ownership-master",
                "Ownership Master",
                "Complete the Ownership & Borrowing quest",
                200,
                "quest_done",
                1,
                Some(r#"{"quest_slug": "ownership-borrowing"}"#),
            ),
            (
                "ferris-approved",
                "Ferris Approved",
                "Complete all quests",
                1000,
                "quest_done",
                8,
                None,
            ),
            (
                "speed-demon",
                "Speed Demon",
                "Pass a task in under 5 minutes",
                75,
                "speed_pass",
                300,
                None,
            ),
            (
                "week-streak",
                "Week Streak",
                "Maintain a 7-day streak",
                100,
                "streak",
                7,
                None,
            ),
            (
                "month-streak",
                "Month Streak",
                "Maintain a 30-day streak",
                500,
                "streak",
                30,
                None,
            ),
            (
                "century",
                "Century",
                "Complete 100 tasks",
                300,
                "tasks_completed",
                100,
                None,
            ),
            (
                "helpful-crab",
                "Helpful Crab",
                "Have a forum answer accepted",
                50,
                "forum_accepted",
                1,
                None,
            ),
            (
                "perfectionist",
                "Perfectionist",
                "Pass a task on your first attempt",
                75,
                "first_pass",
                1,
                Some(r#"{"first_attempt": true}"#),
            ),
            (
                "ten-tasks",
                "Ten Tasks",
                "Complete 10 tasks",
                50,
                "tasks_completed",
                10,
                None,
            ),
            (
                "fifty-tasks",
                "Fifty Tasks",
                "Complete 50 tasks",
                150,
                "tasks_completed",
                50,
                None,
            ),
            (
                "xp-hunter-1k",
                "XP Hunter 1K",
                "Earn 1000 XP",
                50,
                "xp_earned",
                1000,
                None,
            ),
            (
                "xp-hunter-5k",
                "XP Hunter 5K",
                "Earn 5000 XP",
                100,
                "xp_earned",
                5000,
                None,
            ),
            (
                "xp-hunter-10k",
                "XP Hunter 10K",
                "Earn 10000 XP",
                200,
                "xp_earned",
                10000,
                None,
            ),
            (
                "foundations-master",
                "Foundations Master",
                "Complete the Rust Foundations quest",
                150,
                "quest_done",
                1,
                Some(r#"{"quest_slug": "rust-foundations"}"#),
            ),
            (
                "traits-master",
                "Traits Master",
                "Complete the Traits & Generics quest",
                200,
                "quest_done",
                1,
                Some(r#"{"quest_slug": "traits-generics"}"#),
            ),
            (
                "three-day-streak",
                "Three Day Streak",
                "Maintain a 3-day streak",
                50,
                "streak",
                3,
                None,
            ),
            (
                "two-week-streak",
                "Two Week Streak",
                "Maintain a 14-day streak",
                200,
                "streak",
                14,
                None,
            ),
            (
                "early-bird",
                "Early Bird",
                "Submit a solution before 8am",
                25,
                "first_submit",
                1,
                Some(r#"{"before_hour": 8}"#),
            ),
        ];

        for (slug, name, description, xp_reward, condition_type, condition_value, condition_meta) in
            achievements
        {
            let meta_clause = match condition_meta {
                Some(meta) => format!("'{}'::jsonb", meta),
                None => "NULL".to_string(),
            };

            let sql = format!(
                r#"INSERT INTO achievements (id, slug, name, description, icon, xp_reward, condition_type, condition_value, condition_meta, created_at, updated_at)
                VALUES (gen_random_uuid(), '{}', '{}', '{}', NULL, {}, '{}', {}, {}, NOW(), NOW())
                ON CONFLICT (slug) DO NOTHING"#,
                slug, name, description, xp_reward, condition_type, condition_value, meta_clause
            );

            db.execute_unprepared(&sql).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DELETE FROM achievements").await?;
        Ok(())
    }
}
