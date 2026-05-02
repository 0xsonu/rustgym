use std::fmt;

/// Validation error with context.
#[derive(Debug, PartialEq)]
pub enum ValidationError {
    TooShort { field: String, min: usize },
    TooLong { field: String, max: usize },
    InvalidFormat { field: String, expected: String },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::TooShort { field, min } => {
                write!(f, "{} must be at least {} characters", field, min)
            }
            ValidationError::TooLong { field, max } => {
                write!(f, "{} must be at most {} characters", field, max)
            }
            ValidationError::InvalidFormat { field, expected } => {
                write!(f, "{} must match format: {}", field, expected)
            }
        }
    }
}

/// Validates an email address.
pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    if email.len() < 5 {
        return Err(ValidationError::TooShort { field: String::from("email"), min: 5 });
    }
    if email.len() > 254 {
        return Err(ValidationError::TooLong { field: String::from("email"), max: 254 });
    }
    if !email.contains('@') || !email.contains('.') {
        return Err(ValidationError::InvalidFormat {
            field: String::from("email"),
            expected: String::from("user@domain.tld"),
        });
    }
    Ok(())
}
