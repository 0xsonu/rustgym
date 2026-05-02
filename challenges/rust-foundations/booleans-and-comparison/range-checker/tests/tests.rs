use range_checker::{in_range, is_leap_year};

#[test]
fn test_in_range_inside() {
    assert!(in_range(5, 1, 10));
}

#[test]
fn test_in_range_boundaries() {
    assert!(in_range(1, 1, 10));
    assert!(in_range(10, 1, 10));
}

#[test]
fn test_in_range_outside() {
    assert!(!in_range(0, 1, 10));
    assert!(!in_range(11, 1, 10));
}

#[test]
fn test_leap_year() {
    assert!(is_leap_year(2000));
    assert!(is_leap_year(2024));
    assert!(is_leap_year(2004));
}

#[test]
fn test_not_leap_year() {
    assert!(!is_leap_year(1900));
    assert!(!is_leap_year(2023));
    assert!(!is_leap_year(2100));
}
