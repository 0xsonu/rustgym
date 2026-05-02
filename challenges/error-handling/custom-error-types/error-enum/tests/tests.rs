use error_enum::{celsius_to_fahrenheit, TemperatureError};

#[test]
fn test_valid_conversion() {
    assert_eq!(celsius_to_fahrenheit("100"), Ok(212.0));
}

#[test]
fn test_zero() {
    assert_eq!(celsius_to_fahrenheit("0"), Ok(32.0));
}

#[test]
fn test_below_absolute_zero() {
    assert_eq!(celsius_to_fahrenheit("-300"), Err(TemperatureError::BelowAbsoluteZero));
}

#[test]
fn test_parse_error() {
    assert!(matches!(celsius_to_fahrenheit("hot"), Err(TemperatureError::ParseError(_))));
}
