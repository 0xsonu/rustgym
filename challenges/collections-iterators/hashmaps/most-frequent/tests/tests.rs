use most_frequent::most_frequent;

#[test]
fn test_basic() {
    assert_eq!(most_frequent(&["a", "b", "a", "c", "a"]), Some(String::from("a")));
}

#[test]
fn test_empty() {
    assert_eq!(most_frequent(&[]), None);
}

#[test]
fn test_single() {
    assert_eq!(most_frequent(&["only"]), Some(String::from("only")));
}

#[test]
fn test_tie() {
    let result = most_frequent(&["a", "b", "a", "b"]);
    assert!(result == Some(String::from("a")) || result == Some(String::from("b")));
}
