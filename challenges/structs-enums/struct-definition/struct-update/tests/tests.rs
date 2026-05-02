use struct_update::{default_config, with_port};

#[test]
fn test_default_config() {
    let config = default_config();
    assert_eq!(config.host, "localhost");
    assert_eq!(config.port, 8080);
    assert!(!config.debug);
}

#[test]
fn test_with_port() {
    let config = default_config();
    let updated = with_port(config, 3000);
    assert_eq!(updated.port, 3000);
    assert_eq!(updated.host, "localhost");
    assert!(!updated.debug);
}

#[test]
fn test_with_different_port() {
    let config = default_config();
    let updated = with_port(config, 443);
    assert_eq!(updated.port, 443);
}
