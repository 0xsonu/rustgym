use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::{Constraint, Direction, Frame, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::action::{Action, ApiCall, ScreenId};
use crate::api::ApiResponse;
use crate::app::AppState;
use crate::event::AppEvent;

use super::ScreenHandler;

/// Which mode the login screen is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginMode {
    /// Normal mode — navigation keys work (Tab, Enter, Esc to quit).
    Normal,
    /// Editing mode — all printable keys go to the focused field.
    Editing,
}

/// Which field currently has focus on the login form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginField {
    Email,
    Password,
}

/// Login screen with email/password input fields.
#[derive(Debug, Clone)]
pub struct LoginScreen {
    /// Current input mode.
    pub mode: LoginMode,
    /// Which input field is currently focused.
    pub focused_field: LoginField,
    /// Email input buffer.
    pub email: String,
    /// Password input buffer.
    pub password: String,
    /// Error message to display (e.g., invalid credentials).
    pub error: Option<String>,
    /// Whether a login request is currently in-flight.
    pub submitting: bool,
}

impl LoginScreen {
    /// Create a new LoginScreen with empty fields.
    pub fn new() -> Self {
        Self {
            mode: LoginMode::Normal,
            focused_field: LoginField::Email,
            email: String::new(),
            password: String::new(),
            error: None,
            submitting: false,
        }
    }

    /// Toggle focus between Email and Password fields.
    fn toggle_focus(&mut self) {
        self.focused_field = match self.focused_field {
            LoginField::Email => LoginField::Password,
            LoginField::Password => LoginField::Email,
        };
    }

    /// Append a character to the currently focused field.
    fn push_char(&mut self, c: char) {
        match self.focused_field {
            LoginField::Email => self.email.push(c),
            LoginField::Password => self.password.push(c),
        }
    }

    /// Remove the last character from the currently focused field.
    fn pop_char(&mut self) {
        match self.focused_field {
            LoginField::Email => {
                self.email.pop();
            }
            LoginField::Password => {
                self.password.pop();
            }
        }
    }

    /// Validate fields and attempt login submission.
    fn try_submit(&mut self) -> Action {
        if self.email.is_empty() || self.password.is_empty() {
            self.error = Some("Email and password are required".to_string());
            return Action::None;
        }

        self.error = None;
        self.submitting = true;

        Action::ApiRequest(ApiCall::Login {
            email: self.email.clone(),
            password: self.password.clone(),
        })
    }

    /// Handle an API response relevant to login.
    fn handle_api_response(&mut self, response: &crate::event::ApiResult) -> Action {
        self.submitting = false;

        match response {
            Ok(ApiResponse::LoginSuccess {
                access_token,
                username,
                xp,
                ..
            }) => {
                // On success: store token, update auth state, navigate to quest list.
                Action::Batch(vec![
                    Action::SetAuth {
                        token: access_token.clone(),
                        username: username.clone(),
                        xp: *xp,
                    },
                    Action::Replace(ScreenId::QuestList),
                ])
            }
            Err(err) => {
                self.error = Some(err.user_message());
                Action::None
            }
            // Ignore other API responses not meant for this screen.
            _ => Action::None,
        }
    }
}

impl Default for LoginScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenHandler for LoginScreen {
    fn update(&mut self, event: &AppEvent, _app_state: &AppState) -> Action {
        match event {
            AppEvent::Input(key_event) => self.handle_key_input(key_event),
            AppEvent::ApiResponse(result) => self.handle_api_response(result),
            _ => Action::None,
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        self.render_login_form(frame, area, app_state);
    }

    fn key_hints(&self) -> Vec<(&str, &str)> {
        if self.mode == LoginMode::Editing {
            vec![
                ("Esc", "stop editing"),
                ("Tab", "next field"),
                ("Enter", "login"),
            ]
        } else {
            vec![
                ("i", "edit"),
                ("Tab", "switch field"),
                ("Enter", "login"),
                ("Esc", "quit"),
            ]
        }
    }
}

// ─── Key Input Handling ─────────────────────────────────────────────────────

impl LoginScreen {
    fn handle_key_input(&mut self, key: &KeyEvent) -> Action {
        match self.mode {
            LoginMode::Normal => self.handle_normal_mode(key),
            LoginMode::Editing => self.handle_editing_mode(key),
        }
    }

    /// Handle keys in normal (command) mode.
    fn handle_normal_mode(&mut self, key: &KeyEvent) -> Action {
        match key.code {
            // Enter editing mode.
            KeyCode::Char('i') | KeyCode::Char('a') => {
                self.mode = LoginMode::Editing;
                Action::None
            }

            // Quit.
            KeyCode::Char('q') | KeyCode::Esc => Action::Quit,

            // Tab / Shift+Tab to switch focus.
            KeyCode::Tab => {
                self.toggle_focus();
                Action::None
            }
            KeyCode::BackTab => {
                self.toggle_focus();
                Action::None
            }

            // Enter to submit (if fields are filled).
            KeyCode::Enter => self.try_submit(),

            // j/k to switch fields (vim-style).
            KeyCode::Char('j') => {
                self.focused_field = LoginField::Password;
                Action::None
            }
            KeyCode::Char('k') => {
                self.focused_field = LoginField::Email;
                Action::None
            }

            _ => Action::None,
        }
    }

    /// Handle keys in editing mode — all printable chars go to the focused field.
    fn handle_editing_mode(&mut self, key: &KeyEvent) -> Action {
        match key.code {
            // Escape exits editing mode.
            KeyCode::Esc => {
                self.mode = LoginMode::Normal;
                Action::None
            }

            // Tab / Shift+Tab to switch focus (stays in editing mode).
            KeyCode::Tab => {
                self.toggle_focus();
                Action::None
            }
            KeyCode::BackTab => {
                self.toggle_focus();
                Action::None
            }

            // Enter to submit.
            KeyCode::Enter => self.try_submit(),

            // Backspace to delete last character.
            KeyCode::Backspace => {
                self.pop_char();
                Action::None
            }

            // Printable characters (excluding ctrl/alt combinations).
            KeyCode::Char(c)
                if !key.modifiers.contains(KeyModifiers::CONTROL)
                    && !key.modifiers.contains(KeyModifiers::ALT) =>
            {
                self.push_char(c);
                Action::None
            }

            _ => Action::None,
        }
    }
}

// ─── Rendering ──────────────────────────────────────────────────────────────

impl LoginScreen {
    fn render_login_form(&self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        // Center the form vertically and horizontally.
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Min(14),
                Constraint::Percentage(25),
            ])
            .split(area);

        let horizontal_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Min(40),
                Constraint::Percentage(25),
            ])
            .split(vertical_layout[1]);

        let form_area = horizontal_layout[1];

        // Split form area into: title, email field, password field, error/spinner, mode/hints, register hint.
        let form_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2), // Title
                Constraint::Length(3), // Email field
                Constraint::Length(3), // Password field
                Constraint::Length(2), // Error message or spinner
                Constraint::Length(1), // Mode indicator + key hints
                Constraint::Length(1), // Register hint
            ])
            .split(form_area);

        // Title.
        let title = Paragraph::new("RustGym Login")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(title, form_layout[0]);

        // Email field.
        let email_focused = self.focused_field == LoginField::Email;
        let email_editing = email_focused && self.mode == LoginMode::Editing;
        let email_style = if email_editing {
            Style::default().fg(Color::Green)
        } else if email_focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::Gray)
        };
        let email_title = if email_editing { " Email (editing) " } else { " Email " };
        let email_block = Block::default()
            .title(email_title)
            .borders(Borders::ALL)
            .border_style(email_style);
        let email_text = Paragraph::new(self.email.as_str()).block(email_block);
        frame.render_widget(email_text, form_layout[1]);

        // Password field (masked).
        let pass_focused = self.focused_field == LoginField::Password;
        let pass_editing = pass_focused && self.mode == LoginMode::Editing;
        let password_style = if pass_editing {
            Style::default().fg(Color::Green)
        } else if pass_focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::Gray)
        };
        let pass_title = if pass_editing { " Password (editing) " } else { " Password " };
        let password_block = Block::default()
            .title(pass_title)
            .borders(Borders::ALL)
            .border_style(password_style);
        let masked_password: String = "•".repeat(self.password.len());
        let password_text = Paragraph::new(masked_password.as_str()).block(password_block);
        frame.render_widget(password_text, form_layout[2]);

        // Error message or spinner.
        if self.submitting {
            let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
            let idx = (app_state.tick as usize) % spinner_chars.len();
            let spinner_text = format!("{} Logging in...", spinner_chars[idx]);
            let spinner = Paragraph::new(spinner_text)
                .style(Style::default().fg(Color::Cyan))
                .alignment(ratatui::layout::Alignment::Center);
            frame.render_widget(spinner, form_layout[3]);
        } else if let Some(ref err) = self.error {
            let error_paragraph = Paragraph::new(err.as_str())
                .style(Style::default().fg(Color::Red))
                .alignment(ratatui::layout::Alignment::Center);
            frame.render_widget(error_paragraph, form_layout[3]);
        }

        // Mode indicator + key hints.
        let hints = if self.mode == LoginMode::Editing {
            Line::from(vec![
                Span::styled("-- EDIT --", Style::default().fg(Color::Green)),
                Span::raw("  "),
                Span::styled("Esc", Style::default().fg(Color::Yellow)),
                Span::raw(": stop  "),
                Span::styled("Tab", Style::default().fg(Color::Yellow)),
                Span::raw(": next  "),
                Span::styled("Enter", Style::default().fg(Color::Yellow)),
                Span::raw(": login"),
            ])
        } else {
            Line::from(vec![
                Span::styled("i", Style::default().fg(Color::Yellow)),
                Span::raw(": edit  "),
                Span::styled("Tab", Style::default().fg(Color::Yellow)),
                Span::raw(": switch  "),
                Span::styled("Enter", Style::default().fg(Color::Yellow)),
                Span::raw(": login  "),
                Span::styled("Esc/q", Style::default().fg(Color::Yellow)),
                Span::raw(": quit"),
            ])
        };
        let hints_paragraph = Paragraph::new(hints)
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(hints_paragraph, form_layout[4]);

        // Register hint.
        let register_hint = Line::from(vec![
            Span::styled("No account? ", Style::default().fg(Color::DarkGray)),
            Span::styled("Visit https://rustgym.dev to register", Style::default().fg(Color::Cyan)),
        ]);
        let register_paragraph = Paragraph::new(register_hint)
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(register_paragraph, form_layout[5]);
    }

    /// Returns the masked password string as it would be rendered.
    /// Useful for testing that password masking preserves length.
    #[cfg(test)]
    pub fn masked_password(&self) -> String {
        "•".repeat(self.password.len())
    }
}

// ─── Property-Based Tests ───────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
    use proptest::prelude::*;

    use crate::app::AppState;
    use crate::event::AppEvent;
    use crate::screens::ScreenHandler;

    /// Helper to create a KeyEvent for a printable character.
    fn char_key_event(c: char) -> KeyEvent {
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    /// Strategy to generate a LoginField variant.
    fn login_field_strategy() -> impl Strategy<Value = LoginField> {
        prop_oneof![Just(LoginField::Email), Just(LoginField::Password),]
    }

    /// Strategy to generate a LoginScreen with arbitrary initial state.
    fn login_screen_strategy() -> impl Strategy<Value = LoginScreen> {
        (
            login_field_strategy(),
            "[a-zA-Z0-9@._]{0,20}",
            "[a-zA-Z0-9!@#$%^&*]{0,20}",
        )
            .prop_map(|(field, email, password)| {
                let mut screen = LoginScreen::new();
                screen.mode = LoginMode::Editing; // Tests assume editing mode for char input
                screen.focused_field = field;
                screen.email = email;
                screen.password = password;
                screen
            })
    }

    /// Strategy to generate printable ASCII characters (excluding control chars).
    fn printable_char_strategy() -> impl Strategy<Value = char> {
        (0x20u8..=0x7Eu8).prop_map(|b| b as char)
    }

    proptest! {
        /// Feature: cli-tui-rewrite, Property 4: Text input appends characters to the focused field
        ///
        /// **Validates: Requirements 3.3**
        ///
        /// For any printable character `c` and any login screen state with a focused field
        /// (email or password), typing `c` results in the focused field's buffer being the
        /// previous buffer with `c` appended. The unfocused field remains unchanged.
        #[test]
        fn prop_text_input_appends_to_focused_field(
            mut screen in login_screen_strategy(),
            c in printable_char_strategy(),
        ) {
            let app_state = AppState::new();

            // Capture state before input.
            let email_before = screen.email.clone();
            let password_before = screen.password.clone();
            let focused = screen.focused_field;

            // Create the key event and dispatch it.
            let key_event = char_key_event(c);
            let event = AppEvent::Input(key_event);
            let _action = screen.update(&event, &app_state);

            match focused {
                LoginField::Email => {
                    // Focused field (email) should have `c` appended.
                    let mut expected = email_before;
                    expected.push(c);
                    prop_assert_eq!(&screen.email, &expected,
                        "Email field should have char '{}' appended", c);
                    // Unfocused field (password) should be unchanged.
                    prop_assert_eq!(&screen.password, &password_before,
                        "Password field should remain unchanged when email is focused");
                }
                LoginField::Password => {
                    // Focused field (password) should have `c` appended.
                    let mut expected = password_before;
                    expected.push(c);
                    prop_assert_eq!(&screen.password, &expected,
                        "Password field should have char '{}' appended", c);
                    // Unfocused field (email) should be unchanged.
                    prop_assert_eq!(&screen.email, &email_before,
                        "Email field should remain unchanged when password is focused");
                }
            }
        }

        /// Feature: cli-tui-rewrite, Property 5: Password masking preserves length
        ///
        /// **Validates: Requirements 3.4**
        ///
        /// For any password string of length `n`, the rendered password display consists
        /// of exactly `n` mask characters (`•`), preserving no information about the
        /// original characters.
        #[test]
        fn prop_password_masking_preserves_length(
            password in "[\\x20-\\x7E]{0,50}",
        ) {
            let mut screen = LoginScreen::new();
            screen.password = password.clone();

            let masked = screen.masked_password();

            // The mask should have exactly as many `•` characters as the password length.
            prop_assert_eq!(masked.chars().count(), password.len(),
                "Masked password should have same char count as original password");

            // Every character in the mask should be `•`.
            for ch in masked.chars() {
                prop_assert_eq!(ch, '•',
                    "All mask characters should be '•', found '{}'", ch);
            }
        }
    }
}
