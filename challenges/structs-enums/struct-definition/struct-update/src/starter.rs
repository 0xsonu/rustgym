/// A configuration with various settings.
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub debug: bool,
}

/// Creates a default config.
/// TODO: Return a Config with host "localhost", port 8080, debug false.
pub fn default_config() -> Config {
    todo!()
}

/// Creates a new config with a different port, keeping other fields from the default.
/// TODO: Use struct update syntax (..config) to create a new Config with the given port.
pub fn with_port(config: Config, port: u16) -> Config {
    todo!()
}
