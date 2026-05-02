use propagate_errors::sum_positive_numbers;

#[test]
fn test_all_positive() {
    assert_eq!(sum_positive_numbers(&["1", "2", "3"]), Ok(6));
}

#[test]
fn test_mixed() {
    assert_eq!(sum_positive_numbers(&["-1", "5", "-2", "3"]), Ok(8));
}

#[test]
fn test_invalid_input() {
    assert!(sum_positive_numbers(&["1", "bad", "3"]).is_err());
}

#[test]
fn test_empty() {
    assert_eq!(sum_positive_numbers(&[]), Ok(0));
}
