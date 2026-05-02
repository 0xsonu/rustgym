use return_values::{max_of_two, is_even};

#[test]
fn test_max_first_larger() {
    assert_eq!(max_of_two(10, 5), 10);
}

#[test]
fn test_max_second_larger() {
    assert_eq!(max_of_two(3, 8), 8);
}

#[test]
fn test_max_equal() {
    assert_eq!(max_of_two(4, 4), 4);
}

#[test]
fn test_is_even_true() {
    assert!(is_even(4));
}

#[test]
fn test_is_even_false() {
    assert!(!is_even(7));
}
