use std::fmt;
use std::num::ParseIntError;

/// Custom error type with From implementations.
#[derive(Debug)]
pub enum ConfigError {
    MissingField(String),
    InvalidNumber(ParseIntError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingField(field) => write!(f, "missing field: {}", field),
            ConfigError::InvalidNumber(e) => write!(f, "invalid number: {}", e),
        }
    }
}

impl From<ParseIntError> for ConfigError {
    fn from(err: ParseIntError) -> Self {
        ConfigError::InvalidNumber(err)
    }
}

/// Parses a port number from a config map.
pub fn get_port(config: &[(&str, &str)]) -> Result<u16, ConfigError> {
    let value = config
        .iter()
        .find(|(k, _)| *k == "port")
        .map(|(_, v)| *v)
        .ok_or_else(|| ConfigError::MissingField(String::from("port")))?;
    let port: u16 = value.parse()?;
    Ok(port)
}
