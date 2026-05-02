use return_ownership::{give_ownership, take_and_give_back};

#[test]
fn test_give_ownership() {
    let s = give_ownership();
    assert_eq!(s, "yours now");
}

#[test]
fn test_take_and_give_back() {
    let s = String::from("hello");
    let (returned, len) = take_and_give_back(s);
    assert_eq!(returned, "hello");
    assert_eq!(len, 5);
}

#[test]
fn test_take_and_give_back_empty() {
    let s = String::from("");
    let (returned, len) = take_and_give_back(s);
    assert_eq!(returned, "");
    assert_eq!(len, 0);
}
