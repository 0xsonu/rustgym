use basic_hashset::unique_words;

#[test]
fn test_duplicates() {
    assert_eq!(unique_words("hello world hello"), vec!["hello", "world"]);
}

#[test]
fn test_case_insensitive() {
    assert_eq!(unique_words("Hello hello HELLO"), vec!["hello"]);
}

#[test]
fn test_empty() {
    assert_eq!(unique_words(""), Vec::<String>::new());
}

#[test]
fn test_sorted() {
    assert_eq!(unique_words("banana apple cherry"), vec!["apple", "banana", "cherry"]);
}
