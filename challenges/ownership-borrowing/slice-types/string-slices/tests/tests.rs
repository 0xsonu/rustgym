use string_slices::first_word;

#[test]
fn test_multiple_words() {
    assert_eq!(first_word("hello world"), "hello");
}

#[test]
fn test_single_word() {
    assert_eq!(first_word("rust"), "rust");
}

#[test]
fn test_empty() {
    assert_eq!(first_word(""), "");
}

#[test]
fn test_leading_space() {
    assert_eq!(first_word(" leading"), "");
}
