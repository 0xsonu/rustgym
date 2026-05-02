use std::fmt;

/// Custom error type for temperature conversion.
#[derive(Debug, PartialEq)]
pub enum TemperatureError {
    BelowAbsoluteZero,
    ParseError(String),
}

impl fmt::Display for TemperatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemperatureError::BelowAbsoluteZero => write!(f, "temperature below absolute zero"),
            TemperatureError::ParseError(msg) => write!(f, "parse error: {}", msg),
        }
    }
}

/// Converts Celsius to Fahrenheit, returning an error for invalid temperatures.
pub fn celsius_to_fahrenheit(input: &str) -> Result<f64, TemperatureError> {
    let celsius: f64 = input
        .parse()
        .map_err(|_| TemperatureError::ParseError(format!("'{}' is not a number", input)))?;
    if celsius < -273.15 {
        return Err(TemperatureError::BelowAbsoluteZero);
    }
    Ok(celsius * 9.0 / 5.0 + 32.0)
}
