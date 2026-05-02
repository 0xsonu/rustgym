use basic_reference::string_length;

#[test]
fn test_hello() {
    let s = String::from("hello");
    assert_eq!(string_length(&s), 5);
    // s is still usable after borrowing
    assert_eq!(s, "hello");
}

#[test]
fn test_empty() {
    let s = String::from("");
    assert_eq!(string_length(&s), 0);
}

#[test]
fn test_long_string() {
    let s = String::from("Rust is awesome!");
    assert_eq!(string_length(&s), 16);
}
