use basic_struct::create_user;

#[test]
fn test_create_user() {
    let user = create_user("alice", "alice@example.com");
    assert_eq!(user.username, "alice");
    assert_eq!(user.email, "alice@example.com");
    assert!(user.active);
}

#[test]
fn test_different_user() {
    let user = create_user("bob", "bob@test.com");
    assert_eq!(user.username, "bob");
    assert_eq!(user.email, "bob@test.com");
}

#[test]
fn test_active_default() {
    let user = create_user("test", "test@test.com");
    assert!(user.active);
}
