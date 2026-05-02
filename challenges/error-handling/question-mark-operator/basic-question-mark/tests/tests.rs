use basic_question_mark::add_strings;

#[test]
fn test_valid_addition() {
    assert_eq!(add_strings("3", "4"), Ok(7));
}

#[test]
fn test_first_invalid() {
    assert!(add_strings("abc", "4").is_err());
}

#[test]
fn test_second_invalid() {
    assert!(add_strings("3", "xyz").is_err());
}

#[test]
fn test_negative_numbers() {
    assert_eq!(add_strings("-5", "10"), Ok(5));
}
