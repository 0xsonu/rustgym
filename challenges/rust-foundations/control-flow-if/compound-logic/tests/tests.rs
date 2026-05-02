use compound_logic::is_leap_year;

#[test]
fn test_divisible_by_400() {
    assert!(is_leap_year(2000));
}

#[test]
fn test_divisible_by_100_not_400() {
    assert!(!is_leap_year(1900));
}

#[test]
fn test_divisible_by_4() {
    assert!(is_leap_year(2024));
}

#[test]
fn test_not_leap_year() {
    assert!(!is_leap_year(2023));
}
