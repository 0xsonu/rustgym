use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::{Constraint, Direction, Frame, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::action::{Action, ApiCall, ScreenId};
use crate::api::error::AppError;
use crate::api::{ApiResponse, QuestSummary};
use crate::app::{AppState, AuthState};
use crate::event::AppEvent;

use super::ScreenHandler;

// ─── ListScreenState ────────────────────────────────────────────────────────

/// Shared state for any list-based screen: tracks cursor position and scroll.
#[derive(Debug, Clone)]
pub struct ListScreenState {
    /// Currently selected item index.
    pub selected_index: usize,
    /// Scroll offset for the visible window.
    pub scroll_offset: usize,
    /// Total number of items in the list.
    pub items_count: usize,
}

impl ListScreenState {
    /// Create a new ListScreenState with zero items.
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            scroll_offset: 0,
            items_count: 0,
        }
    }

    /// Move the cursor down by one, clamped to the last item.
    pub fn move_down(&mut self) {
        if self.items_count == 0 {
            return;
        }
        if self.selected_index < self.items_count - 1 {
            self.selected_index += 1;
        }
    }

    /// Move the cursor up by one, clamped to the first item.
    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }
}

impl Default for ListScreenState {
    fn default() -> Self {
        Self::new()
    }
}

// ─── QuestListScreen ────────────────────────────────────────────────────────

/// Quest list screen showing available quests with progress.
#[derive(Debug, Clone)]
pub struct QuestListScreen {
    /// List navigation state.
    pub list_state: ListScreenState,
    /// Locally stored quest data (populated from API response).
    pub quests: Vec<QuestSummary>,
    /// Whether the initial fetch has been triggered.
    pub fetched: bool,
}

impl QuestListScreen {
    /// Create a new QuestListScreen.
    pub fn new() -> Self {
        Self {
            list_state: ListScreenState::new(),
            quests: Vec::new(),
            fetched: false,
        }
    }

    /// Handle keyboard input.
    fn handle_key_input(&mut self, key: &KeyEvent) -> Action {
        match key.code {
            KeyCode::Char('j') => {
                self.list_state.move_down();
                Action::None
            }
            KeyCode::Char('k') => {
                self.list_state.move_up();
                Action::None
            }
            KeyCode::Enter => {
                if !self.quests.is_empty() {
                    let slug = self.quests[self.list_state.selected_index].slug.clone();
                    Action::Push(ScreenId::QuestDetail { slug })
                } else {
                    Action::None
                }
            }
            KeyCode::Char('L') => {
                // Logout
                Action::Logout
            }
            KeyCode::Char('q')
                if !key.modifiers.contains(KeyModifiers::CONTROL)
                    && !key.modifiers.contains(KeyModifiers::ALT) =>
            {
                Action::Quit
            }
            _ => Action::None,
        }
    }

    /// Handle an API response relevant to the quest list.
    fn handle_api_response(&mut self, result: &crate::event::ApiResult) -> Action {
        match result {
            Ok(ApiResponse::QuestList { quests }) => {
                self.quests = quests.clone();
                self.list_state.items_count = self.quests.len();
                // Clamp selected_index if needed.
                if self.list_state.selected_index >= self.quests.len() && !self.quests.is_empty() {
                    self.list_state.selected_index = self.quests.len() - 1;
                }
                Action::None
            }
            Err(AppError::Unauthorized) => {
                // 401: clear token and redirect to login.
                Action::Batch(vec![
                    Action::ShowError("Session expired — please log in again".to_string()),
                    Action::Replace(ScreenId::Login),
                ])
            }
            Err(_) => {
                // Other errors are handled by the global error display.
                Action::None
            }
            // Ignore other API responses not meant for this screen.
            _ => Action::None,
        }
    }

    /// Check if we need to trigger the initial fetch.
    fn maybe_fetch(&mut self, app_state: &AppState) -> Action {
        if !self.fetched {
            self.fetched = true;
            // If cache already has quests, use them.
            if let Some(ref cached_quests) = app_state.cache.quests {
                // We can't directly use cached_quests (different type), but
                // we trigger a fetch anyway to get fresh data with progress.
                let _ = cached_quests;
            }
            Action::ApiRequest(ApiCall::FetchQuests)
        } else {
            Action::None
        }
    }
}

impl Default for QuestListScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenHandler for QuestListScreen {
    fn update(&mut self, event: &AppEvent, app_state: &AppState) -> Action {
        // On first update, trigger fetch if needed.
        let fetch_action = self.maybe_fetch(app_state);
        if !matches!(fetch_action, Action::None) {
            // If we need to fetch, check if this event also needs handling.
            let event_action = match event {
                AppEvent::Input(key_event) => self.handle_key_input(key_event),
                AppEvent::ApiResponse(result) => self.handle_api_response(result),
                _ => Action::None,
            };
            // Return fetch action first; event action will be processed next tick.
            return if matches!(event_action, Action::None) {
                fetch_action
            } else {
                Action::Batch(vec![fetch_action, event_action])
            };
        }

        match event {
            AppEvent::Input(key_event) => self.handle_key_input(key_event),
            AppEvent::ApiResponse(result) => self.handle_api_response(result),
            _ => Action::None,
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        // Main layout: header, list, status bar.
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(1),   // List area
                Constraint::Length(1), // Status bar
            ])
            .split(area);

        self.render_header(frame, layout[0], app_state);

        if app_state.loading && self.quests.is_empty() {
            self.render_loading(frame, layout[1], app_state);
        } else {
            self.render_quest_list(frame, layout[1]);
        }

        self.render_status_bar(frame, layout[2]);
    }

    fn key_hints(&self) -> Vec<(&str, &str)> {
        vec![
            ("j/k", "navigate"),
            ("Enter", "select"),
            ("L", "logout"),
            ("q", "quit"),
        ]
    }
}

// ─── Rendering ──────────────────────────────────────────────────────────────

impl QuestListScreen {
    /// Render the header with title and username.
    fn render_header(&self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        let username = match &app_state.auth {
            AuthState::LoggedIn { username, .. } => username.as_str(),
            _ => "guest",
        };

        let header_text = Line::from(vec![
            Span::styled(
                "RustGym — Quests",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                format!("[{}]", username),
                Style::default().fg(Color::Gray),
            ),
        ]);

        let header = Paragraph::new(header_text)
            .block(Block::default().borders(Borders::BOTTOM));
        frame.render_widget(header, area);
    }

    /// Render the loading spinner.
    fn render_loading(&self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        let idx = (app_state.tick as usize) % spinner_chars.len();
        let loading_text = format!("{} Loading quests...", spinner_chars[idx]);

        let loading = Paragraph::new(loading_text)
            .style(Style::default().fg(Color::Cyan))
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(loading, area);
    }

    /// Render the scrollable quest list.
    fn render_quest_list(&self, frame: &mut Frame, area: Rect) {
        if self.quests.is_empty() {
            let empty = Paragraph::new("No quests available.")
                .style(Style::default().fg(Color::Gray))
                .alignment(ratatui::layout::Alignment::Center);
            frame.render_widget(empty, area);
            return;
        }

        // Calculate visible window based on area height.
        let visible_height = area.height as usize;
        let selected = self.list_state.selected_index;

        // Determine scroll offset to keep selected item visible.
        let scroll_offset = if selected < self.list_state.scroll_offset {
            selected
        } else if selected >= self.list_state.scroll_offset + visible_height {
            selected - visible_height + 1
        } else {
            self.list_state.scroll_offset
        };

        // Build lines for visible quests.
        let mut lines: Vec<Line> = Vec::new();

        for (i, quest) in self.quests.iter().enumerate().skip(scroll_offset).take(visible_height) {
            let is_selected = i == selected;

            // Determine progress state and color.
            let (_progress_ratio, status_color) = match &quest.user_progress {
                Some(progress) if progress.is_completed => (1.0, Color::Green),
                Some(progress) if progress.tasks_completed > 0 => {
                    let ratio = if progress.tasks_total > 0 {
                        progress.tasks_completed as f64 / progress.tasks_total as f64
                    } else {
                        0.0
                    };
                    (ratio, Color::Yellow)
                }
                _ => (0.0, Color::DarkGray),
            };

            // Build the cursor indicator.
            let cursor = if is_selected { "> " } else { "  " };

            // Build progress text.
            let progress_text = match &quest.user_progress {
                Some(progress) => format!("{}/{}", progress.tasks_completed, progress.tasks_total),
                None => format!("0/{}", quest.task_count),
            };

            // Build the line.
            let base_style = if is_selected {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default().fg(status_color)
            };

            let title_style = if is_selected {
                base_style.add_modifier(Modifier::BOLD)
            } else {
                base_style
            };

            let meta_style = if is_selected {
                Style::default().bg(Color::DarkGray).fg(Color::Gray)
            } else {
                Style::default().fg(Color::Gray)
            };

            let line = Line::from(vec![
                Span::styled(cursor, title_style),
                Span::styled(&quest.title, title_style),
                Span::styled(
                    format!("  {} levels, {} tasks  ", quest.level_count, quest.task_count),
                    meta_style,
                ),
                Span::styled(progress_text, Style::default().fg(status_color)),
            ]);

            lines.push(line);
        }

        let list_paragraph = Paragraph::new(lines);
        frame.render_widget(list_paragraph, area);
    }

    /// Render the status bar with key hints.
    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let hints = Line::from(vec![
            Span::styled("j/k", Style::default().fg(Color::Yellow)),
            Span::raw(": navigate  "),
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::raw(": select  "),
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(": quit"),
        ]);

        let status_bar = Paragraph::new(hints)
            .style(Style::default().bg(Color::DarkGray));
        frame.render_widget(status_bar, area);
    }
}


// ─── Property Tests ─────────────────────────────────────────────────────────

// Feature: cli-tui-rewrite, Property 1: Cursor movement is clamped within list bounds

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Validates: Requirements 2.1, 2.2, 2.8, 2.9

    proptest! {
        /// move_down: after calling move_down(), selected_index == min(pos + 1, n - 1)
        #[test]
        fn prop_move_down_clamps_to_last_item(
            items_count in 1usize..=100,
            selected_index in 0usize..100,
        ) {
            let pos = selected_index.min(items_count - 1);
            let mut state = ListScreenState {
                selected_index: pos,
                scroll_offset: 0,
                items_count,
            };

            state.move_down();

            let expected = std::cmp::min(pos + 1, items_count - 1);
            prop_assert_eq!(state.selected_index, expected);
        }

        /// move_up: after calling move_up(), selected_index == max(pos - 1, 0)
        #[test]
        fn prop_move_up_clamps_to_first_item(
            items_count in 1usize..=100,
            selected_index in 0usize..100,
        ) {
            let pos = selected_index.min(items_count - 1);
            let mut state = ListScreenState {
                selected_index: pos,
                scroll_offset: 0,
                items_count,
            };

            state.move_up();

            let expected = pos.saturating_sub(1);
            prop_assert_eq!(state.selected_index, expected);
        }

        /// Boundary: when at last item, move_down() stays at last item
        #[test]
        fn prop_move_down_at_last_stays(items_count in 1usize..=100) {
            let mut state = ListScreenState {
                selected_index: items_count - 1,
                scroll_offset: 0,
                items_count,
            };

            state.move_down();

            prop_assert_eq!(state.selected_index, items_count - 1);
        }

        /// Boundary: when at first item (0), move_up() stays at 0
        #[test]
        fn prop_move_up_at_first_stays(items_count in 1usize..=100) {
            let mut state = ListScreenState {
                selected_index: 0,
                scroll_offset: 0,
                items_count,
            };

            state.move_up();

            prop_assert_eq!(state.selected_index, 0);
        }

        /// selected_index is always < items_count after any sequence of moves
        #[test]
        fn prop_selected_index_always_within_bounds(
            items_count in 1usize..=100,
            initial_pos in 0usize..100,
            moves in proptest::collection::vec(prop_oneof![Just(true), Just(false)], 0..50),
        ) {
            let pos = initial_pos.min(items_count - 1);
            let mut state = ListScreenState {
                selected_index: pos,
                scroll_offset: 0,
                items_count,
            };

            for move_down in moves {
                if move_down {
                    state.move_down();
                } else {
                    state.move_up();
                }
                prop_assert!(
                    state.selected_index < state.items_count,
                    "selected_index {} must be < items_count {}",
                    state.selected_index,
                    state.items_count
                );
            }
        }
    }
}
