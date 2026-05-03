/// Application-level error type for the CLI TUI.
///
/// Categorizes errors into actionable variants so the UI can display
/// appropriate messages and offer relevant recovery actions (retry, re-login, etc.).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AppError {
    /// Network timeout (>30s).
    Timeout,
    /// Connection refused to the given URL.
    ConnectionRefused { url: String },
    /// HTTP 401 — session expired.
    Unauthorized,
    /// HTTP 404 — resource not found.
    NotFound,
    /// HTTP 5xx — server error.
    ServerError,
    /// File I/O error with path and description.
    FileError { path: String, message: String },
    /// No editor binary found on the system.
    EditorNotFound,
    /// Generic error with a human-readable message.
    Other(String),
}

impl AppError {
    /// Returns a human-readable error message suitable for display in the TUI.
    pub fn user_message(&self) -> String {
        match self {
            Self::Timeout => "Request timed out".to_string(),
            Self::ConnectionRefused { url } => format!("Cannot connect to server at {}", url),
            Self::Unauthorized => "Session expired — please log in again".to_string(),
            Self::NotFound => "Resource not found".to_string(),
            Self::ServerError => "Server error — please try again later".to_string(),
            Self::FileError { path, message } => format!("File error at {}: {}", path, message),
            Self::EditorNotFound => {
                "No editor found. Set $EDITOR environment variable.".to_string()
            }
            Self::Other(msg) => msg.clone(),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            return AppError::Timeout;
        }

        if err.is_connect() {
            let url = err
                .url()
                .map(|u| u.to_string())
                .unwrap_or_else(|| "unknown".to_string());
            return AppError::ConnectionRefused { url };
        }

        AppError::Other(err.to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeout_message() {
        let err = AppError::Timeout;
        assert_eq!(err.user_message(), "Request timed out");
    }

    #[test]
    fn connection_refused_message() {
        let err = AppError::ConnectionRefused {
            url: "http://localhost:3000".to_string(),
        };
        assert_eq!(
            err.user_message(),
            "Cannot connect to server at http://localhost:3000"
        );
    }

    #[test]
    fn unauthorized_message() {
        let err = AppError::Unauthorized;
        assert_eq!(err.user_message(), "Session expired — please log in again");
    }

    #[test]
    fn not_found_message() {
        let err = AppError::NotFound;
        assert_eq!(err.user_message(), "Resource not found");
    }

    #[test]
    fn server_error_message() {
        let err = AppError::ServerError;
        assert_eq!(err.user_message(), "Server error — please try again later");
    }

    #[test]
    fn file_error_message() {
        let err = AppError::FileError {
            path: "/home/user/.rustgym/config.toml".to_string(),
            message: "Permission denied".to_string(),
        };
        assert_eq!(
            err.user_message(),
            "File error at /home/user/.rustgym/config.toml: Permission denied"
        );
    }

    #[test]
    fn editor_not_found_message() {
        let err = AppError::EditorNotFound;
        assert_eq!(
            err.user_message(),
            "No editor found. Set $EDITOR environment variable."
        );
    }

    #[test]
    fn other_message() {
        let err = AppError::Other("Something went wrong".to_string());
        assert_eq!(err.user_message(), "Something went wrong");
    }

    #[test]
    fn clone_works() {
        let err = AppError::ConnectionRefused {
            url: "http://example.com".to_string(),
        };
        let cloned = err.clone();
        assert_eq!(err.user_message(), cloned.user_message());
    }

    #[test]
    fn debug_works() {
        let err = AppError::Timeout;
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("Timeout"));
    }
}
