use basic_result::parse_age;

#[test]
fn test_valid_age() {
    assert_eq!(parse_age("25"), Ok(25));
}

#[test]
fn test_zero() {
    assert_eq!(parse_age("0"), Ok(0));
}

#[test]
fn test_invalid_string() {
    assert!(parse_age("abc").is_err());
}

#[test]
fn test_negative() {
    assert!(parse_age("-5").is_err());
}

#[test]
fn test_max_valid() {
    assert_eq!(parse_age("150"), Ok(150));
}
