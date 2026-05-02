/// Temperature in Celsius.
pub struct Celsius(pub f64);

/// Temperature in Fahrenheit.
pub struct Fahrenheit(pub f64);

/// TODO: Implement From<Celsius> for Fahrenheit.
/// Formula: F = C * 9/5 + 32
impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        todo!()
    }
}

/// TODO: Implement From<Fahrenheit> for Celsius.
/// Formula: C = (F - 32) * 5/9
impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        todo!()
    }
}
