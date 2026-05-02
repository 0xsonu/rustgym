/// A trait for things that can be converted to a display string.
pub trait Displayable {
    fn display(&self) -> String;
}

pub struct Celsius(pub f64);
pub struct Fahrenheit(pub f64);

/// TODO: Implement Displayable for Celsius. Return "{value:.1}°C".
impl Displayable for Celsius {
    fn display(&self) -> String {
        todo!()
    }
}

/// TODO: Implement Displayable for Fahrenheit. Return "{value:.1}°F".
impl Displayable for Fahrenheit {
    fn display(&self) -> String {
        todo!()
    }
}

/// Creates a Celsius value.
/// TODO: Return a Celsius instance (the return type is impl Displayable).
pub fn celsius(temp: f64) -> impl Displayable {
    todo!();
    #[allow(unreachable_code)]
    Celsius(temp)
}

/// Creates a Fahrenheit value.
/// TODO: Return a Fahrenheit instance (the return type is impl Displayable).
pub fn fahrenheit(temp: f64) -> impl Displayable {
    todo!();
    #[allow(unreachable_code)]
    Fahrenheit(temp)
}
