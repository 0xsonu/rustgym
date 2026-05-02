/// A 2D point that implements Copy.
/// TODO: Add #[derive(Debug, Clone, Copy, PartialEq)] to make Point copyable.
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Creates a point and returns it along with a translated copy.
/// TODO: Create a new Point with dx/dy added, return both (original is still valid because Copy).
pub fn translate(point: Point, dx: f64, dy: f64) -> (Point, Point) {
    todo!()
}

/// Returns the distance between two points.
/// TODO: Use the distance formula: sqrt((x2-x1)^2 + (y2-y1)^2)
pub fn distance(a: Point, b: Point) -> f64 {
    todo!()
}
