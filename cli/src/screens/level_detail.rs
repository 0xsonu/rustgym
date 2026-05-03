use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::{Constraint, Direction, Frame, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::action::{Action, ApiCall, ScreenId};
use crate::api::error::AppError;
use crate::api::{ApiResponse, TaskSummary};
use crate::app::AppState;
use crate::event::AppEvent;

use super::quest_list::ListScreenState;
use super::ScreenHandler;

// ─── LevelDetailScreen ──────────────────────────────────────────────────────

/// Level detail screen showing tasks within a level.
#[derive(Debug, Clone)]
pub struct LevelDetailScreen {
    /// The slug identifying the parent quest.
    pub quest_slug: String,
    /// The slug identifying this level.
    pub level_slug: String,
    /// List navigation state.
    pub list_state: ListScreenState,
    /// Locally stored task data (populated from API response).
    pub tasks: Vec<TaskSummary>,
    /// Whether the initial fetch has been triggered.
    pub fetched: bool,
    /// Level title (populated from API response).
    pub title: String,
}

impl LevelDetailScreen {
    /// Create a new LevelDetailScreen for the given quest and level slugs.
    pub fn new(quest_slug: String, level_slug: String) -> Self {
        Self {
            quest_slug,
            level_slug,
            list_state: ListScreenState::new(),
            tasks: Vec::new(),
            fetched: false,
            title: String::new(),
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
                if !self.tasks.is_empty() {
                    let slug =
                        self.tasks[self.list_state.selected_index].slug.clone();
                    Action::Push(ScreenId::TaskDetail { slug })
                } else {
                    Action::None
                }
            }
            KeyCode::Char('b') => Action::Pop,
            _ => Action::None,
        }
    }

    /// Handle an API response relevant to the level detail screen.
    fn handle_api_response(&mut self, result: &crate::event::ApiResult) -> Action {
        match result {
            Ok(ApiResponse::LevelDetail {
                slug,
                title,
                tasks,
            }) => {
                // Only accept responses for our level slug.
                if slug == &self.level_slug {
                    self.title = title.clone();
                    self.tasks = tasks.clone();
                    self.list_state.items_count = self.tasks.len();
                    // Clamp selected_index if needed.
                    if self.list_state.selected_index >= self.tasks.len()
                        && !self.tasks.is_empty()
                    {
                        self.list_state.selected_index = self.tasks.len() - 1;
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
            Action::ApiRequest(ApiCall::FetchLevelDetail {
                quest_slug: self.quest_slug.clone(),
                level_slug: self.level_slug.clone(),
            })
        } else {
            Action::None
        }
    }

    /// Map a difficulty string to a color.
    pub fn difficulty_color(difficulty: &str) -> Color {
        match difficulty.to_lowercase().as_str() {
            "beginner" => Color::Green,
            "intermediate" => Color::Yellow,
            "advanced" => Color::Red,
            _ => Color::Gray,
        }
    }
}

impl ScreenHandler for LevelDetailScreen {
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

        if app_state.loading && self.tasks.is_empty() {
            self.render_loading(frame, layout[1], app_state);
        } else {
            self.render_task_list(frame, layout[1]);
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

impl LevelDetailScreen {
    /// Render the header with level title.
    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let title_text = if self.title.is_empty() {
            self.level_slug.clone()
        } else {
            self.title.clone()
        };

        let header_text = Line::from(vec![Span::styled(
            format!("Level — {}", title_text),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )]);

        let header =
            Paragraph::new(header_text).block(Block::default().borders(Borders::BOTTOM));
        frame.render_widget(header, area);
    }

    /// Render the loading spinner.
    fn render_loading(&self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        let idx = (app_state.tick as usize) % spinner_chars.len();
        let loading_text = format!("{} Loading tasks...", spinner_chars[idx]);

        let loading = Paragraph::new(loading_text)
            .style(Style::default().fg(Color::Cyan))
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(loading, area);
    }

    /// Render the scrollable task list.
    fn render_task_list(&self, frame: &mut Frame, area: Rect) {
        if self.tasks.is_empty() {
            let empty = Paragraph::new("No tasks available.")
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

        // Build lines for visible tasks.
        let mut lines: Vec<Line> = Vec::new();

        for (i, task) in self
            .tasks
            .iter()
            .enumerate()
            .skip(scroll_offset)
            .take(visible_height)
        {
            let is_selected = i == selected;

            // Difficulty color.
            let diff_color = Self::difficulty_color(&task.difficulty);

            // Completion status.
            let status_text = match &task.user_progress {
                Some(progress) if progress.status == "completed" => " ✓",
                Some(_) => " ○",
                None => " ○",
            };

            let status_color = match &task.user_progress {
                Some(progress) if progress.status == "completed" => Color::Green,
                _ => Color::DarkGray,
            };

            // Build the cursor indicator.
            let cursor = if is_selected { "> " } else { "  " };

            // Build the line.
            let base_style = if is_selected {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
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

            let diff_style = if is_selected {
                Style::default().bg(Color::DarkGray).fg(diff_color)
            } else {
                Style::default().fg(diff_color)
            };

            let line = Line::from(vec![
                Span::styled(cursor, title_style),
                Span::styled(&task.title, title_style),
                Span::styled("  [", meta_style),
                Span::styled(&task.difficulty, diff_style),
                Span::styled("]", meta_style),
                Span::styled(format!("  {} XP", task.xp_reward), meta_style),
                Span::styled(
                    status_text,
                    if is_selected {
                        Style::default().bg(Color::DarkGray).fg(status_color)
                    } else {
                        Style::default().fg(status_color)
                    },
                ),
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
