mod action;
mod api;
mod app;
mod config;
mod editor;
mod event;
mod screens;
mod widgets;

use std::io::{self, stdout, Stdout};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc;

use action::{Action, ApiCall, ScreenId};
use api::ApiClient;
use app::App;
use config::CliConfig;
use event::{ApiResult, EventStream};
use screens::{
    LevelDetailScreen, LoginScreen, QuestDetailScreen, QuestListScreen, Screen, SubmitScreen,
    TaskDetailScreen,
};

/// Default API URL when none is configured.
const DEFAULT_API_URL: &str = "https://rustgym.dev";

/// Package version from Cargo.toml.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Parsed CLI arguments.
struct CliArgs {
    api_url: Option<String>,
}

/// Parse CLI arguments manually (no Clap dependency).
fn parse_args() -> CliArgs {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut api_url: Option<String> = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                println!("rustgym-cli {}", VERSION);
                println!();
                println!("USAGE:");
                println!("    rustgym [OPTIONS]");
                println!();
                println!("OPTIONS:");
                println!("    --api-url <URL>    API base URL (default: {})", DEFAULT_API_URL);
                println!("    --help, -h         Print help information");
                println!("    --version, -V      Print version information");
                std::process::exit(0);
            }
            "--version" | "-V" => {
                println!("rustgym-cli {}", VERSION);
                std::process::exit(0);
            }
            "--api-url" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("Error: --api-url requires a value");
                    std::process::exit(1);
                }
                api_url = Some(args[i].clone());
            }
            other => {
                eprintln!("Error: unknown argument '{}'", other);
                eprintln!("Run with --help for usage information.");
                std::process::exit(1);
            }
        }
        i += 1;
    }

    CliArgs { api_url }
}

/// Resolve the API URL from CLI args, config file, or default.
fn resolve_api_url(cli_url: Option<String>) -> String {
    let config = CliConfig::load().unwrap_or_default();
    config.resolve_api_url(cli_url.as_deref())
}

/// Install a panic hook that restores the terminal before printing the error.
fn install_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        // Best-effort terminal restoration.
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        // Delegate to the original hook for the actual panic message.
        original_hook(info);
    }));
}

/// Set up the terminal for TUI rendering.
fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

/// Restore the terminal to its original state.
fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

/// Create a Screen variant from a ScreenId.
fn create_screen(screen_id: &ScreenId) -> Screen {
    match screen_id {
        ScreenId::Login => Screen::Login(LoginScreen::new()),
        ScreenId::QuestList => Screen::QuestList(QuestListScreen::new()),
        ScreenId::QuestDetail { slug } => {
            Screen::QuestDetail(QuestDetailScreen::new(slug.clone()))
        }
        ScreenId::LevelDetail {
            quest_slug,
            level_slug,
        } => Screen::LevelDetail(LevelDetailScreen::new(
            quest_slug.clone(),
            level_slug.clone(),
        )),
        ScreenId::TaskDetail { slug } => Screen::TaskDetail(TaskDetailScreen::new(slug.clone())),
        ScreenId::Submit { slug } => Screen::Submit(SubmitScreen::new(slug.clone())),
    }
}

/// Execute a single action returned by the app's update method.
fn execute_action(
    app: &mut App,
    action: Action,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    api_client: &mut ApiClient,
) -> bool {
    match action {
        Action::None => {}
        Action::Push(screen_id) => {
            // Clear error message on navigation.
            app.state.error_message = None;
            let screen = create_screen(&screen_id);
            app.state.nav_stack.push(screen_id);
            app.screen_stack.push(screen);
        }
        Action::Pop => {
            // Clear error message on navigation.
            app.state.error_message = None;
            if app.state.nav_stack.len() > 1 {
                // Check if we're popping from a Submit screen — if so, force
                // the parent screens to refetch so progress updates are visible.
                let was_submit = matches!(
                    app.state.nav_stack.last(),
                    Some(ScreenId::Submit { .. })
                );

                app.state.nav_stack.pop();
                app.screen_stack.pop();

                // After popping from submit, reset fetched flags on remaining screens
                // so they re-query the API for updated progress.
                if was_submit {
                    for screen in app.screen_stack.iter_mut() {
                        match screen {
                            Screen::QuestList(s) => s.fetched = false,
                            Screen::QuestDetail(s) => s.fetched = false,
                            Screen::LevelDetail(s) => s.fetched = false,
                            _ => {}
                        }
                    }
                }
            }
        }
        Action::Replace(screen_id) => {
            if !app.state.nav_stack.is_empty() {
                app.state.nav_stack.pop();
                app.screen_stack.pop();
            }
            let screen = create_screen(&screen_id);
            app.state.nav_stack.push(screen_id);
            app.screen_stack.push(screen);
        }
        Action::Quit => {
            return true;
        }
        Action::SetAuth { token, username, xp } => {
            // Update app auth state.
            app.state.auth = app::AuthState::LoggedIn {
                token: token.clone(),
                username,
                xp,
            };
            // Update API client token.
            api_client.set_token(Some(token.clone()));
            // Persist token to config.
            if let Ok(mut cfg) = CliConfig::load() {
                cfg.token = Some(token);
                let _ = cfg.save();
            }
        }
        Action::Logout => {
            // Clear auth state.
            app.state.auth = app::AuthState::LoggedOut;
            // Clear API client token.
            api_client.set_token(None);
            // Clear token from config.
            if let Ok(mut cfg) = CliConfig::load() {
                let _ = cfg.clear_token();
            }
            // Navigate to login screen.
            app.state.error_message = None;
            // Clear screen stack and nav stack, start fresh at login.
            app.screen_stack.clear();
            app.state.nav_stack.clear();
            let screen = create_screen(&ScreenId::Login);
            app.state.nav_stack.push(ScreenId::Login);
            app.screen_stack.push(screen);
        }
        Action::ApiRequest(call) => {
            app.state.last_api_request = Some(call.clone());
            app.state.loading = true;
            match call {
                ApiCall::Login { email, password } => api_client.login(email, password),
                ApiCall::FetchQuests => api_client.fetch_quests(),
                ApiCall::FetchQuestDetail { slug } => api_client.fetch_quest_detail(slug),
                ApiCall::FetchLevelDetail {
                    quest_slug,
                    level_slug,
                } => api_client.fetch_level_detail(quest_slug, level_slug),
                ApiCall::FetchTaskDetail { slug } => api_client.fetch_task_detail(slug),
                ApiCall::SubmitSolution { slug, code } => api_client.submit_solution(slug, code),
            }
        }
        Action::LaunchEditor { slug, starter_code, description_md } => {
            // Ensure the challenge file exists with starter code before opening.
            match editor::ensure_challenge_file(&slug, &starter_code, &description_md) {
                Ok(file_path) => {
                    match editor::open(terminal, &file_path, &slug) {
                        Ok(()) => {}
                        Err(e) => {
                            app.state.error_message = Some(e.user_message());
                        }
                    }
                }
                Err(e) => {
                    app.state.error_message = Some(e.user_message());
                }
            }
        }
        Action::ShowError(msg) => {
            app.state.error_message = Some(msg);
        }
        Action::Batch(actions) => {
            for a in actions {
                if execute_action(app, a, terminal, api_client) {
                    return true;
                }
            }
        }
    }
    false
}

#[tokio::main]
async fn main() {
    // Parse CLI arguments.
    let cli_args = parse_args();

    // Persist --api-url to config if provided.
    if let Some(ref url) = cli_args.api_url {
        if let Ok(mut cfg) = CliConfig::load() {
            cfg.api_url = Some(url.clone());
            let _ = cfg.save();
        }
    }

    let _api_url = resolve_api_url(cli_args.api_url);

    // Install panic hook before terminal setup.
    install_panic_hook();

    // Set up terminal.
    let mut terminal = match setup_terminal() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to initialize terminal: {}", e);
            std::process::exit(1);
        }
    };

    // Create tokio mpsc channel for API responses.
    let (api_tx, api_rx) = mpsc::unbounded_channel::<ApiResult>();

    // Create API client.
    let mut api_client = ApiClient::new(_api_url.clone(), None, api_tx.clone());

    // Create event stream.
    let mut event_stream = EventStream::new(api_rx);

    // Create application — check for existing token to skip login.
    let mut app = App::new();

    // If a token exists in config, start on the quest list instead of login.
    if let Ok(cfg) = CliConfig::load() {
        if let Some(ref token) = cfg.token {
            if !token.is_empty() {
                // Set auth state and API client token.
                app.state.auth = app::AuthState::LoggedIn {
                    token: token.clone(),
                    username: String::new(), // Will be populated on next API call
                    xp: 0,                   // Will be updated when user data is fetched
                };
                api_client.set_token(Some(token.clone()));

                // Replace login screen with quest list.
                app.screen_stack.clear();
                app.state.nav_stack.clear();
                app.screen_stack.push(Screen::QuestList(QuestListScreen::new()));
                app.state.nav_stack.push(ScreenId::QuestList);
            }
        }
    }

    // Main event loop.
    loop {
        // Render current state.
        if let Err(e) = terminal.draw(|frame| app.render(frame)) {
            eprintln!("Render error: {}", e);
            break;
        }

        // Wait for next event.
        let event = event_stream.next().await;

        // Update app state and get action.
        let action = app.update(event);

        // Execute the action.
        let should_quit = execute_action(&mut app, action, &mut terminal, &mut api_client);
        if should_quit {
            break;
        }
    }

    // Restore terminal on exit.
    if let Err(e) = restore_terminal(&mut terminal) {
        eprintln!("Failed to restore terminal: {}", e);
    }
}
