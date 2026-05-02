use nested_conditions::letter_grade;

#[test]
fn test_grade_a() {
    assert_eq!(letter_grade(95), "A");
}

#[test]
fn test_grade_b() {
    assert_eq!(letter_grade(85), "B");
}

#[test]
fn test_grade_c() {
    assert_eq!(letter_grade(72), "C");
}

#[test]
fn test_grade_d() {
    assert_eq!(letter_grade(65), "D");
}

#[test]
fn test_grade_f() {
    assert_eq!(letter_grade(50), "F");
}
