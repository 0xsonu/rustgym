use std::fmt;
use std::num::ParseIntError;

/// Application-level error type that wraps multiple error sources.
#[derive(Debug)]
pub enum AppError {
    Parse(ParseIntError),
    Validation(String),
    NotFound { entity: String, id: u64 },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Parse(e) => write!(f, "parse error: {}", e),
            AppError::Validation(msg) => write!(f, "validation error: {}", msg),
            AppError::NotFound { entity, id } => write!(f, "{} with id {} not found", entity, id),
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::Parse(err)
    }
}

/// Processes a user record from raw strings.
pub fn process_user_record(id_str: &str, name: &str) -> Result<String, AppError> {
    let id: u64 = id_str.parse()?;

    if name.is_empty() {
        return Err(AppError::Validation(String::from("name cannot be empty")));
    }
    if id == 0 {
        return Err(AppError::NotFound { entity: String::from("user"), id });
    }

    Ok(format!("User #{}: {}", id, name))
}
