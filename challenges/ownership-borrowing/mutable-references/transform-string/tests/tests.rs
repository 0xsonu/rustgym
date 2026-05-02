use transform_string::capitalize_first;

#[test]
fn test_lowercase() {
    let mut s = String::from("hello");
    capitalize_first(&mut s);
    assert_eq!(s, "Hello");
}

#[test]
fn test_all_caps() {
    let mut s = String::from("WORLD");
    capitalize_first(&mut s);
    assert_eq!(s, "World");
}

#[test]
fn test_empty() {
    let mut s = String::from("");
    capitalize_first(&mut s);
    assert_eq!(s, "");
}

#[test]
fn test_single_char() {
    let mut s = String::from("a");
    capitalize_first(&mut s);
    assert_eq!(s, "A");
}
