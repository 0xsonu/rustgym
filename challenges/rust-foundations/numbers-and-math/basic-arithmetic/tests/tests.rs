use basic_arithmetic::calculate;

#[test]
fn test_positive_numbers() {
    assert_eq!(calculate(10, 3), (13, 7, 30, 3, 1));
}

#[test]
fn test_with_one() {
    assert_eq!(calculate(7, 1), (8, 6, 7, 7, 0));
}

#[test]
fn test_negative_numbers() {
    assert_eq!(calculate(-6, 2), (-4, -8, -12, -3, 0));
}

#[test]
fn test_equal_numbers() {
    assert_eq!(calculate(5, 5), (10, 0, 25, 1, 0));
}
