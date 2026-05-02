use trait_return_type::{celsius, fahrenheit, Displayable};

#[test]
fn test_celsius_display() {
    let c = celsius(100.0);
    assert_eq!(c.display(), "100.0°C");
}

#[test]
fn test_fahrenheit_display() {
    let f = fahrenheit(212.0);
    assert_eq!(f.display(), "212.0°F");
}

#[test]
fn test_celsius_negative() {
    let c = celsius(-40.0);
    assert_eq!(c.display(), "-40.0°C");
}

#[test]
fn test_fahrenheit_fractional() {
    let f = fahrenheit(98.6);
    assert_eq!(f.display(), "98.6°F");
}
