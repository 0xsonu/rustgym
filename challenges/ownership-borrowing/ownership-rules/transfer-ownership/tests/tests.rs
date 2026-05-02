use transfer_ownership::{take_and_append, take_and_uppercase};

#[test]
fn test_uppercase() {
    let s = String::from("hello");
    assert_eq!(take_and_uppercase(s), "HELLO");
}

#[test]
fn test_uppercase_mixed() {
    let s = String::from("Hello World");
    assert_eq!(take_and_uppercase(s), "HELLO WORLD");
}

#[test]
fn test_append() {
    let s = String::from("Hello");
    assert_eq!(take_and_append(s, " World"), "Hello World");
}

#[test]
fn test_append_empty() {
    let s = String::from("Rust");
    assert_eq!(take_and_append(s, ""), "Rust");
}
