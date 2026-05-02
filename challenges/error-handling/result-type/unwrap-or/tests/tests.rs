use unwrap_or::{get_config_value, get_config_or_else};

#[test]
fn test_valid_value() {
    assert_eq!(get_config_value("42", 0), 42);
}

#[test]
fn test_invalid_uses_default() {
    assert_eq!(get_config_value("abc", 100), 100);
}

#[test]
fn test_or_else_valid() {
    assert_eq!(get_config_or_else("10", || 99), 10);
}

#[test]
fn test_or_else_invalid() {
    assert_eq!(get_config_or_else("bad", || 99), 99);
}
