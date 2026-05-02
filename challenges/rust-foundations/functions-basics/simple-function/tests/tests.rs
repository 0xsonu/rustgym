use simple_function::add;

#[test]
fn test_add_positive() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn test_add_negative() {
    assert_eq!(add(-1, -4), -5);
}

#[test]
fn test_add_zero() {
    assert_eq!(add(0, 0), 0);
}

#[test]
fn test_add_mixed() {
    assert_eq!(add(10, -3), 7);
}
