use logical_operators::can_enter;

#[test]
fn test_adult_with_ticket() {
    assert!(can_enter(21, true));
}

#[test]
fn test_adult_without_ticket() {
    assert!(!can_enter(21, false));
}

#[test]
fn test_minor_with_ticket() {
    assert!(!can_enter(16, true));
}

#[test]
fn test_minor_without_ticket() {
    assert!(!can_enter(16, false));
}
