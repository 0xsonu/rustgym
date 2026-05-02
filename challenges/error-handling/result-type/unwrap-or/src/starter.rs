/// Parses a config value with a default fallback.
/// TODO: Use unwrap_or to provide a default when parsing fails.
pub fn get_config_value(input: &str, default: i32) -> i32 {
    todo!()
}

/// Parses a config value, computing default lazily.
/// TODO: Use unwrap_or_else to compute a default when parsing fails.
pub fn get_config_or_else(input: &str, compute_default: fn() -> i32) -> i32 {
    todo!()
}
