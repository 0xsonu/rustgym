/// Converts Celsius to Fahrenheit: F = C * 9/5 + 32
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

/// Converts Fahrenheit to Celsius: C = (F - 32) * 5/9
pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
