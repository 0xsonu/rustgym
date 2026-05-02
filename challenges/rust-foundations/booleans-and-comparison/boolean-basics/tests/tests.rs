use boolean_basics::check_age;

#[test]
fn test_adult() {
    assert!(check_age(18));
    assert!(check_age(25));
}

#[test]
fn test_minor() {
    assert!(!check_age(17));
    assert!(!check_age(0));
}

#[test]
fn test_boundary() {
    assert!(!check_age(17));
    assert!(check_age(18));
}
