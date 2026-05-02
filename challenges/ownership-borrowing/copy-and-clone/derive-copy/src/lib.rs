/// A 2D point that implements Copy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Creates a point and returns it along with a translated copy.
pub fn translate(point: Point, dx: f64, dy: f64) -> (Point, Point) {
    let translated = Point {
        x: point.x + dx,
        y: point.y + dy,
    };
    (point, translated)
}

/// Returns the distance between two points.
pub fn distance(a: Point, b: Point) -> f64 {
    ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt()
}
