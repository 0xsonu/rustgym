/// Squares a number.
pub fn square(n: f64) -> f64 {
    n * n
}

/// Calculates the Euclidean distance between two points.
pub fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    (square(x2 - x1) + square(y2 - y1)).sqrt()
}
