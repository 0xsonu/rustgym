use error_display::{validate_email, ValidationError};

#[test]
fn test_valid_email() {
    assert_eq!(validate_email("user@example.com"), Ok(()));
}

#[test]
fn test_too_short() {
    assert_eq!(
        validate_email("a@b"),
        Err(ValidationError::TooShort { field: String::from("email"), min: 5 })
    );
}

#[test]
fn test_invalid_format() {
    assert_eq!(
        validate_email("invalid-email"),
        Err(ValidationError::InvalidFormat {
            field: String::from("email"),
            expected: String::from("user@domain.tld"),
        })
    );
}

#[test]
fn test_display_message() {
    let err = ValidationError::TooShort { field: String::from("email"), min: 5 };
    assert_eq!(err.to_string(), "email must be at least 5 characters");
}
