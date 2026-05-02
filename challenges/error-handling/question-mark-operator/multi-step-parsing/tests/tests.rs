use multi_step_parsing::{parse_point, Point};

#[test]
fn test_valid_point() {
    assert_eq!(parse_point("3.5,4.2"), Ok(Point { x: 3.5, y: 4.2 }));
}

#[test]
fn test_with_spaces() {
    assert_eq!(parse_point("1.0, 2.0"), Ok(Point { x: 1.0, y: 2.0 }));
}

#[test]
fn test_invalid_format() {
    assert!(parse_point("1.0").is_err());
}

#[test]
fn test_invalid_number() {
    assert!(parse_point("abc,2.0").is_err());
}
