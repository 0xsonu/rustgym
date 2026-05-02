use borrow_vec::sum_and_average;

#[test]
fn test_basic() {
    let nums = vec![1.0, 2.0, 3.0, 4.0];
    let (sum, avg) = sum_and_average(&nums);
    assert_eq!(sum, 10.0);
    assert_eq!(avg, 2.5);
    // nums is still usable
    assert_eq!(nums.len(), 4);
}

#[test]
fn test_empty() {
    let nums: Vec<f64> = vec![];
    assert_eq!(sum_and_average(&nums), (0.0, 0.0));
}

#[test]
fn test_single() {
    let nums = vec![5.0];
    assert_eq!(sum_and_average(&nums), (5.0, 5.0));
}
