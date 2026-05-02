use shared_access::common_elements;

#[test]
fn test_some_common() {
    assert_eq!(common_elements(&[1, 2, 3, 4], &[3, 4, 5, 6]), vec![3, 4]);
}

#[test]
fn test_no_common() {
    assert_eq!(common_elements(&[1, 2], &[3, 4]), Vec::<i32>::new());
}

#[test]
fn test_all_common() {
    assert_eq!(common_elements(&[1, 2, 3], &[1, 2, 3]), vec![1, 2, 3]);
}

#[test]
fn test_empty_slice() {
    assert_eq!(common_elements(&[], &[1, 2, 3]), Vec::<i32>::new());
}
