use vec_filter_map::extract_positives;

#[test]
fn test_mixed() {
    assert_eq!(extract_positives(&[-1, 2, -3, 4, 5]), vec![4, 8, 10]);
}

#[test]
fn test_all_negative() {
    assert_eq!(extract_positives(&[-1, -2, -3]), Vec::<i32>::new());
}

#[test]
fn test_all_positive() {
    assert_eq!(extract_positives(&[1, 2, 3]), vec![2, 4, 6]);
}

#[test]
fn test_with_zero() {
    assert_eq!(extract_positives(&[0, 1, -1]), vec![2]);
}
