use if_expression::absolute_value;

#[test]
fn test_positive_unchanged() {
    assert_eq!(absolute_value(5), 5);
}

#[test]
fn test_negative_flipped() {
    assert_eq!(absolute_value(-7), 7);
}

#[test]
fn test_zero() {
    assert_eq!(absolute_value(0), 0);
}
