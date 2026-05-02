/// Represents a point in 2D space.
#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Parses a point from "x,y" format.
pub fn parse_point(input: &str) -> Result<Point, String> {
    let parts: Vec<&str> = input.split(',').collect();
    if parts.len() != 2 {
        return Err(String::from("expected format 'x,y'"));
    }
    let x: f64 = parts[0].trim().parse().map_err(|_| format!("invalid x: '{}'", parts[0].trim()))?;
    let y: f64 = parts[1].trim().parse().map_err(|_| format!("invalid y: '{}'", parts[1].trim()))?;
    Ok(Point { x, y })
}
