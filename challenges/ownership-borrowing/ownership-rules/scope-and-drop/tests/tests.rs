use scope_and_drop::{create_and_measure, string_length};

#[test]
fn test_string_length() {
    assert_eq!(string_length(), 14);
}

#[test]
fn test_create_and_measure_hello() {
    let (s, len) = create_and_measure("hello");
    assert_eq!(s, "hello");
    assert_eq!(len, 5);
}

#[test]
fn test_create_and_measure_empty() {
    let (s, len) = create_and_measure("");
    assert_eq!(s, "");
    assert_eq!(len, 0);
}

#[test]
fn test_create_and_measure_long() {
    let (s, len) = create_and_measure("Rust is great!");
    assert_eq!(s, "Rust is great!");
    assert_eq!(len, 14);
}
