use loop_with_break::find_first_above;

#[test]
fn test_found() {
    assert_eq!(find_first_above(&[1, 5, 3, 8, 2], 4), Some(5));
}

#[test]
fn test_not_found() {
    assert_eq!(find_first_above(&[1, 2, 3], 10), None);
}

#[test]
fn test_empty_slice() {
    assert_eq!(find_first_above(&[], 0), None);
}

#[test]
fn test_first_element() {
    assert_eq!(find_first_above(&[100, 1, 2], 50), Some(100));
}
