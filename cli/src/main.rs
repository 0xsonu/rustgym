use std::path::PathBuf;

use clap::{Parser, Subcommand};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};

mod config;

use config::CliConfig;

/// RustGym CLI — interact with the RustGym platform from your terminal.
#[derive(Parser)]
#[command(name = "rustgym", version, about, long_about = None)]
struct Cli {
    /// API base URL override (default: https://rustgym.dev)
    #[arg(long, global = true)]
    api_url: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Authenticate with the RustGym platform
    Login {
        /// Email address
        #[arg(short, long)]
        email: String,
        /// Password
        #[arg(short, long)]
        password: String,
    },
    /// List available challenges
    List {
        /// Filter by quest slug
        #[arg(long)]
        quest: Option<String>,
        /// Filter by difficulty (beginner, intermediate, advanced)
        #[arg(long)]
        difficulty: Option<String>,
        /// Filter by completion status (completed, in-progress, not-started)
        #[arg(long)]
        status: Option<String>,
    },
    /// Download challenge files to ./challenges/<slug>/
    Download {
        /// Challenge slug to download
        slug: String,
    },
    /// Submit solution for a challenge
    Submit {
        /// Challenge slug to submit
        slug: String,
        /// Path to solution file (default: ./challenges/<slug>/src/lib.rs)
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
}

// --- API Response Types ---

#[derive(Debug, Deserialize)]
struct LoginResponse {
    access_token: String,
    #[allow(dead_code)]
    refresh_token: String,
}

#[derive(Debug, Deserialize)]
struct TaskListItem {
    slug: String,
    title: String,
    difficulty: String,
    #[serde(default)]
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TaskDetail {
    #[allow(dead_code)]
    slug: String,
    title: String,
    description: String,
    #[serde(default)]
    starter_code: Option<String>,
    #[serde(default)]
    test_code: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
struct SubmitRequest {
    code: String,
}

#[derive(Debug, Deserialize)]
struct SubmitResponse {
    passed: bool,
    #[serde(default)]
    total_tests: u32,
    #[serde(default)]
    passed_tests: u32,
    #[serde(default)]
    output: Option<String>,
    #[serde(default)]
    error: Option<String>,
}

// --- Main ---

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let api_url = cli
        .api_url
        .unwrap_or_else(|| "https://rustgym.dev".to_string());

    let result = match cli.command {
        Commands::Login { email, password } => cmd_login(&api_url, &email, &password).await,
        Commands::List {
            quest,
            difficulty,
            status,
        } => cmd_list(&api_url, quest, difficulty, status).await,
        Commands::Download { slug } => cmd_download(&api_url, &slug).await,
        Commands::Submit { slug, file } => cmd_submit(&api_url, &slug, file).await,
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}

// --- Command Implementations ---

async fn cmd_login(
    api_url: &str,
    email: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner().template("{spinner:.green} {msg}")?);
    spinner.set_message("Authenticating...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/api/v1/auth/login", api_url))
        .json(&serde_json::json!({
            "email": email,
            "password": password,
        }))
        .send()
        .await?;

    spinner.finish_and_clear();

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Login failed ({}): {}", status, body).into());
    }

    let login_resp: LoginResponse = resp.json().await?;

    // Store token in config
    let mut cfg = CliConfig::load()?;
    cfg.token = Some(login_resp.access_token);
    cfg.api_url = Some(api_url.to_string());
    cfg.save()?;

    println!("{} Logged in successfully!", "✓".green().bold());
    println!(
        "  Token stored in {}",
        CliConfig::config_path()?.display().to_string().dimmed()
    );

    Ok(())
}

async fn cmd_list(
    api_url: &str,
    quest: Option<String>,
    difficulty: Option<String>,
    status: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let cfg = CliConfig::load()?;
    let token = cfg
        .token
        .ok_or("Not logged in. Run `rustgym login` first.")?;

    let client = reqwest::Client::new();
    let mut url = format!("{}/api/v1/tasks", api_url);
    let mut params = Vec::new();

    if let Some(q) = &quest {
        params.push(format!("quest={}", q));
    }
    if let Some(d) = &difficulty {
        params.push(format!("difficulty={}", d));
    }
    if let Some(s) = &status {
        params.push(format!("status={}", s));
    }

    if !params.is_empty() {
        url = format!("{}?{}", url, params.join("&"));
    }

    let resp = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status_code = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Failed to list challenges ({}): {}", status_code, body).into());
    }

    let tasks: Vec<TaskListItem> = resp.json().await?;

    if tasks.is_empty() {
        println!("{}", "No challenges found matching your filters.".yellow());
        return Ok(());
    }

    println!(
        "\n{}\n",
        format!("  {} challenges found", tasks.len()).bold().white()
    );
    println!(
        "  {:<30} {:<15} {}",
        "SLUG".dimmed(),
        "DIFFICULTY".dimmed(),
        "TITLE".dimmed()
    );
    println!("  {}", "─".repeat(70).dimmed());

    for task in &tasks {
        let difficulty_colored = match task.difficulty.as_str() {
            "beginner" => task.difficulty.green(),
            "intermediate" => task.difficulty.yellow(),
            "advanced" => task.difficulty.red(),
            _ => task.difficulty.normal(),
        };

        let status_icon = match task.status.as_deref() {
            Some("completed") => "✓".green(),
            Some("in-progress") => "◐".yellow(),
            _ => "○".dimmed(),
        };

        println!(
            "  {} {:<28} {:<15} {}",
            status_icon, task.slug, difficulty_colored, task.title
        );
    }

    println!();
    Ok(())
}

async fn cmd_download(api_url: &str, slug: &str) -> Result<(), Box<dyn std::error::Error>> {
    let cfg = CliConfig::load()?;
    let token = cfg
        .token
        .ok_or("Not logged in. Run `rustgym login` first.")?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner().template("{spinner:.green} {msg}")?);
    spinner.set_message(format!("Downloading challenge '{}'...", slug));
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/api/v1/tasks/{}", api_url, slug))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    spinner.finish_and_clear();

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Failed to download challenge ({}): {}", status, body).into());
    }

    let task: TaskDetail = resp.json().await?;

    // Create challenge directory
    let challenge_dir = PathBuf::from("challenges").join(slug);
    let src_dir = challenge_dir.join("src");
    std::fs::create_dir_all(&src_dir)?;

    // Write description
    std::fs::write(
        challenge_dir.join("description.md"),
        format!("# {}\n\n{}", task.title, task.description),
    )?;

    // Write starter code
    let starter = task
        .starter_code
        .unwrap_or_else(|| "// Write your solution here\n".to_string());
    std::fs::write(src_dir.join("lib.rs"), &starter)?;

    // Write test code if available
    if let Some(tests) = &task.test_code {
        let tests_dir = challenge_dir.join("tests");
        std::fs::create_dir_all(&tests_dir)?;
        std::fs::write(tests_dir.join("tests.rs"), tests)?;
    }

    // Write Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name = "{slug}"
version = "0.1.0"
edition = "2021"
"#,
    );
    std::fs::write(challenge_dir.join("Cargo.toml"), cargo_toml)?;

    println!(
        "{} Challenge '{}' downloaded to ./challenges/{}/",
        "✓".green().bold(),
        task.title,
        slug
    );
    println!(
        "  Edit: {}",
        format!("challenges/{}/src/lib.rs", slug).cyan()
    );
    println!("  Submit: {}", format!("rustgym submit {}", slug).cyan());

    Ok(())
}

async fn cmd_submit(
    api_url: &str,
    slug: &str,
    file: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let cfg = CliConfig::load()?;
    let token = cfg
        .token
        .ok_or("Not logged in. Run `rustgym login` first.")?;

    // Read solution file
    let solution_path = file.unwrap_or_else(|| {
        PathBuf::from("challenges")
            .join(slug)
            .join("src")
            .join("lib.rs")
    });

    if !solution_path.exists() {
        return Err(format!(
            "Solution file not found: {}\nRun `rustgym download {}` first.",
            solution_path.display(),
            slug
        )
        .into());
    }

    let code = std::fs::read_to_string(&solution_path)?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner().template("{spinner:.green} {msg}")?);
    spinner.set_message("Submitting solution...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/api/v1/submissions", api_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "task_slug": slug,
            "code": code,
        }))
        .send()
        .await?;

    spinner.finish_and_clear();

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Submission failed ({}): {}", status, body).into());
    }

    let result: SubmitResponse = resp.json().await?;

    println!();
    if result.passed {
        println!(
            "  {} {}",
            "✓ All tests passed!".green().bold(),
            format!("({}/{})", result.passed_tests, result.total_tests).dimmed()
        );
    } else {
        println!(
            "  {} {}",
            "✗ Some tests failed.".red().bold(),
            format!("({}/{})", result.passed_tests, result.total_tests).dimmed()
        );
    }

    if let Some(output) = &result.output {
        if !output.is_empty() {
            println!("\n  {}", "Output:".bold());
            for line in output.lines() {
                println!("    {}", line);
            }
        }
    }

    if let Some(error) = &result.error {
        if !error.is_empty() {
            println!("\n  {}", "Errors:".red().bold());
            for line in error.lines() {
                println!("    {}", line.red());
            }
        }
    }

    println!();
    Ok(())
}
