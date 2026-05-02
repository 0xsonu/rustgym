use multiple_bindings::personal_info;

#[test]
fn test_name() {
    let (name, _, _) = personal_info();
    assert_eq!(name, "Alice");
}

#[test]
fn test_age() {
    let (_, age, _) = personal_info();
    assert_eq!(age, 30);
}

#[test]
fn test_student_status() {
    let (_, _, is_student) = personal_info();
    assert!(is_student);
}

#[test]
fn test_all_together() {
    let info = personal_info();
    assert_eq!(info, (String::from("Alice"), 30, true));
}
