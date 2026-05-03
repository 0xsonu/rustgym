use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::Frame;

use crate::action::{Action, ApiCall, ScreenId};
use crate::api::error::AppError;
use crate::config::CliConfig;
use crate::event::AppEvent;
use crate::screens::{LoginScreen, Screen, ScreenHandler};
use crate::widgets::StatusBar;

/// Authentication state of the application.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AuthState {
    /// Token status not yet determined (e.g., checking config on startup).
    Unknown,
    /// No valid token available.
    LoggedOut,
    /// Authenticated with a valid token.
    LoggedIn { token: String, username: String, xp: i32 },
}

/// Summary data for a quest as returned by the API list endpoint.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct QuestSummary {
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub level_count: i64,
    pub task_count: i64,
}

/// Detail data for a single quest (levels within it).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct QuestDetailData {
    pub slug: String,
    pub title: String,
    pub levels: Vec<serde_json::Value>,
}

/// Detail data for a single level (tasks within it).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LevelDetailData {
    pub slug: String,
    pub title: String,
    pub tasks: Vec<serde_json::Value>,
}

/// Detail data for a single task.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TaskDetailData {
    pub slug: String,
    pub title: String,
    pub description_md: String,
    pub starter_code: String,
    pub difficulty: String,
    pub xp_reward: i32,
    pub hint_md: Option<String>,
}

/// Result of a code submission.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SubmissionResult {
    pub status: String,
    pub test_results: serde_json::Value,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub duration_ms: i32,
    pub xp_awarded: i32,
    pub attempt_number: i32,
    pub leveled_up: bool,
    pub new_level: Option<i32>,
}

/// Cached API data to avoid redundant fetches.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DataCache {
    pub quests: Option<Vec<QuestSummary>>,
    pub quest_details: HashMap<String, QuestDetailData>,
    pub level_details: HashMap<String, LevelDetailData>,
    pub task_details: HashMap<String, TaskDetailData>,
    pub last_submission: Option<SubmissionResult>,
}

impl DataCache {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self {
            quests: None,
            quest_details: HashMap::new(),
            level_details: HashMap::new(),
            task_details: HashMap::new(),
            last_submission: None,
        }
    }
}

impl Default for DataCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Global application state shared across all screens.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Navigation stack of screen identifiers.
    pub nav_stack: Vec<ScreenId>,
    /// Authentication state.
    pub auth: AuthState,
    /// Cached API data.
    pub cache: DataCache,
    /// Global loading flag (true while an API request is in-flight).
    pub loading: bool,
    /// Transient error message displayed to the user.
    pub error_message: Option<String>,
    /// Last API request for retry on `r` key press.
    pub last_api_request: Option<ApiCall>,
    /// Tick counter for animations.
    pub tick: u64,
    /// Terminal dimensions (columns, rows).
    pub terminal_size: (u16, u16),
}

impl AppState {
    /// Create a new AppState with sensible defaults.
    pub fn new() -> Self {
        Self {
            nav_stack: vec![ScreenId::Login],
            auth: AuthState::Unknown,
            cache: DataCache::new(),
            loading: false,
            error_message: None,
            last_api_request: None,
            tick: 0,
            terminal_size: (80, 24),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// The top-level application that owns state and dispatches to screens.
pub struct App {
    pub state: AppState,
    pub screen_stack: Vec<Screen>,
}

impl App {
    /// Create a new App with default state.
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
            screen_stack: vec![Screen::Login(LoginScreen::new())],
        }
    }

    /// Process an event and return an action to execute.
    ///
    /// Dispatches to the current screen's update method based on the top of
    /// the navigation stack. Global error handling intercepts Unauthorized
    /// errors before they reach individual screens.
    pub fn update(&mut self, event: AppEvent) -> Action {
        // Increment tick counter on tick events.
        if let AppEvent::Tick = &event {
            self.state.tick = self.state.tick.wrapping_add(1);
        }

        // Update terminal size on resize events.
        if let AppEvent::Resize(cols, rows) = &event {
            self.state.terminal_size = (*cols, *rows);
        }

        // Global error handling: intercept API error responses.
        if let AppEvent::ApiResponse(Err(ref err)) = event {
            self.state.loading = false;
            match err {
                AppError::Unauthorized => {
                    // Clear token from config (best-effort).
                    if let Ok(mut config) = CliConfig::load() {
                        let _ = config.clear_token();
                    }
                    // Set auth state to LoggedOut.
                    self.state.auth = AuthState::LoggedOut;
                    // Set error message.
                    self.state.error_message =
                        Some("Session expired — please log in again".to_string());
                    // Navigate to Login screen.
                    return Action::Replace(ScreenId::Login);
                }
                other => {
                    // Set error message for display; let screen handle it too.
                    self.state.error_message = Some(other.user_message());
                }
            }
        }

        // Clear loading on successful API response.
        if let AppEvent::ApiResponse(Ok(_)) = &event {
            self.state.loading = false;
        }

        // Global Ctrl+C: always quit gracefully.
        if let AppEvent::Input(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers,
            ..
        }) = &event
        {
            if modifiers.contains(KeyModifiers::CONTROL) {
                return Action::Quit;
            }
        }

        // Clear error message on any user key input (except Ctrl+C which already handled above).
        // This ensures transient errors don't persist and block the UI.
        if let AppEvent::Input(_) = &event {
            if self.state.error_message.is_some() {
                self.state.error_message = None;
            }
        }

        // Dispatch to the current screen handler.
        if let Some(screen) = self.screen_stack.last_mut() {
            screen.update(&event, &self.state)
        } else {
            Action::None
        }
    }

    /// Render the current screen to the terminal frame.
    ///
    /// Splits the frame into a content area and a status bar at the bottom,
    /// then dispatches rendering to the current screen.
    pub fn render(&self, frame: &mut Frame) {
        let size = frame.area();

        // Split into content area + 1-line status bar at the bottom.
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(size);

        let content_area = layout[0];
        let status_area = layout[1];

        // Render the current screen into the content area.
        if let Some(screen) = self.screen_stack.last() {
            screen.render(frame, content_area, &self.state);

            // Render the StatusBar widget in the bottom line.
            let (username, xp) = match &self.state.auth {
                AuthState::LoggedIn { username, xp, .. } => (username.as_str(), *xp),
                _ => ("", 0i32),
            };

            let screen_name = self.current_screen_name();
            let key_hints = screen.key_hints();

            let status_bar = StatusBar::new(screen_name, username, xp, key_hints);
            frame.render_widget(status_bar, status_area);
        }
    }

    /// Get a human-readable name for the current screen.
    fn current_screen_name(&self) -> &str {
        match self.state.nav_stack.last() {
            Some(ScreenId::Login) => "Login",
            Some(ScreenId::QuestList) => "Quests",
            Some(ScreenId::QuestDetail { .. }) => "Quest",
            Some(ScreenId::LevelDetail { .. }) => "Level",
            Some(ScreenId::TaskDetail { .. }) => "Task",
            Some(ScreenId::Submit { .. }) => "Submit",
            None => "RustGym",
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
