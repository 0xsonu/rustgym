use word_count::word_count;

#[test]
fn test_basic() {
    let counts = word_count("hello world hello");
    assert_eq!(counts.get("hello"), Some(&2));
    assert_eq!(counts.get("world"), Some(&1));
}

#[test]
fn test_case_insensitive() {
    let counts = word_count("Hello hello HELLO");
    assert_eq!(counts.get("hello"), Some(&3));
}

#[test]
fn test_empty() {
    let counts = word_count("");
    assert!(counts.is_empty());
}
