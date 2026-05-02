use single_owner::create_owned_string;

#[test]
fn test_returns_correct_string() {
    assert_eq!(create_owned_string(), "I am owned!");
}

#[test]
fn test_returns_owned_string() {
    let s = create_owned_string();
    // We can modify it because we own it
    let mut s = s;
    s.push_str(" And modified!");
    assert!(s.contains("I am owned!"));
}

#[test]
fn test_each_call_creates_new() {
    let s1 = create_owned_string();
    let s2 = create_owned_string();
    assert_eq!(s1, s2);
}
