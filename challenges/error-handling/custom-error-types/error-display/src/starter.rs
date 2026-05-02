use std::fmt;

/// Validation error with context.
/// TODO: Define TooShort, TooLong, and InvalidFormat variants with named fields.
#[derive(Debug, PartialEq)]
pub enum ValidationError {
    // Add variants here
}

// TODO: Implement Display for ValidationError.
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// Validates an email address.
/// TODO: Check length (5-254) and format (contains @ and .).
pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    todo!()
}
