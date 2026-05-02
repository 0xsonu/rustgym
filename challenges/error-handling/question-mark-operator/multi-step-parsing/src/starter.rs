#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Parses a point from "x,y" format.
/// TODO: Split at comma, parse both parts as f64 using ?, return Point.
pub fn parse_point(input: &str) -> Result<Point, String> {
    todo!()
}
