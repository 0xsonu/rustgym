// Feature: cli-tui-rewrite, Property 12: 401 response triggers session expiry flow
// Feature: cli-tui-rewrite, Property 13: Config error messages include path and OS error

use proptest::prelude::*;

use rustgym_cli::action::{Action, ScreenId};
use rustgym_cli::api::error::AppError;
use rustgym_cli::app::{App, AuthState};
use rustgym_cli::event::AppEvent;

// ─── Helpers ────────────────────────────────────────────────────────────────

/// Generate a random ScreenId for building navigation stacks.
fn arb_screen_id() -> impl Strategy<Value = ScreenId> {
    prop_oneof![
        Just(ScreenId::Login),
        Just(ScreenId::QuestList),
        "[a-z]{3,10}".prop_map(|slug| ScreenId::QuestDetail { slug }),
        ("[a-z]{3,10}", "[a-z]{3,10}")
            .prop_map(|(quest_slug, level_slug)| ScreenId::LevelDetail {
                quest_slug,
                level_slug
            }),
        "[a-z]{3,10}".prop_map(|slug| ScreenId::TaskDetail { slug }),
        "[a-z]{3,10}".prop_map(|slug| ScreenId::Submit { slug }),
    ]
}

/// Generate a navigation stack of a given size (always non-empty).
fn arb_nav_stack(min_size: usize, max_size: usize) -> impl Strategy<Value = Vec<ScreenId>> {
    proptest::collection::vec(arb_screen_id(), min_size..=max_size)
}

/// Generate a random username.
fn arb_username() -> impl Strategy<Value = String> {
    "[a-zA-Z][a-zA-Z0-9_]{2,15}"
}

/// Generate a random token string.
fn arb_token() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9]{20,40}"
}

/// Generate a random file path (Unix-style).
fn arb_file_path() -> impl Strategy<Value = String> {
    prop_oneof![
        "[a-z]{1,5}(/[a-z]{1,8}){1,4}/[a-z]{1,8}\\.[a-z]{2,4}"
            .prop_map(|p| format!("/{}", p)),
        Just("/home/user/.rustgym/config.toml".to_string()),
        Just("/tmp/test.toml".to_string()),
        "[a-z]{3,10}".prop_map(|name| format!("/home/{}/.rustgym/config.toml", name)),
    ]
}

/// Generate a random OS error message.
fn arb_os_error() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("Permission denied (os error 13)".to_string()),
        Just("No such file or directory (os error 2)".to_string()),
        Just("Read-only file system (os error 30)".to_string()),
        Just("Disk quota exceeded (os error 122)".to_string()),
        Just("Input/output error (os error 5)".to_string()),
        "[A-Z][a-z ]{5,30}\\(os error [0-9]{1,3}\\)".prop_map(|s| s),
    ]
}

// ─── Property 12: 401 response triggers session expiry flow ─────────────────

// **Validates: Requirements 9.4, 4.6**
//
// For any API endpoint that returns a 401 status code while the user is
// authenticated, the system SHALL clear the stored token AND transition to
// the Login screen. The resulting auth state SHALL be LoggedOut.
proptest! {
    #[test]
    fn prop_401_response_clears_auth_and_navigates_to_login(
        nav_stack in arb_nav_stack(1, 5),
        username in arb_username(),
        token in arb_token(),
    ) {
        // Set up an App with authenticated state and a random nav stack.
        let mut app = App::new();
        app.state.auth = AuthState::LoggedIn {
            token: token.clone(),
            username: username.clone(),
            xp: 100,
        };
        app.state.nav_stack = nav_stack;

        // Send an Unauthorized API response event.
        let event = AppEvent::ApiResponse(Err(AppError::Unauthorized));
        let action = app.update(event);

        // Verify the returned action is Replace(Login).
        match &action {
            Action::Replace(ScreenId::Login) => { /* expected */ }
            other => {
                prop_assert!(
                    false,
                    "Expected Action::Replace(Login), got {:?}",
                    other
                );
            }
        }

        // Verify auth state is LoggedOut.
        match &app.state.auth {
            AuthState::LoggedOut => { /* expected */ }
            other => {
                prop_assert!(
                    false,
                    "Expected AuthState::LoggedOut, got {:?}",
                    other
                );
            }
        }

        // Verify error_message is set to the session expired message.
        prop_assert_eq!(
            app.state.error_message.as_deref(),
            Some("Session expired — please log in again")
        );
    }

    #[test]
    fn prop_401_response_works_regardless_of_initial_nav_stack(
        nav_stack in arb_nav_stack(1, 8),
        username in arb_username(),
        token in arb_token(),
    ) {
        // Regardless of what screens are on the nav stack, a 401 should
        // always result in Replace(Login).
        let mut app = App::new();
        app.state.auth = AuthState::LoggedIn {
            token: token.clone(),
            username: username.clone(),
            xp: 50,
        };
        app.state.nav_stack = nav_stack.clone();

        let event = AppEvent::ApiResponse(Err(AppError::Unauthorized));
        let action = app.update(event);

        // Action must be Replace(Login)
        match &action {
            Action::Replace(ScreenId::Login) => { /* expected */ }
            other => {
                prop_assert!(
                    false,
                    "Expected Action::Replace(Login) for nav_stack {:?}, got {:?}",
                    nav_stack,
                    other
                );
            }
        }

        // Auth state must be LoggedOut
        match &app.state.auth {
            AuthState::LoggedOut => { /* expected */ }
            other => {
                prop_assert!(
                    false,
                    "Expected AuthState::LoggedOut, got {:?}",
                    other
                );
            }
        }
    }
}

// ─── Property 13: Config error messages include path and OS error ───────────

// **Validates: Requirements 9.6**
//
// For any file path `p` and OS error `e` encountered during config file
// operations, the displayed error message SHALL contain both `p` (the full
// file path) and `e` (the OS error description).
proptest! {
    #[test]
    fn prop_file_error_user_message_contains_path_and_error(
        path in arb_file_path(),
        error_msg in arb_os_error(),
    ) {
        let err = AppError::FileError {
            path: path.clone(),
            message: error_msg.clone(),
        };

        let user_message = err.user_message();

        // The user message must contain the file path.
        prop_assert!(
            user_message.contains(&path),
            "user_message '{}' does not contain path '{}'",
            user_message,
            path
        );

        // The user message must contain the OS error description.
        prop_assert!(
            user_message.contains(&error_msg),
            "user_message '{}' does not contain error message '{}'",
            user_message,
            error_msg
        );
    }

    #[test]
    fn prop_file_error_message_format_is_consistent(
        path in arb_file_path(),
        error_msg in arb_os_error(),
    ) {
        let err = AppError::FileError {
            path: path.clone(),
            message: error_msg.clone(),
        };

        let user_message = err.user_message();

        // The message should follow the format "File error at {path}: {message}"
        let expected = format!("File error at {}: {}", path, error_msg);
        prop_assert_eq!(
            &user_message,
            &expected,
            "Expected '{}', got '{}'",
            expected,
            user_message
        );
    }
}
