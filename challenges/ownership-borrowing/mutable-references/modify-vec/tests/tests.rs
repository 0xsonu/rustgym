use modify_vec::double_elements;

#[test]
fn test_basic() {
    let mut nums = vec![1, 2, 3];
    double_elements(&mut nums);
    assert_eq!(nums, vec![2, 4, 6]);
}

#[test]
fn test_empty() {
    let mut nums: Vec<i32> = vec![];
    double_elements(&mut nums);
    assert_eq!(nums, Vec::<i32>::new());
}

#[test]
fn test_negative() {
    let mut nums = vec![-1, -2, 0];
    double_elements(&mut nums);
    assert_eq!(nums, vec![-2, -4, 0]);
}
