use result_map::sum_strings;

#[test]
fn test_all_valid() {
    assert_eq!(sum_strings(&["1", "2", "3"]), Ok(6));
}

#[test]
fn test_with_invalid() {
    assert!(sum_strings(&["1", "abc", "3"]).is_err());
}

#[test]
fn test_empty() {
    assert_eq!(sum_strings(&[]), Ok(0));
}

#[test]
fn test_negative_numbers() {
    assert_eq!(sum_strings(&["-1", "5", "-2"]), Ok(2));
}
