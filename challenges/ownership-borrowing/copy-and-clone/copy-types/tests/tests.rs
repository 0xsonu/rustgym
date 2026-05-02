use copy_types::demonstrate_copy;

#[test]
fn test_basic() {
    assert_eq!(demonstrate_copy(5), (5, 10, 15));
}

#[test]
fn test_zero() {
    assert_eq!(demonstrate_copy(0), (0, 0, 0));
}

#[test]
fn test_negative() {
    assert_eq!(demonstrate_copy(-3), (-3, -6, -9));
}
