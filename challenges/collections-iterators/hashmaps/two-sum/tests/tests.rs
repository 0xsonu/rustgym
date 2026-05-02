use two_sum::two_sum;

#[test]
fn test_found() {
    assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
}

#[test]
fn test_not_found() {
    assert_eq!(two_sum(&[1, 2, 3], 10), None);
}

#[test]
fn test_negative() {
    assert_eq!(two_sum(&[-1, 5, 3, -4], 1), Some((0, 3)));
}

#[test]
fn test_empty() {
    assert_eq!(two_sum(&[], 5), None);
}
