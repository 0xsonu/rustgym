//! Curriculum seeder binary.
//!
//! Reads `challenges/manifest.json` and upserts quests, levels, and tasks
//! into the database. For each task, it also attempts to load challenge files
//! from the filesystem.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use serde::Deserialize;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use uuid::Uuid;

use entity::{levels, quests, tasks};

// ─── Manifest Schema ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct Manifest {
    quests: Vec<ManifestQuest>,
}

#[derive(Debug, Deserialize)]
struct ManifestQuest {
    slug: String,
    title: String,
    description: String,
    order_index: i32,
    icon: String,
    color: String,
    prerequisite_quest_slug: Option<String>,
    levels: Vec<ManifestLevel>,
}

#[derive(Debug, Deserialize)]
struct ManifestLevel {
    slug: String,
    title: String,
    description: String,
    order_index: i32,
    tasks: Vec<ManifestTask>,
}

#[derive(Debug, Deserialize)]
struct ManifestTask {
    slug: String,
    title: String,
    difficulty: String,
    xp_reward: i32,
    order_index: i32,
    tags: Vec<String>,
}

// ─── Main ───────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url =
        std::env::var("DATABASE_URL").context("DATABASE_URL environment variable must be set")?;

    info!("Connecting to database...");
    let db = Database::connect(&database_url)
        .await
        .context("Failed to connect to database")?;
    info!("Connected to database");

    // Determine the challenges directory path relative to the workspace root.
    // The binary runs from `backend/`, so challenges is at `../challenges/`.
    let challenges_dir = find_challenges_dir()?;
    let manifest_path = challenges_dir.join("manifest.json");

    info!("Loading manifest from: {}", manifest_path.display());
    let manifest_content =
        std::fs::read_to_string(&manifest_path).context("Failed to read manifest.json")?;
    let manifest: Manifest =
        serde_json::from_str(&manifest_content).context("Failed to parse manifest.json")?;

    info!("Manifest loaded: {} quests", manifest.quests.len());

    seed_quests(&db, &manifest, &challenges_dir).await?;

    info!("Seeding complete!");
    Ok(())
}

// ─── Seeding Logic ──────────────────────────────────────────────────────────

async fn seed_quests(
    db: &DatabaseConnection,
    manifest: &Manifest,
    challenges_dir: &Path,
) -> Result<()> {
    // First pass: upsert all quests (without prerequisite_quest_id)
    for quest_manifest in &manifest.quests {
        upsert_quest(db, quest_manifest).await?;
    }

    // Second pass: set prerequisite_quest_id references
    for quest_manifest in &manifest.quests {
        if let Some(ref prereq_slug) = quest_manifest.prerequisite_quest_slug {
            set_quest_prerequisite(db, &quest_manifest.slug, prereq_slug).await?;
        }
    }

    // Third pass: upsert levels and tasks
    for quest_manifest in &manifest.quests {
        let quest = quests::Entity::find()
            .filter(quests::Column::Slug.eq(&quest_manifest.slug))
            .one(db)
            .await?
            .context(format!(
                "Quest '{}' not found after upsert",
                quest_manifest.slug
            ))?;

        for level_manifest in &quest_manifest.levels {
            let level_id = upsert_level(db, &quest.id, level_manifest).await?;

            for task_manifest in &level_manifest.tasks {
                upsert_task(
                    db,
                    &level_id,
                    task_manifest,
                    challenges_dir,
                    &quest_manifest.slug,
                    &level_manifest.slug,
                )
                .await?;
            }
        }
    }

    Ok(())
}

async fn upsert_quest(db: &DatabaseConnection, quest: &ManifestQuest) -> Result<()> {
    let now = Utc::now().fixed_offset();

    let existing = quests::Entity::find()
        .filter(quests::Column::Slug.eq(&quest.slug))
        .one(db)
        .await?;

    if let Some(existing) = existing {
        // Update existing quest
        let mut active: quests::ActiveModel = existing.into();
        active.title = Set(quest.title.clone());
        active.description = Set(Some(quest.description.clone()));
        active.order_index = Set(quest.order_index);
        active.icon = Set(Some(quest.icon.clone()));
        active.color = Set(Some(quest.color.clone()));
        active.updated_at = Set(now);
        active.update(db).await?;
        info!("Updated quest: {}", quest.slug);
    } else {
        // Insert new quest
        let active = quests::ActiveModel {
            id: Set(Uuid::new_v4()),
            slug: Set(quest.slug.clone()),
            title: Set(quest.title.clone()),
            description: Set(Some(quest.description.clone())),
            order_index: Set(quest.order_index),
            icon: Set(Some(quest.icon.clone())),
            color: Set(Some(quest.color.clone())),
            is_published: Set(true),
            prerequisite_quest_id: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };
        active.insert(db).await?;
        info!("Inserted quest: {}", quest.slug);
    }

    Ok(())
}

async fn set_quest_prerequisite(
    db: &DatabaseConnection,
    quest_slug: &str,
    prereq_slug: &str,
) -> Result<()> {
    let prereq = quests::Entity::find()
        .filter(quests::Column::Slug.eq(prereq_slug))
        .one(db)
        .await?
        .context(format!("Prerequisite quest '{}' not found", prereq_slug))?;

    let quest = quests::Entity::find()
        .filter(quests::Column::Slug.eq(quest_slug))
        .one(db)
        .await?
        .context(format!("Quest '{}' not found", quest_slug))?;

    let mut active: quests::ActiveModel = quest.into();
    active.prerequisite_quest_id = Set(Some(prereq.id));
    active.update(db).await?;

    Ok(())
}

async fn upsert_level(
    db: &DatabaseConnection,
    quest_id: &Uuid,
    level: &ManifestLevel,
) -> Result<Uuid> {
    let now = Utc::now().fixed_offset();

    let existing = levels::Entity::find()
        .filter(levels::Column::Slug.eq(&level.slug))
        .one(db)
        .await?;

    if let Some(existing) = existing {
        let id = existing.id;
        let mut active: levels::ActiveModel = existing.into();
        active.title = Set(level.title.clone());
        active.description = Set(Some(level.description.clone()));
        active.order_index = Set(level.order_index);
        active.quest_id = Set(*quest_id);
        active.updated_at = Set(now);
        active.update(db).await?;
        info!("  Updated level: {}", level.slug);
        Ok(id)
    } else {
        let id = Uuid::new_v4();
        let active = levels::ActiveModel {
            id: Set(id),
            quest_id: Set(*quest_id),
            slug: Set(level.slug.clone()),
            title: Set(level.title.clone()),
            description: Set(Some(level.description.clone())),
            order_index: Set(level.order_index),
            is_published: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
        };
        active.insert(db).await?;
        info!("  Inserted level: {}", level.slug);
        Ok(id)
    }
}

async fn upsert_task(
    db: &DatabaseConnection,
    level_id: &Uuid,
    task: &ManifestTask,
    challenges_dir: &Path,
    quest_slug: &str,
    level_slug: &str,
) -> Result<()> {
    let now = Utc::now().fixed_offset();

    // Try to load challenge files from the filesystem
    let task_dir = challenges_dir
        .join(quest_slug)
        .join(level_slug)
        .join(&task.slug);

    let description_md = read_file_or_placeholder(&task_dir.join("description.md"), &task.title);
    let starter_code = read_file_or_placeholder(
        &task_dir.join("src/starter.rs"),
        "// Write your solution here\n",
    );
    let solution_code = read_file_or_placeholder(
        &task_dir.join("src/lib.rs"),
        "// Solution not yet available\n",
    );
    let test_code = read_file_or_placeholder(
        &task_dir.join("tests/tests.rs"),
        "#[test]\nfn test_placeholder() {\n    // TODO: Add tests\n    assert!(true);\n}\n",
    );
    let cargo_toml = read_file_or_placeholder(
        &task_dir.join("Cargo.toml"),
        &default_cargo_toml(&task.slug),
    );

    let difficulty = parse_difficulty(&task.difficulty);

    let existing = tasks::Entity::find()
        .filter(tasks::Column::Slug.eq(&task.slug))
        .one(db)
        .await?;

    if let Some(existing) = existing {
        let mut active: tasks::ActiveModel = existing.into();
        active.title = Set(task.title.clone());
        active.description_md = Set(description_md);
        active.starter_code = Set(starter_code);
        active.solution_code = Set(solution_code);
        active.test_code = Set(test_code);
        active.cargo_toml = Set(cargo_toml);
        active.difficulty = Set(difficulty);
        active.xp_reward = Set(task.xp_reward);
        active.order_index = Set(task.order_index);
        active.tags = Set(task.tags.clone());
        active.level_id = Set(*level_id);
        active.updated_at = Set(now);
        active.update(db).await?;
        info!("    Updated task: {}", task.slug);
    } else {
        let active = tasks::ActiveModel {
            id: Set(Uuid::new_v4()),
            level_id: Set(*level_id),
            slug: Set(task.slug.clone()),
            title: Set(task.title.clone()),
            description_md: Set(description_md),
            starter_code: Set(starter_code),
            solution_code: Set(solution_code),
            test_code: Set(test_code),
            cargo_toml: Set(cargo_toml),
            difficulty: Set(difficulty),
            xp_reward: Set(task.xp_reward),
            order_index: Set(task.order_index),
            is_published: Set(true),
            tags: Set(task.tags.clone()),
            hint_md: Set(None),
            syntest_rules: Set(vec![]),
            created_at: Set(now),
            updated_at: Set(now),
        };
        active.insert(db).await?;
        info!("    Inserted task: {}", task.slug);
    }

    Ok(())
}

// ─── Helpers ────────────────────────────────────────────────────────────────

fn find_challenges_dir() -> Result<PathBuf> {
    // Try relative paths from common working directories
    let candidates = [
        PathBuf::from("../challenges"), // running from backend/
        PathBuf::from("challenges"),    // running from project root
        PathBuf::from("./challenges"),  // explicit current dir
    ];

    for candidate in &candidates {
        if candidate.exists() && candidate.is_dir() {
            return Ok(candidate.clone());
        }
    }

    // Fall back to looking for manifest.json specifically
    for candidate in &candidates {
        let manifest = candidate.join("manifest.json");
        if manifest.exists() {
            return Ok(candidate.clone());
        }
    }

    // Default to ../challenges (the expected path when running from backend/)
    Ok(PathBuf::from("../challenges"))
}

fn read_file_or_placeholder(path: &Path, placeholder: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => {
            warn!("File not found: {}, using placeholder", path.display());
            placeholder.to_string()
        }
    }
}

fn parse_difficulty(s: &str) -> tasks::Difficulty {
    match s.to_lowercase().as_str() {
        "beginner" => tasks::Difficulty::Beginner,
        "easy" => tasks::Difficulty::Easy,
        "medium" => tasks::Difficulty::Medium,
        "hard" => tasks::Difficulty::Hard,
        "advanced" => tasks::Difficulty::Advanced,
        _ => {
            warn!("Unknown difficulty '{}', defaulting to Medium", s);
            tasks::Difficulty::Medium
        }
    }
}

fn default_cargo_toml(task_slug: &str) -> String {
    format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
"#,
        task_slug
    )
}
