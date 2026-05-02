use result_chaining::validate_username;

#[test]
fn test_valid() {
    assert_eq!(validate_username("RustUser"), Ok(String::from("rustuser")));
}

#[test]
fn test_too_short() {
    assert!(validate_username("ab").is_err());
}

#[test]
fn test_too_long() {
    assert!(validate_username("a_very_long_username_that_exceeds").is_err());
}

#[test]
fn test_invalid_chars() {
    assert!(validate_username("user@name").is_err());
}

#[test]
fn test_with_underscore() {
    assert_eq!(validate_username("rust_dev"), Ok(String::from("rust_dev")));
}
