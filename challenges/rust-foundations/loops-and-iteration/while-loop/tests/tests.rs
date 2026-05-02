use while_loop::count_digits;

#[test]
fn test_single_digit() {
    assert_eq!(count_digits(5), 1);
}

#[test]
fn test_two_digits() {
    assert_eq!(count_digits(42), 2);
}

#[test]
fn test_many_digits() {
    assert_eq!(count_digits(123456), 6);
}

#[test]
fn test_zero() {
    assert_eq!(count_digits(0), 1);
}
