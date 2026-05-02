use number_classifier::{classify, is_even};

#[test]
fn test_positive() {
    assert_eq!(classify(5), "positive");
}

#[test]
fn test_negative() {
    assert_eq!(classify(-3), "negative");
}

#[test]
fn test_zero() {
    assert_eq!(classify(0), "zero");
}

#[test]
fn test_even_numbers() {
    assert!(is_even(4));
    assert!(is_even(0));
    assert!(is_even(-2));
}

#[test]
fn test_odd_numbers() {
    assert!(!is_even(3));
    assert!(!is_even(-1));
}
