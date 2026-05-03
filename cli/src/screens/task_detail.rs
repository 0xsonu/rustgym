use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::{Constraint, Direction, Frame, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::action::{Action, ApiCall, ScreenId};
use crate::api::error::AppError;
use crate::api::ApiResponse;
use crate::app::{AppState, TaskDetailData};
use crate::event::AppEvent;

use super::ScreenHandler;

// ─── TaskDetailScreen ───────────────────────────────────────────────────────

/// Task detail screen showing task description and actions.
#[derive(Debug, Clone)]
pub struct TaskDetailScreen {
    /// The slug identifying this task.
    pub slug: String,
    /// Vertical scroll offset for the description area.
    pub scroll_offset: u16,
    /// Whether the hint section is currently visible.
    pub show_hint: bool,
    /// Locally cached task detail data.
    pub task_data: Option<TaskDetailData>,
    /// Whether the initial fetch has been triggered.
    pub fetched: bool,
}

impl TaskDetailScreen {
    /// Create a new TaskDetailScreen for the given task slug.
    pub fn new(slug: String) -> Self {
        Self {
            slug,
            scroll_offset: 0,
            show_hint: false,
            task_data: None,
            fetched: false,
        }
    }

    /// Handle keyboard input.
    fn handle_key_input(&mut self, key: &KeyEvent) -> Action {
        match key.code {
            KeyCode::Char('b') => Action::Pop,
            KeyCode::Char('s') => Action::Push(ScreenId::Submit {
                slug: self.slug.clone(),
            }),
            KeyCode::Char('i')
            | KeyCode::Char('a')
            | KeyCode::Char('A')
            | KeyCode::Char('I')
            | KeyCode::Enter => {
                let starter_code = self
                    .task_data
                    .as_ref()
                    .map(|d| d.starter_code.clone())
                    .unwrap_or_default();
                let description_md = self
                    .task_data
                    .as_ref()
                    .map(|d| d.description_md.clone())
                    .unwrap_or_default();
                Action::LaunchEditor {
                    slug: self.slug.clone(),
                    starter_code,
                    description_md,
                }
            }
            KeyCode::Char('?') => {
                self.show_hint = !self.show_hint;
                Action::None
            }
            KeyCode::Char('j') => {
                self.scroll_offset = self.scroll_offset.saturating_add(1);
                Action::None
            }
            KeyCode::Char('k') => {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
                Action::None
            }
            _ => Action::None,
        }
    }

    /// Handle an API response relevant to the task detail screen.
    fn handle_api_response(&mut self, result: &crate::event::ApiResult) -> Action {
        match result {
            Ok(ApiResponse::TaskDetail(data)) => {
                if data.slug == self.slug {
                    self.task_data = Some(TaskDetailData {
                        slug: data.slug.clone(),
                        title: data.title.clone(),
                        description_md: data.description_md.clone(),
                        starter_code: data.starter_code.clone(),
                        difficulty: data.difficulty.clone(),
                        xp_reward: data.xp_reward,
                        hint_md: data.hint_md.clone(),
                    });
                }
                Action::None
            }
            Err(AppError::Unauthorized) => Action::Batch(vec![
                Action::ShowError("Session expired — please log in again".to_string()),
                Action::Replace(ScreenId::Login),
            ]),
            Err(_) => Action::None,
            _ => Action::None,
        }
    }

    /// Check if we need to trigger the initial fetch.
    fn maybe_fetch(&mut self, app_state: &AppState) -> Action {
        if !self.fetched {
            self.fetched = true;
            // Check cache first.
            if let Some(cached) = app_state.cache.task_details.get(&self.slug) {
                self.task_data = Some(cached.clone());
                return Action::None;
            }
            Action::ApiRequest(ApiCall::FetchTaskDetail {
                slug: self.slug.clone(),
            })
        } else {
            Action::None
        }
    }

    /// Map a difficulty string to a color.
    fn difficulty_color(difficulty: &str) -> Color {
        match difficulty.to_lowercase().as_str() {
            "beginner" => Color::Green,
            "intermediate" => Color::Yellow,
            "advanced" => Color::Red,
            _ => Color::Gray,
        }
    }

    /// Parse markdown text into styled lines with basic formatting:
    /// - Lines starting with `#` are rendered bold
    /// - Lines within ``` blocks are rendered with gray background (code style)
    fn render_markdown(text: &str) -> Vec<Line<'static>> {
        let mut lines: Vec<Line<'static>> = Vec::new();
        let mut in_code_block = false;

        for raw_line in text.lines() {
            if raw_line.trim_start().starts_with("```") {
                in_code_block = !in_code_block;
                // Render the fence line itself in code style.
                lines.push(Line::from(Span::styled(
                    raw_line.to_string(),
                    Style::default().fg(Color::DarkGray),
                )));
                continue;
            }

            if in_code_block {
                lines.push(Line::from(Span::styled(
                    raw_line.to_string(),
                    Style::default().fg(Color::White).bg(Color::DarkGray),
                )));
            } else if raw_line.trim_start().starts_with('#') {
                // Header line — render bold.
                let header_text = raw_line.trim_start_matches('#').trim().to_string();
                lines.push(Line::from(Span::styled(
                    header_text,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )));
            } else {
                lines.push(Line::from(Span::raw(raw_line.to_string())));
            }
        }

        lines
    }
}

impl ScreenHandler for TaskDetailScreen {
    fn update(&mut self, event: &AppEvent, app_state: &AppState) -> Action {
        // On first update, trigger fetch if needed.
        let fetch_action = self.maybe_fetch(app_state);
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
        // Main layout: header, content, status bar.
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(1),   // Content area
                Constraint::Length(1), // Status bar
            ])
            .split(area);

        self.render_header(frame, layout[0]);

        if app_state.loading && self.task_data.is_none() {
            self.render_loading(frame, layout[1], app_state);
        } else {
            self.render_content(frame, layout[1]);
        }

        self.render_status_bar(frame, layout[2]);
    }

    fn key_hints(&self) -> Vec<(&str, &str)> {
        vec![
            ("i", "open editor"),
            ("s", "submit"),
            ("?", "hint"),
            ("b", "back"),
        ]
    }
}

// ─── Rendering ──────────────────────────────────────────────────────────────

impl TaskDetailScreen {
    /// Render the header with task title, difficulty badge, and XP reward.
    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let (title, difficulty, xp) = match &self.task_data {
            Some(data) => (data.title.clone(), data.difficulty.clone(), data.xp_reward),
            None => (self.slug.clone(), String::new(), 0),
        };

        let diff_color = Self::difficulty_color(&difficulty);

        let mut spans = vec![Span::styled(
            format!("  {} ", title),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )];

        if !difficulty.is_empty() {
            spans.push(Span::styled(
                format!(" [{}] ", difficulty),
                Style::default().fg(diff_color).add_modifier(Modifier::BOLD),
            ));
        }

        if xp > 0 {
            spans.push(Span::styled(
                format!(" {} XP", xp),
                Style::default().fg(Color::Magenta),
            ));
        }

        let header_line = Line::from(spans);
        let header = Paragraph::new(header_line)
            .block(Block::default().borders(Borders::BOTTOM));
        frame.render_widget(header, area);
    }

    /// Render the loading spinner.
    fn render_loading(&self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        let idx = (app_state.tick as usize) % spinner_chars.len();
        let loading_text = format!("{} Loading task...", spinner_chars[idx]);

        let loading = Paragraph::new(loading_text)
            .style(Style::default().fg(Color::Cyan))
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(loading, area);
    }

    /// Render the main content area: description + optional hint.
    fn render_content(&self, frame: &mut Frame, area: Rect) {
        let Some(data) = &self.task_data else {
            let empty = Paragraph::new("No task data available.")
                .style(Style::default().fg(Color::Gray))
                .alignment(ratatui::layout::Alignment::Center);
            frame.render_widget(empty, area);
            return;
        };

        // Build all content lines: description + optional hint.
        let mut all_lines = Self::render_markdown(&data.description_md);

        // Add hint section if toggled on and hint is available.
        if self.show_hint {
            if let Some(hint_md) = &data.hint_md {
                all_lines.push(Line::from(""));
                all_lines.push(Line::from(Span::styled(
                    "━━━ Hint ━━━",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )));
                all_lines.push(Line::from(""));
                let hint_lines = Self::render_markdown(hint_md);
                all_lines.extend(hint_lines);
            } else {
                all_lines.push(Line::from(""));
                all_lines.push(Line::from(Span::styled(
                    "No hint available for this task.",
                    Style::default().fg(Color::DarkGray),
                )));
            }
        }

        let content = Paragraph::new(all_lines)
            .wrap(Wrap { trim: false })
            .scroll((self.scroll_offset, 0));
        frame.render_widget(content, area);
    }

    /// Render the status bar with key hints.
    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let hints = Line::from(vec![
            Span::styled("i", Style::default().fg(Color::Yellow)),
            Span::raw(": open editor  "),
            Span::styled("s", Style::default().fg(Color::Yellow)),
            Span::raw(": submit  "),
            Span::styled("?", Style::default().fg(Color::Yellow)),
            Span::raw(": hint  "),
            Span::styled("b", Style::default().fg(Color::Yellow)),
            Span::raw(": back"),
        ]);

        let status_bar =
            Paragraph::new(hints).style(Style::default().bg(Color::DarkGray));
        frame.render_widget(status_bar, area);
    }
}
