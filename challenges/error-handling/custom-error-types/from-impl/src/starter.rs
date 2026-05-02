use std::fmt;
use std::num::ParseIntError;

/// Custom error type with From implementations.
/// TODO: Define MissingField(String) and InvalidNumber(ParseIntError) variants.
#[derive(Debug)]
pub enum ConfigError {
    // Add variants here
}

// TODO: Implement Display for ConfigError.
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

// TODO: Implement From<ParseIntError> for ConfigError.
impl From<ParseIntError> for ConfigError {
    fn from(err: ParseIntError) -> Self {
        todo!()
    }
}

/// Parses a port number from a config map.
/// TODO: Find "port" key, parse value, use ? to propagate errors.
pub fn get_port(config: &[(&str, &str)]) -> Result<u16, ConfigError> {
    todo!()
}
