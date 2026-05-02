use partial_eq_impl::Version;

#[test]
fn test_equal() {
    let v1 = Version::new(1, 2, 3);
    let v2 = Version::new(1, 2, 3);
    assert_eq!(v1, v2);
}

#[test]
fn test_not_equal() {
    let v1 = Version::new(1, 2, 3);
    let v2 = Version::new(1, 2, 4);
    assert_ne!(v1, v2);
}

#[test]
fn test_greater_major() {
    let v1 = Version::new(2, 0, 0);
    let v2 = Version::new(1, 9, 9);
    assert!(v1 > v2);
}

#[test]
fn test_greater_minor() {
    let v1 = Version::new(1, 3, 0);
    let v2 = Version::new(1, 2, 9);
    assert!(v1 > v2);
}

#[test]
fn test_less_than() {
    let v1 = Version::new(0, 1, 0);
    let v2 = Version::new(1, 0, 0);
    assert!(v1 < v2);
}
