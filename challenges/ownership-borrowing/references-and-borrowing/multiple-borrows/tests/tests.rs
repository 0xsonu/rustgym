use multiple_borrows::longer;

#[test]
fn test_first_longer() {
    assert_eq!(longer("hello world", "hi"), "hello world");
}

#[test]
fn test_second_longer() {
    assert_eq!(longer("hi", "hello world"), "hello world");
}

#[test]
fn test_equal_length() {
    assert_eq!(longer("abc", "xyz"), "abc");
}
