use basic_if_else::sign;

#[test]
fn test_positive() {
    assert_eq!(sign(5), "positive");
}

#[test]
fn test_negative() {
    assert_eq!(sign(-3), "negative");
}

#[test]
fn test_zero() {
    assert_eq!(sign(0), "zero");
}

#[test]
fn test_large_positive() {
    assert_eq!(sign(1000), "positive");
}
