use split_at_delimiter::split_once_at;

#[test]
fn test_email() {
    assert_eq!(split_once_at("user@example.com", '@'), ("user", "example.com"));
}

#[test]
fn test_no_delimiter() {
    assert_eq!(split_once_at("hello", ':'), ("hello", ""));
}

#[test]
fn test_multiple_delimiters() {
    assert_eq!(split_once_at("a:b:c", ':'), ("a", "b:c"));
}

#[test]
fn test_delimiter_at_start() {
    assert_eq!(split_once_at(":hello", ':'), ("", "hello"));
}
