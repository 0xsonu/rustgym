use comparison_chain::grade;

#[test]
fn test_grade_a() {
    assert_eq!(grade(95), "A");
    assert_eq!(grade(90), "A");
    assert_eq!(grade(100), "A");
}

#[test]
fn test_grade_b() {
    assert_eq!(grade(85), "B");
    assert_eq!(grade(80), "B");
}

#[test]
fn test_grade_c() {
    assert_eq!(grade(75), "C");
    assert_eq!(grade(70), "C");
}

#[test]
fn test_grade_d() {
    assert_eq!(grade(65), "D");
    assert_eq!(grade(60), "D");
}

#[test]
fn test_grade_f() {
    assert_eq!(grade(59), "F");
    assert_eq!(grade(0), "F");
}
