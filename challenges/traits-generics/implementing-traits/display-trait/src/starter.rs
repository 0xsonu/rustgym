use std::fmt;

/// A color with RGB values.
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// TODO: Implement fmt::Display for Color.
/// Format: "rgb(r, g, b)"
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// A point in 2D space.
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// TODO: Implement fmt::Display for Point.
/// Format: "(x, y)"
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
