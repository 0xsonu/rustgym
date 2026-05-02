use basic_mut_ref::append_world;

#[test]
fn test_hello() {
    let mut s = String::from("hello");
    append_world(&mut s);
    assert_eq!(s, "hello world");
}

#[test]
fn test_empty() {
    let mut s = String::from("");
    append_world(&mut s);
    assert_eq!(s, " world");
}

#[test]
fn test_other_string() {
    let mut s = String::from("brave new");
    append_world(&mut s);
    assert_eq!(s, "brave new world");
}
