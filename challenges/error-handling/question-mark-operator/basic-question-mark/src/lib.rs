use std::num::ParseIntError;

/// Parses two strings and returns their sum.
pub fn add_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x: i32 = a.parse()?;
    let y: i32 = b.parse()?;
    Ok(x + y)
}
