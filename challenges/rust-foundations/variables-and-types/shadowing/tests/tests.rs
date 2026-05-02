use shadowing::shadow_transform;

#[test]
fn test_returns_ten_string() {
    assert_eq!(shadow_transform(), "10");
}

#[test]
fn test_returns_string_type() {
    let result = shadow_transform();
    assert_eq!(result.parse::<i32>().unwrap(), 10);
}

#[test]
fn test_not_empty() {
    assert!(!shadow_transform().is_empty());
}
