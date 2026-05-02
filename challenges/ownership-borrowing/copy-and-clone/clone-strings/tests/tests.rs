use clone_strings::{clone_and_modify, double_greeting};

#[test]
fn test_clone_and_modify() {
    let s = String::from("hello");
    let (orig, cloned) = clone_and_modify(s);
    assert_eq!(orig, "hello");
    assert_eq!(cloned, "hello");
}

#[test]
fn test_clone_independence() {
    let s = String::from("test");
    let (orig, cloned) = clone_and_modify(s);
    assert_eq!(orig, cloned);
}

#[test]
fn test_double_greeting() {
    let name = String::from("Alice");
    let (formal, casual) = double_greeting(name);
    assert_eq!(formal, "Hello, Alice!");
    assert_eq!(casual, "Hey, Alice!");
}
