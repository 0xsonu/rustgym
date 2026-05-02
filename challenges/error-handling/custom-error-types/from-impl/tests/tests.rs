use from_impl::get_port;

#[test]
fn test_valid_port() {
    let config = vec![("host", "localhost"), ("port", "8080")];
    assert_eq!(get_port(&config).unwrap(), 8080);
}

#[test]
fn test_missing_port() {
    let config = vec![("host", "localhost")];
    assert!(get_port(&config).is_err());
}

#[test]
fn test_invalid_port() {
    let config = vec![("port", "abc")];
    assert!(get_port(&config).is_err());
}
