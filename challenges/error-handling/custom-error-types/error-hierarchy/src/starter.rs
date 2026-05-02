use std::fmt;
use std::num::ParseIntError;

/// Application-level error type.
/// TODO: Define Parse(ParseIntError), Validation(String), NotFound { entity, id } variants.
#[derive(Debug)]
pub enum AppError {
    // Add variants here
}

// TODO: Implement Display for AppError.
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

// TODO: Implement From<ParseIntError> for AppError.
impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        todo!()
    }
}

/// Processes a user record from raw strings.
/// TODO: Parse id, validate name, check for id=0.
pub fn process_user_record(id_str: &str, name: &str) -> Result<String, AppError> {
    todo!()
}
