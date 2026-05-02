/// A circle with a radius.
pub struct Circle {
    pub radius: f64,
}

impl Circle {
    /// Creates a new Circle with the given radius.
    pub fn new(radius: f64) -> Self {
        Circle { radius }
    }

    /// Returns the area of the circle (π * r²).
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    /// Returns the circumference of the circle (2 * π * r).
    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}
