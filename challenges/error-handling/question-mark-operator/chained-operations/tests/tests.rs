use chained_operations::parse_key_value;

#[test]
fn test_valid() {
    assert_eq!(parse_key_value("port=8080"), Ok(8080));
}

#[test]
fn test_with_spaces() {
    assert_eq!(parse_key_value("count = 42"), Ok(42));
}

#[test]
fn test_no_delimiter() {
    assert!(parse_key_value("invalid").is_err());
}

#[test]
fn test_invalid_value() {
    assert!(parse_key_value("key=abc").is_err());
}
