/// Parses a string into a valid age (0-150).
pub fn parse_age(input: &str) -> Result<u8, String> {
    let age: u8 = input
        .parse()
        .map_err(|_| format!("'{}' is not a valid number", input))?;
    if age > 150 {
        Err(format!("age {} is out of range (0-150)", age))
    } else {
        Ok(age)
    }
}
