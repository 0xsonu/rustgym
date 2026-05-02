use move_collection::{filter_long_strings, uppercase_all};

#[test]
fn test_filter_long() {
    let strings = vec![
        String::from("hi"),
        String::from("hello"),
        String::from("hey"),
    ];
    let result = filter_long_strings(strings, 2);
    assert_eq!(result, vec!["hello", "hey"]);
}

#[test]
fn test_filter_none_match() {
    let strings = vec![String::from("a"), String::from("b")];
    let result = filter_long_strings(strings, 5);
    assert!(result.is_empty());
}

#[test]
fn test_uppercase_all() {
    let strings = vec![String::from("hello"), String::from("world")];
    let result = uppercase_all(strings);
    assert_eq!(result, vec!["HELLO", "WORLD"]);
}

#[test]
fn test_uppercase_empty() {
    let strings: Vec<String> = vec![];
    let result = uppercase_all(strings);
    assert!(result.is_empty());
}
