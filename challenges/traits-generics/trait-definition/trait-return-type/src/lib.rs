/// A trait for things that can be converted to a display string.
pub trait Displayable {
    fn display(&self) -> String;
}

pub struct Celsius(pub f64);
pub struct Fahrenheit(pub f64);

impl Displayable for Celsius {
    fn display(&self) -> String {
        format!("{:.1}°C", self.0)
    }
}

impl Displayable for Fahrenheit {
    fn display(&self) -> String {
        format!("{:.1}°F", self.0)
    }
}

/// Creates a Celsius value.
pub fn celsius(temp: f64) -> impl Displayable {
    Celsius(temp)
}

/// Creates a Fahrenheit value.
pub fn fahrenheit(temp: f64) -> impl Displayable {
    Fahrenheit(temp)
}
