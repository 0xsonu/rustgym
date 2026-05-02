/// Parses a config value with a default fallback.
pub fn get_config_value(input: &str, default: i32) -> i32 {
    input.parse().unwrap_or(default)
}

/// Parses a config value, computing default lazily.
pub fn get_config_or_else(input: &str, compute_default: fn() -> i32) -> i32 {
    input.parse().unwrap_or_else(|_| compute_default())
}
