use option_question_mark::extract_domain;

#[test]
fn test_valid_email() {
    assert_eq!(extract_domain("user@example.com"), Some("example.com"));
}

#[test]
fn test_no_at() {
    assert_eq!(extract_domain("invalid"), None);
}

#[test]
fn test_empty_domain() {
    assert_eq!(extract_domain("user@"), None);
}

#[test]
fn test_multiple_at() {
    assert_eq!(extract_domain("a@b@c"), Some("b@c"));
}
