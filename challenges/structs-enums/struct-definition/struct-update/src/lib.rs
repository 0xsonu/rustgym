/// A configuration with various settings.
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub debug: bool,
}

/// Creates a default config.
pub fn default_config() -> Config {
    Config {
        host: String::from("localhost"),
        port: 8080,
        debug: false,
    }
}

/// Creates a new config with a different port, keeping other fields from the default.
pub fn with_port(config: Config, port: u16) -> Config {
    Config { port, ..config }
}
