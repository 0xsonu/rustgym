use hello_variables::create_greeting;

#[test]
fn test_returns_greeting() {
    assert_eq!(create_greeting(), "Hello, Rust!");
}

#[test]
fn test_returns_string_type() {
    let result = create_greeting();
    assert_eq!(result.len(), 12);
}

#[test]
fn test_not_empty() {
    assert!(!create_greeting().is_empty());
}
