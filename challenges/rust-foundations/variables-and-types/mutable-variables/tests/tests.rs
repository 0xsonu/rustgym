use mutable_variables::count_up;

#[test]
fn test_returns_three() {
    assert_eq!(count_up(), 3);
}

#[test]
fn test_positive_result() {
    assert!(count_up() > 0);
}

#[test]
fn test_exact_value() {
    let result = count_up();
    assert_eq!(result, 3, "Expected exactly 3 increments from 0");
}
