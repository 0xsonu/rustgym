/// Parses a "key=value" pair and returns the value as an integer.
pub fn parse_key_value(input: &str) -> Result<i32, String> {
    let parts: Vec<&str> = input.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(String::from("missing '=' delimiter"));
    }
    let value = parts[1]
        .trim()
        .parse::<i32>()
        .map_err(|e| format!("invalid value: {}", e))?;
    Ok(value)
}
