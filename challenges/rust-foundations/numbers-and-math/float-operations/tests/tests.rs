use float_operations::float_math;

#[test]
fn test_positive_number() {
    let (abs, sqrt, rounded) = float_math(9.0);
    assert_eq!(abs, 9.0);
    assert_eq!(sqrt, 3.0);
    assert_eq!(rounded, 9.0);
}

#[test]
fn test_negative_number() {
    let (abs, sqrt, rounded) = float_math(-16.0);
    assert_eq!(abs, 16.0);
    assert_eq!(sqrt, 4.0);
    assert_eq!(rounded, -16.0);
}

#[test]
fn test_fractional() {
    let (abs, sqrt, rounded) = float_math(-2.7);
    assert!((abs - 2.7).abs() < 1e-10);
    assert!((sqrt - 2.7_f64.abs().sqrt()).abs() < 1e-10);
    assert_eq!(rounded, -3.0);
}
