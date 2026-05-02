use filter_in_place::remove_negatives;

#[test]
fn test_mixed() {
    let mut nums = vec![1, -2, 3, -4, 5];
    remove_negatives(&mut nums);
    assert_eq!(nums, vec![1, 3, 5]);
}

#[test]
fn test_all_negative() {
    let mut nums = vec![-1, -2, -3];
    remove_negatives(&mut nums);
    assert_eq!(nums, Vec::<i32>::new());
}

#[test]
fn test_no_negatives() {
    let mut nums = vec![1, 2, 3];
    remove_negatives(&mut nums);
    assert_eq!(nums, vec![1, 2, 3]);
}

#[test]
fn test_with_zero() {
    let mut nums = vec![-1, 0, 1];
    remove_negatives(&mut nums);
    assert_eq!(nums, vec![0, 1]);
}
