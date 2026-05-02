use error_hierarchy::process_user_record;

#[test]
fn test_valid_record() {
    assert_eq!(process_user_record("42", "Alice"), Ok(String::from("User #42: Alice")));
}

#[test]
fn test_invalid_id() {
    assert!(process_user_record("abc", "Alice").is_err());
}

#[test]
fn test_empty_name() {
    assert!(process_user_record("1", "").is_err());
}

#[test]
fn test_not_found() {
    assert!(process_user_record("0", "Ghost").is_err());
}

#[test]
fn test_display_not_found() {
    let result = process_user_record("0", "Ghost");
    assert_eq!(result.unwrap_err().to_string(), "user with id 0 not found");
}
