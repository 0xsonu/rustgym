use builder_pattern::EmailBuilder;

#[test]
fn test_build_email() {
    let email = EmailBuilder::new()
        .to("alice@example.com")
        .subject("Hello")
        .body("Hi there!")
        .build();
    assert_eq!(email.to, "alice@example.com");
    assert_eq!(email.subject, "Hello");
    assert_eq!(email.body, "Hi there!");
}

#[test]
fn test_different_order() {
    let email = EmailBuilder::new()
        .body("Content")
        .to("bob@test.com")
        .subject("Test")
        .build();
    assert_eq!(email.to, "bob@test.com");
    assert_eq!(email.subject, "Test");
    assert_eq!(email.body, "Content");
}

#[test]
fn test_empty_build() {
    let email = EmailBuilder::new().build();
    assert_eq!(email.to, "");
    assert_eq!(email.subject, "");
    assert_eq!(email.body, "");
}
