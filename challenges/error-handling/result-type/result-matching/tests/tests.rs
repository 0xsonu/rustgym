use result_matching::safe_divide;

#[test]
fn test_valid_division() {
    assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
}

#[test]
fn test_division_by_zero() {
    assert_eq!(safe_divide(5.0, 0.0), Err(String::from("division by zero")));
}

#[test]
fn test_negative_division() {
    assert_eq!(safe_divide(-6.0, 3.0), Ok(-2.0));
}
