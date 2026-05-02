use error_context::{calculate, CalcError};

#[test]
fn test_addition() {
    assert_eq!(calculate(5, '+', 3), Ok(8));
}

#[test]
fn test_division_by_zero() {
    assert_eq!(calculate(10, '/', 0), Err(CalcError::DivisionByZero));
}

#[test]
fn test_invalid_operator() {
    assert_eq!(calculate(1, '%', 2), Err(CalcError::InvalidOperator('%')));
}

#[test]
fn test_overflow() {
    assert_eq!(calculate(i64::MAX, '+', 1), Err(CalcError::Overflow));
}

#[test]
fn test_subtraction() {
    assert_eq!(calculate(10, '-', 3), Ok(7));
}
