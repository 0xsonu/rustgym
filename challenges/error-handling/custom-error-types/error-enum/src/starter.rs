use std::fmt;

/// Custom error type for temperature conversion.
/// TODO: Define variants BelowAbsoluteZero and ParseError(String).
#[derive(Debug, PartialEq)]
pub enum TemperatureError {
    // Add variants here
}

// TODO: Implement Display for TemperatureError.
impl fmt::Display for TemperatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// Converts Celsius to Fahrenheit, returning an error for invalid temperatures.
/// TODO: Parse input, check for absolute zero, convert.
pub fn celsius_to_fahrenheit(input: &str) -> Result<f64, TemperatureError> {
    todo!()
}
