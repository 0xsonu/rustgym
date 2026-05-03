pub mod level_detail;
pub mod login;
pub mod quest_detail;
pub mod quest_list;
pub mod submit;
pub mod task_detail;

use ratatui::prelude::{Frame, Rect};

use crate::action::Action;
use crate::app::AppState;
use crate::event::AppEvent;

/// Every screen implements this trait.
pub trait ScreenHandler {
    /// Process an event, return an action to execute.
    fn update(&mut self, event: &AppEvent, app_state: &AppState) -> Action;

    /// Render the screen into the given frame area.
    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState);

    /// Return key hints for the status bar.
    fn key_hints(&self) -> Vec<(&str, &str)>;
}

// ---------------------------------------------------------------------------
// Re-export screen types from their modules.
// ---------------------------------------------------------------------------

pub use level_detail::LevelDetailScreen;
pub use login::LoginScreen;
pub use quest_detail::QuestDetailScreen;
pub use quest_list::QuestListScreen;
pub use submit::SubmitScreen;
pub use task_detail::TaskDetailScreen;

// ---------------------------------------------------------------------------
// Screen enum — wraps all screen variants and delegates to inner handlers
// ---------------------------------------------------------------------------

/// Enum wrapping all screen variants for dynamic dispatch via delegation.
pub enum Screen {
    Login(LoginScreen),
    QuestList(QuestListScreen),
    QuestDetail(QuestDetailScreen),
    LevelDetail(LevelDetailScreen),
    TaskDetail(TaskDetailScreen),
    Submit(SubmitScreen),
}

impl ScreenHandler for Screen {
    fn update(&mut self, event: &AppEvent, app_state: &AppState) -> Action {
        match self {
            Screen::Login(s) => s.update(event, app_state),
            Screen::QuestList(s) => s.update(event, app_state),
            Screen::QuestDetail(s) => s.update(event, app_state),
            Screen::LevelDetail(s) => s.update(event, app_state),
            Screen::TaskDetail(s) => s.update(event, app_state),
            Screen::Submit(s) => s.update(event, app_state),
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        match self {
            Screen::Login(s) => s.render(frame, area, app_state),
            Screen::QuestList(s) => s.render(frame, area, app_state),
            Screen::QuestDetail(s) => s.render(frame, area, app_state),
            Screen::LevelDetail(s) => s.render(frame, area, app_state),
            Screen::TaskDetail(s) => s.render(frame, area, app_state),
            Screen::Submit(s) => s.render(frame, area, app_state),
        }
    }

    fn key_hints(&self) -> Vec<(&str, &str)> {
        match self {
            Screen::Login(s) => s.key_hints(),
            Screen::QuestList(s) => s.key_hints(),
            Screen::QuestDetail(s) => s.key_hints(),
            Screen::LevelDetail(s) => s.key_hints(),
            Screen::TaskDetail(s) => s.key_hints(),
            Screen::Submit(s) => s.key_hints(),
        }
    }
}
