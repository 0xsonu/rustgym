/// Validates a username through multiple checks.
pub fn validate_username(input: &str) -> Result<String, String> {
    let trimmed = input.trim();

    if trimmed.len() < 3 {
        return Err(String::from("username must be at least 3 characters"));
    }
    if trimmed.len() > 20 {
        return Err(String::from("username must be at most 20 characters"));
    }
    if !trimmed.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(String::from("username must contain only alphanumeric characters and underscores"));
    }

    Ok(trimmed.to_lowercase())
}
