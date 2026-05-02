use for_range::sum_range;

#[test]
fn test_one_to_five() {
    assert_eq!(sum_range(1, 5), 15);
}

#[test]
fn test_single_number() {
    assert_eq!(sum_range(3, 3), 3);
}

#[test]
fn test_negative_range() {
    assert_eq!(sum_range(-2, 2), 0);
}

#[test]
fn test_invalid_range() {
    assert_eq!(sum_range(5, 1), 0);
}
