/// A rectangle with width and height.
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

/// Creates a new Rectangle.
pub fn new_rectangle(width: f64, height: f64) -> Rectangle {
    Rectangle { width, height }
}

/// Returns the area of the rectangle.
pub fn area(rect: &Rectangle) -> f64 {
    rect.width * rect.height
}

/// Returns true if the rectangle is a square.
pub fn is_square(rect: &Rectangle) -> bool {
    (rect.width - rect.height).abs() < f64::EPSILON
}
