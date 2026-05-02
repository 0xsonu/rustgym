use basic_move::move_and_return;

#[test]
fn test_basic_move() {
    let s = String::from("hello");
    assert_eq!(move_and_return(s), "Moved: hello");
}

#[test]
fn test_move_empty() {
    let s = String::from("");
    assert_eq!(move_and_return(s), "Moved: ");
}

#[test]
fn test_move_long_string() {
    let s = String::from("Rust is awesome");
    assert_eq!(move_and_return(s), "Moved: Rust is awesome");
}
