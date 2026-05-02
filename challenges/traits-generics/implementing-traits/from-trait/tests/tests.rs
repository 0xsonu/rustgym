use from_trait::{Celsius, Fahrenheit};

#[test]
fn test_celsius_to_fahrenheit() {
    let f: Fahrenheit = Celsius(100.0).into();
    assert!((f.0 - 212.0).abs() < 1e-10);
}

#[test]
fn test_freezing_point() {
    let f: Fahrenheit = Celsius(0.0).into();
    assert!((f.0 - 32.0).abs() < 1e-10);
}

#[test]
fn test_fahrenheit_to_celsius() {
    let c: Celsius = Fahrenheit(212.0).into();
    assert!((c.0 - 100.0).abs() < 1e-10);
}

#[test]
fn test_roundtrip() {
    let original = Celsius(37.0);
    let f: Fahrenheit = Celsius(37.0).into();
    let back: Celsius = f.into();
    assert!((back.0 - original.0).abs() < 1e-10);
}
