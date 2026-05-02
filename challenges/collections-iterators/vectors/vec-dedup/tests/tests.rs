use vec_dedup::unique_sorted;

#[test]
fn test_with_duplicates() {
    assert_eq!(unique_sorted(vec![3, 1, 2, 1, 3, 2]), vec![1, 2, 3]);
}

#[test]
fn test_no_duplicates() {
    assert_eq!(unique_sorted(vec![5, 3, 1]), vec![1, 3, 5]);
}

#[test]
fn test_all_same() {
    assert_eq!(unique_sorted(vec![7, 7, 7]), vec![7]);
}

#[test]
fn test_empty() {
    assert_eq!(unique_sorted(vec![]), Vec::<i32>::new());
}
