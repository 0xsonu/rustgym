use temperature_converter::{celsius_to_fahrenheit, fahrenheit_to_celsius};

#[test]
fn test_boiling_point() {
    assert!((celsius_to_fahrenheit(100.0) - 212.0).abs() < 1e-10);
}

#[test]
fn test_freezing_point() {
    assert!((celsius_to_fahrenheit(0.0) - 32.0).abs() < 1e-10);
}

#[test]
fn test_body_temperature() {
    assert!((celsius_to_fahrenheit(37.0) - 98.6).abs() < 1e-10);
}

#[test]
fn test_fahrenheit_to_celsius_freezing() {
    assert!((fahrenheit_to_celsius(32.0) - 0.0).abs() < 1e-10);
}

#[test]
fn test_roundtrip() {
    let original = 25.0;
    let converted = fahrenheit_to_celsius(celsius_to_fahrenheit(original));
    assert!((converted - original).abs() < 1e-10);
}
