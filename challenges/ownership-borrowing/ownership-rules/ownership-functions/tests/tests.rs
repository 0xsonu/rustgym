use ownership_functions::{double_values, join_strings};

#[test]
fn test_double_values() {
    assert_eq!(double_values(vec![1, 2, 3]), vec![2, 4, 6]);
}

#[test]
fn test_double_empty() {
    assert_eq!(double_values(vec![]), Vec::<i32>::new());
}

#[test]
fn test_double_negative() {
    assert_eq!(double_values(vec![-1, -2]), vec![-2, -4]);
}

#[test]
fn test_join_strings() {
    let a = String::from("Hello");
    let b = String::from("World");
    assert_eq!(join_strings(a, b), "Hello World");
}

#[test]
fn test_join_empty() {
    let a = String::from("");
    let b = String::from("Rust");
    assert_eq!(join_strings(a, b), " Rust");
}
