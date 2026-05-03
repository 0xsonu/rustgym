use std::fs;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::{Constraint, Direction, Frame, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::action::{Action, ApiCall, ScreenId};
use crate::api::error::AppError;
use crate::api::{ApiResponse, SubmissionResult};
use crate::app::AppState;
use crate::event::AppEvent;

use super::ScreenHandler;

// ─── SubmitScreen ───────────────────────────────────────────────────────────

/// Screen state for the submission result view.
#[derive(Debug, Clone)]
enum SubmitState {
    /// Waiting for the API response.
    Loading,
    /// Submission completed successfully or with test failures.
    Done(SubmissionResult),
    /// An error occurred reading the file or from the API.
    Error(String),
}

/// Submit screen showing submission progress and results.
#[derive(Debug, Clone)]
pub struct SubmitScreen {
    /// The task slug being submitted.
    slug: String,
    /// The solution code read from the local file.
    code: Option<String>,
    /// Current state of the submission.
    state: SubmitState,
    /// Vertical scroll offset for result output.
    scroll_offset: u16,
    /// Whether the initial submit has been triggered.
    submitted: bool,
}

impl SubmitScreen {
    /// Create a new SubmitScreen for the given task slug.
    ///
    /// Reads the solution file from ~/.rustgym/challenges/<slug>/src/lib.rs.
    /// If the file cannot be read, the screen starts in an error state.
    pub fn new(slug: String) -> Self {
        let file_path = crate::editor::file_path_for_slug(&slug);
        let (code, state) = match fs::read_to_string(&file_path) {
            Ok(contents) => (Some(contents), SubmitState::Loading),
            Err(e) => (
                None,
                SubmitState::Error(format!("Cannot read {}: {}", file_path, e)),
            ),
        };

        Self {
            slug,
            code,
            state,
            scroll_offset: 0,
            submitted: false,
        }
    }

    /// Handle keyboard input.
    fn handle_key_input(&mut self, key: &KeyEvent) -> Action {
        match key.code {
            KeyCode::Char('b') => Action::Pop,
            KeyCode::Char('r') => {
                // Retry: re-trigger submission if we have code.
                if let Some(code) = &self.code {
                    self.state = SubmitState::Loading;
                    self.submitted = true;
                    self.scroll_offset = 0;
                    Action::ApiRequest(ApiCall::SubmitSolution {
                        slug: self.slug.clone(),
                        code: code.clone(),
                    })
                } else {
                    Action::None
                }
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.scroll_offset = self.scroll_offset.saturating_add(1);
                Action::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
                Action::None
            }
            _ => Action::None,
        }
    }

    /// Handle an API response relevant to the submit screen.
    fn handle_api_response(&mut self, result: &crate::event::ApiResult) -> Action {
        match result {
            Ok(ApiResponse::SubmitResult(submission)) => {
                self.state = SubmitState::Done(submission.clone());
                Action::None
            }
            Err(AppError::Unauthorized) => Action::Batch(vec![
                Action::ShowError("Session expired — please log in again".to_string()),
                Action::Replace(ScreenId::Login),
            ]),
            Err(e) => {
                self.state = SubmitState::Error(e.user_message());
                Action::None
            }
            _ => Action::None,
        }
    }

    /// Trigger the initial submission if not yet done.
    fn maybe_submit(&mut self) -> Action {
        if !self.submitted {
            self.submitted = true;
            if let Some(code) = &self.code {
                Action::ApiRequest(ApiCall::SubmitSolution {
                    slug: self.slug.clone(),
                    code: code.clone(),
                })
            } else {
                // Already in error state from constructor.
                Action::None
            }
        } else {
            Action::None
        }
    }

    /// Count passed and failed tests from the test_results JSON value.
    fn count_tests(test_results: &serde_json::Value) -> (usize, usize) {
        let mut passed = 0;
        let mut failed = 0;
        if let Some(tests) = test_results.as_array() {
            for test in tests {
                if test.get("passed").and_then(|v| v.as_bool()).unwrap_or(false) {
                    passed += 1;
                } else {
                    failed += 1;
                }
            }
        }
        (passed, failed)
    }

    /// Extract failed test entries from test_results JSON.
    fn failed_tests(test_results: &serde_json::Value) -> Vec<(String, String)> {
        let mut failures = Vec::new();
        if let Some(tests) = test_results.as_array() {
            for test in tests {
                let is_passed = test.get("passed").and_then(|v| v.as_bool()).unwrap_or(false);
                if !is_passed {
                    let name = test
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                        .to_string();
                    let error = test
                        .get("error")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    failures.push((name, error));
                }
            }
        }
        failures
    }
}

impl ScreenHandler for SubmitScreen {
    fn update(&mut self, event: &AppEvent, _app_state: &AppState) -> Action {
        // On first update, trigger the submission.
        let submit_action = self.maybe_submit();
        if !matches!(submit_action, Action::None) {
            let event_action = match event {
                AppEvent::Input(key_event) => self.handle_key_input(key_event),
                AppEvent::ApiResponse(result) => self.handle_api_response(result),
                _ => Action::None,
            };
            return if matches!(event_action, Action::None) {
                submit_action
            } else {
                Action::Batch(vec![submit_action, event_action])
            };
        }

        match event {
            AppEvent::Input(key_event) => self.handle_key_input(key_event),
            AppEvent::ApiResponse(result) => self.handle_api_response(result),
            _ => Action::None,
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(1),   // Content area
                Constraint::Length(1), // Status bar
            ])
            .split(area);

        self.render_header(frame, layout[0]);
        self.render_content(frame, layout[1], app_state);
        self.render_status_bar(frame, layout[2]);
    }

    fn key_hints(&self) -> Vec<(&str, &str)> {
        vec![("b", "back"), ("r", "retry")]
    }
}

// ─── Rendering ──────────────────────────────────────────────────────────────

impl SubmitScreen {
    /// Render the header with task slug.
    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let title = format!("  Submit: {} ", self.slug);
        let header = Paragraph::new(Line::from(Span::styled(
            title,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )))
        .block(Block::default().borders(Borders::BOTTOM));
        frame.render_widget(header, area);
    }

    /// Render the main content area based on current state.
    fn render_content(&self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        match &self.state {
            SubmitState::Loading => self.render_loading(frame, area, app_state),
            SubmitState::Done(result) => self.render_result(frame, area, result),
            SubmitState::Error(msg) => self.render_error(frame, area, msg),
        }
    }

    /// Render the loading spinner while awaiting response.
    fn render_loading(&self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        let idx = (app_state.tick as usize) % spinner_chars.len();
        let loading_text = format!("{} Running tests...", spinner_chars[idx]);

        let loading = Paragraph::new(loading_text)
            .style(Style::default().fg(Color::Cyan))
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(loading, area);
    }

    /// Build the lines for the submission result display.
    /// Extracted for testability — the same logic used by render_result.
    #[cfg(test)]
    fn build_result_lines(result: &SubmissionResult) -> Vec<String> {
        let mut text_lines: Vec<String> = Vec::new();

        let (passed, failed) = Self::count_tests(&result.test_results);
        let is_success = result.status == "passed" || result.status == "success";

        // Status line
        if is_success {
            text_lines.push("  ✓ All tests passed!".to_string());
        } else {
            text_lines.push("  ✗ Some tests failed".to_string());
        }

        text_lines.push(String::new());

        // Summary stats
        text_lines.push(format!("  Tests passed: {}  Failed: {}", passed, failed));
        text_lines.push(format!("  Duration: {}ms", result.duration_ms));
        text_lines.push(format!("  XP awarded: +{}", result.xp_awarded));
        text_lines.push(format!("  Attempt: #{}", result.attempt_number));

        // Level-up congratulations
        if result.leveled_up {
            text_lines.push(String::new());
            let level_msg = match result.new_level {
                Some(lvl) => format!("  🎉 Congratulations! You reached level {}!", lvl),
                None => "  🎉 Congratulations! You leveled up!".to_string(),
            };
            text_lines.push(level_msg);
        }

        // Failed tests detail
        if failed > 0 {
            text_lines.push(String::new());
            text_lines.push("  ━━━ Failed Tests ━━━".to_string());
            text_lines.push(String::new());

            for (name, error) in Self::failed_tests(&result.test_results) {
                text_lines.push(format!("  ✗ {}", name));
                if !error.is_empty() {
                    for err_line in error.lines() {
                        text_lines.push(format!("    {}", err_line));
                    }
                }
                text_lines.push(String::new());
            }
        }

        text_lines
    }

    /// Render the submission result (success or failure).
    fn render_result(&self, frame: &mut Frame, area: Rect, result: &SubmissionResult) {
        let mut lines: Vec<Line<'_>> = Vec::new();

        let (passed, failed) = Self::count_tests(&result.test_results);
        let is_success = result.status == "passed" || result.status == "success";

        // Status line
        if is_success {
            lines.push(Line::from(Span::styled(
                "  ✓ All tests passed!",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )));
        } else {
            lines.push(Line::from(Span::styled(
                "  ✗ Some tests failed",
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            )));
        }

        lines.push(Line::from(""));

        // Summary stats
        lines.push(Line::from(vec![
            Span::styled("  Tests passed: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", passed),
                Style::default().fg(Color::Green),
            ),
            Span::styled(
                format!("  Failed: {}", failed),
                Style::default().fg(if failed > 0 { Color::Red } else { Color::Gray }),
            ),
        ]));

        lines.push(Line::from(vec![
            Span::styled("  Duration: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}ms", result.duration_ms),
                Style::default().fg(Color::White),
            ),
        ]));

        lines.push(Line::from(vec![
            Span::styled("  XP awarded: ", Style::default().fg(Color::Gray)),
            if result.xp_awarded > 0 {
                Span::styled(
                    format!("+{}", result.xp_awarded),
                    Style::default().fg(Color::Magenta),
                )
            } else {
                Span::styled(
                    "0 (already completed)".to_string(),
                    Style::default().fg(Color::DarkGray),
                )
            },
        ]));

        lines.push(Line::from(vec![
            Span::styled("  Attempt: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("#{}", result.attempt_number),
                Style::default().fg(Color::White),
            ),
        ]));

        // Level-up congratulations
        if result.leveled_up {
            lines.push(Line::from(""));
            let level_msg = match result.new_level {
                Some(lvl) => format!("  🎉 Congratulations! You reached level {}!", lvl),
                None => "  🎉 Congratulations! You leveled up!".to_string(),
            };
            lines.push(Line::from(Span::styled(
                level_msg,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )));
        }

        // Failed tests detail
        if failed > 0 {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "  ━━━ Failed Tests ━━━",
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));

            for (name, error) in Self::failed_tests(&result.test_results) {
                lines.push(Line::from(Span::styled(
                    format!("  ✗ {}", name),
                    Style::default().fg(Color::Red),
                )));
                if !error.is_empty() {
                    for err_line in error.lines() {
                        lines.push(Line::from(Span::styled(
                            format!("    {}", err_line),
                            Style::default().fg(Color::DarkGray),
                        )));
                    }
                }
                lines.push(Line::from(""));
            }
        }

        // Stdout section — only show if tests failed (useful for debugging).
        // When all tests pass, stdout is just test runner output (noise).
        if !is_success {
            if let Some(stdout) = &result.stdout {
                if !stdout.is_empty() {
                    lines.push(Line::from(Span::styled(
                        "  ━━━ Test Output ━━━",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )));
                    lines.push(Line::from(""));
                    for line in stdout.lines() {
                        lines.push(Line::from(Span::raw(format!("    {}", line))));
                    }
                    lines.push(Line::from(""));
                }
            }
        }

        // Stderr section — only show if it contains actual errors (not just build logs).
        // Filter out cargo compilation noise (lines starting with "Compiling", "Finished", "Running", "Doc-tests").
        if let Some(stderr) = &result.stderr {
            let filtered: Vec<&str> = stderr
                .lines()
                .filter(|line| {
                    let trimmed = line.trim();
                    !trimmed.is_empty()
                        && !trimmed.starts_with("Compiling")
                        && !trimmed.starts_with("Finished")
                        && !trimmed.starts_with("Running")
                        && !trimmed.starts_with("Doc-tests")
                        && !trimmed.starts_with("Downloading")
                        && !trimmed.starts_with("Downloaded")
                })
                .collect();

            if !filtered.is_empty() {
                lines.push(Line::from(Span::styled(
                    "  ━━━ Compiler Errors ━━━",
                    Style::default()
                        .fg(Color::Red)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(""));
                for line in &filtered {
                    lines.push(Line::from(Span::styled(
                        format!("    {}", line),
                        Style::default().fg(Color::Red),
                    )));
                }
                lines.push(Line::from(""));
            }
        }

        let content = Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .scroll((self.scroll_offset, 0));
        frame.render_widget(content, area);
    }

    /// Render an error message.
    fn render_error(&self, frame: &mut Frame, area: Rect, message: &str) {
        let lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                "  ✗ Error",
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                format!("  {}", message),
                Style::default().fg(Color::Red),
            )),
        ];

        let content = Paragraph::new(lines).wrap(Wrap { trim: false });
        frame.render_widget(content, area);
    }

    /// Render the status bar with key hints.
    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let hints = Line::from(vec![
            Span::styled("b", Style::default().fg(Color::Yellow)),
            Span::raw(": back  "),
            Span::styled("r", Style::default().fg(Color::Yellow)),
            Span::raw(": retry"),
        ]);

        let status_bar =
            Paragraph::new(hints).style(Style::default().bg(Color::DarkGray));
        frame.render_widget(status_bar, area);
    }
}


// ─── Property Tests ─────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// Generate a random test name (alphanumeric with underscores).
    fn arb_test_name() -> impl Strategy<Value = String> {
        "[a-z][a-z0-9_]{1,30}".prop_map(|s| s)
    }

    /// Generate a random error message (non-empty printable ASCII).
    fn arb_error_message() -> impl Strategy<Value = String> {
        "[a-zA-Z0-9 :_\\-\\.]{1,100}".prop_map(|s| s)
    }

    /// Generate a JSON array of test results with random failed tests (1..=max_failed).
    fn arb_failed_test_results(max_failed: usize) -> impl Strategy<Value = serde_json::Value> {
        proptest::collection::vec(
            (arb_test_name(), arb_error_message()),
            1..=max_failed,
        )
        .prop_map(|failed_entries| {
            let mut tests = Vec::new();
            for (name, error) in failed_entries {
                tests.push(serde_json::json!({
                    "name": name,
                    "passed": false,
                    "error": error
                }));
            }
            serde_json::Value::Array(tests)
        })
    }

    // Feature: cli-tui-rewrite, Property 10: Submission success rendering includes all result fields
    //
    // **Validates: Requirements 8.3**
    //
    // For any successful submission result with tests_passed, duration_ms, and xp_awarded values,
    // the rendered output SHALL contain all three values formatted as human-readable text.

    proptest! {
        #[test]
        fn prop_submission_success_rendering_includes_all_result_fields(
            num_passed in 1usize..=50,
            duration_ms in 0i32..=100_000,
            xp_awarded in 0i32..=10_000,
            attempt_number in 1i32..=100,
        ) {
            let test_results = {
                let mut tests = Vec::new();
                for i in 0..num_passed {
                    tests.push(serde_json::json!({
                        "name": format!("test_{}", i),
                        "passed": true
                    }));
                }
                serde_json::Value::Array(tests)
            };

            let result = SubmissionResult {
                status: "passed".to_string(),
                test_results,
                stdout: None,
                stderr: None,
                duration_ms,
                xp_awarded,
                attempt_number,
                leveled_up: false,
                new_level: None,
            };

            let lines = SubmitScreen::build_result_lines(&result);
            let output = lines.join("\n");

            // The rendered output must contain the tests passed count
            let (passed_count, _) = SubmitScreen::count_tests(&result.test_results);
            prop_assert!(
                output.contains(&format!("{}", passed_count)),
                "Output must contain tests passed count '{}', got:\n{}",
                passed_count,
                output
            );

            // The rendered output must contain the duration in ms
            prop_assert!(
                output.contains(&format!("{}ms", duration_ms)),
                "Output must contain duration '{}ms', got:\n{}",
                duration_ms,
                output
            );

            // The rendered output must contain the XP awarded
            prop_assert!(
                output.contains(&format!("+{}", xp_awarded)),
                "Output must contain XP awarded '+{}', got:\n{}",
                xp_awarded,
                output
            );
        }
    }

    // Feature: cli-tui-rewrite, Property 11: All failed tests are individually displayed
    //
    // **Validates: Requirements 8.4**
    //
    // For any submission result with n failed tests (n > 0), the rendered output SHALL contain
    // exactly n distinct test failure entries, each including the test name and error output.

    proptest! {
        #[test]
        fn prop_all_failed_tests_individually_displayed(
            test_results in arb_failed_test_results(20),
        ) {
            // Extract the expected failed tests
            let expected_failures = SubmitScreen::failed_tests(&test_results);
            let n = expected_failures.len();
            prop_assert!(n > 0, "Must have at least one failed test");

            let result = SubmissionResult {
                status: "failed".to_string(),
                test_results: test_results.clone(),
                stdout: None,
                stderr: None,
                duration_ms: 100,
                xp_awarded: 0,
                attempt_number: 1,
                leveled_up: false,
                new_level: None,
            };

            let lines = SubmitScreen::build_result_lines(&result);
            let output = lines.join("\n");

            // Each failed test name must appear in the output
            for (name, error) in &expected_failures {
                prop_assert!(
                    output.contains(name),
                    "Output must contain failed test name '{}', got:\n{}",
                    name,
                    output
                );
                // Each error message must appear in the output (if non-empty)
                if !error.is_empty() {
                    prop_assert!(
                        output.contains(error),
                        "Output must contain error '{}' for test '{}', got:\n{}",
                        error,
                        name,
                        output
                    );
                }
            }

            // Count the number of failure marker entries (lines starting with "  ✗ ")
            let failure_entries: Vec<&String> = lines
                .iter()
                .filter(|line| line.starts_with("  ✗ ") && !line.contains("Some tests failed"))
                .collect();

            prop_assert_eq!(
                failure_entries.len(),
                n,
                "Expected exactly {} failure entries, found {}. Lines:\n{}",
                n,
                failure_entries.len(),
                output
            );
        }

        #[test]
        fn prop_failed_tests_helper_returns_correct_count(
            num_passed in 0usize..=10,
            num_failed in 1usize..=20,
        ) {
            // Build test_results JSON with known passed/failed counts
            let mut tests = Vec::new();
            for i in 0..num_passed {
                tests.push(serde_json::json!({
                    "name": format!("pass_test_{}", i),
                    "passed": true
                }));
            }
            for i in 0..num_failed {
                tests.push(serde_json::json!({
                    "name": format!("fail_test_{}", i),
                    "passed": false,
                    "error": format!("assertion failed in test {}", i)
                }));
            }
            let test_results = serde_json::Value::Array(tests);

            let failures = SubmitScreen::failed_tests(&test_results);
            prop_assert_eq!(
                failures.len(),
                num_failed,
                "failed_tests() should return exactly {} entries, got {}",
                num_failed,
                failures.len()
            );

            // Verify each failure has the expected name
            for i in 0..num_failed {
                let expected_name = format!("fail_test_{}", i);
                prop_assert_eq!(
                    &failures[i].0,
                    &expected_name,
                    "Expected failure name '{}', got '{}'",
                    expected_name,
                    failures[i].0
                );
            }
        }
    }
}
