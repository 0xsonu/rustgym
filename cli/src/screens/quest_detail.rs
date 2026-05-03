use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::{Constraint, Direction, Frame, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::action::{Action, ApiCall, ScreenId};
use crate::api::error::AppError;
use crate::api::{ApiResponse, LevelSummary};
use crate::app::AppState;
use crate::event::AppEvent;

use super::quest_list::ListScreenState;
use super::ScreenHandler;

// ─── QuestDetailScreen ──────────────────────────────────────────────────────

/// Quest detail screen showing levels within a quest.
#[derive(Debug, Clone)]
pub struct QuestDetailScreen {
    /// The slug identifying this quest.
    pub slug: String,
    /// List navigation state.
    pub list_state: ListScreenState,
    /// Locally stored level data (populated from API response).
    pub levels: Vec<LevelSummary>,
    /// Whether the initial fetch has been triggered.
    pub fetched: bool,
    /// Quest title (populated from API response).
    pub title: String,
}

impl QuestDetailScreen {
    /// Create a new QuestDetailScreen for the given quest slug.
    pub fn new(slug: String) -> Self {
        Self {
            slug,
            list_state: ListScreenState::new(),
            levels: Vec::new(),
            fetched: false,
            title: String::new(),
        }
    }

    /// Handle keyboard input.
    fn handle_key_input(&mut self, key: &KeyEvent) -> Action {
        match key.code {
            KeyCode::Char('j') => {
                self.move_to_next_unlocked(true);
                Action::None
            }
            KeyCode::Char('k') => {
                self.move_to_next_unlocked(false);
                Action::None
            }
            KeyCode::Enter => {
                if !self.levels.is_empty() {
                    let idx = self.list_state.selected_index;
                    // Don't allow entering locked levels.
                    if self.is_level_locked(idx) {
                        return Action::None;
                    }
                    let level_slug = self.levels[idx].slug.clone();
                    Action::Push(ScreenId::LevelDetail {
                        quest_slug: self.slug.clone(),
                        level_slug,
                    })
                } else {
                    Action::None
                }
            }
            KeyCode::Char('b') => Action::Pop,
            _ => Action::None,
        }
    }

    /// Move cursor to the next/previous unlocked level.
    fn move_to_next_unlocked(&mut self, forward: bool) {
        if self.levels.is_empty() {
            return;
        }
        let current = self.list_state.selected_index;
        if forward {
            // Find next unlocked level after current.
            for i in (current + 1)..self.levels.len() {
                if !self.is_level_locked(i) {
                    self.list_state.selected_index = i;
                    return;
                }
            }
            // No unlocked level found below — stay put.
        } else {
            // Find previous unlocked level before current.
            if current == 0 {
                return;
            }
            for i in (0..current).rev() {
                if !self.is_level_locked(i) {
                    self.list_state.selected_index = i;
                    return;
                }
            }
            // No unlocked level found above — stay put.
        }
    }

    /// Handle an API response relevant to the quest detail screen.
    fn handle_api_response(&mut self, result: &crate::event::ApiResult) -> Action {
        match result {
            Ok(ApiResponse::QuestDetail {
                slug,
                title,
                levels,
            }) => {
                // Only accept responses for our quest slug.
                if slug == &self.slug {
                    self.title = title.clone();
                    self.levels = levels.clone();
                    self.list_state.items_count = self.levels.len();
                    // Clamp selected_index if needed.
                    if self.list_state.selected_index >= self.levels.len()
                        && !self.levels.is_empty()
                    {
                        self.list_state.selected_index = self.levels.len() - 1;
                    }
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
    fn maybe_fetch(&mut self) -> Action {
        if !self.fetched {
            self.fetched = true;
            Action::ApiRequest(ApiCall::FetchQuestDetail {
                slug: self.slug.clone(),
            })
        } else {
            Action::None
        }
    }

    /// Determine if a level is locked based on sequential prerequisite logic.
    /// A level is locked if the previous level in the list is not completed.
    pub fn is_level_locked(&self, index: usize) -> bool {
        if index == 0 {
            return false;
        }
        // Previous level must be completed for this one to be unlocked.
        match &self.levels[index - 1].user_progress {
            Some(progress) => !progress.is_completed,
            None => true, // No progress on previous level means it's not completed.
        }
    }
}

impl ScreenHandler for QuestDetailScreen {
    fn update(&mut self, event: &AppEvent, _app_state: &AppState) -> Action {
        // On first update, trigger fetch if needed.
        let fetch_action = self.maybe_fetch();
        if !matches!(fetch_action, Action::None) {
            let event_action = match event {
                AppEvent::Input(key_event) => self.handle_key_input(key_event),
                AppEvent::ApiResponse(result) => self.handle_api_response(result),
                _ => Action::None,
            };
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

        self.render_header(frame, layout[0]);

        if app_state.loading && self.levels.is_empty() {
            self.render_loading(frame, layout[1], app_state);
        } else {
            self.render_level_list(frame, layout[1]);
        }

        self.render_status_bar(frame, layout[2]);
    }

    fn key_hints(&self) -> Vec<(&str, &str)> {
        vec![
            ("j/k", "navigate"),
            ("Enter", "select"),
            ("b", "back"),
        ]
    }
}

// ─── Rendering ──────────────────────────────────────────────────────────────

impl QuestDetailScreen {
    /// Render the header with quest title.
    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let title_text = if self.title.is_empty() {
            self.slug.clone()
        } else {
            self.title.clone()
        };

        let header_text = Line::from(vec![
            Span::styled(
                format!("Quest — {}", title_text),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]);

        let header =
            Paragraph::new(header_text).block(Block::default().borders(Borders::BOTTOM));
        frame.render_widget(header, area);
    }

    /// Render the loading spinner.
    fn render_loading(&self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        let idx = (app_state.tick as usize) % spinner_chars.len();
        let loading_text = format!("{} Loading levels...", spinner_chars[idx]);

        let loading = Paragraph::new(loading_text)
            .style(Style::default().fg(Color::Cyan))
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(loading, area);
    }

    /// Render the scrollable level list.
    fn render_level_list(&self, frame: &mut Frame, area: Rect) {
        if self.levels.is_empty() {
            let empty = Paragraph::new("No levels available.")
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

        // Build lines for visible levels.
        let mut lines: Vec<Line> = Vec::new();

        for (i, level) in self
            .levels
            .iter()
            .enumerate()
            .skip(scroll_offset)
            .take(visible_height)
        {
            let is_selected = i == selected;
            let is_locked = self.is_level_locked(i);

            // Determine completion status and color.
            let status_color = match &level.user_progress {
                Some(progress) if progress.is_completed => Color::Green,
                Some(progress) if progress.tasks_completed > 0 => Color::Yellow,
                _ => Color::DarkGray,
            };

            // Build the cursor indicator.
            let cursor = if is_selected { "> " } else { "  " };

            // Lock icon for levels with unmet prerequisites.
            let lock_indicator = if is_locked { "🔒 " } else { "" };

            // Completion status text.
            let status_text = match &level.user_progress {
                Some(progress) if progress.is_completed => " ✓".to_string(),
                Some(progress) => {
                    format!(" {}/{}", progress.tasks_completed, progress.tasks_total)
                }
                None => format!(" 0/{}", level.task_count),
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
                Span::styled(lock_indicator.to_string(), meta_style),
                Span::styled(&level.title, title_style),
                Span::styled(
                    format!("  {} tasks", level.task_count),
                    meta_style,
                ),
                Span::styled(status_text, Style::default().fg(status_color)),
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
            Span::styled("b", Style::default().fg(Color::Yellow)),
            Span::raw(": back"),
        ]);

        let status_bar =
            Paragraph::new(hints).style(Style::default().bg(Color::DarkGray));
        frame.render_widget(status_bar, area);
    }
}
