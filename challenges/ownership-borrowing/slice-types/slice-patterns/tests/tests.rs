use slice_patterns::middle_elements;

#[test]
fn test_normal() {
    assert_eq!(middle_elements(&[1, 2, 3, 4, 5]), &[2, 3, 4]);
}

#[test]
fn test_three_elements() {
    assert_eq!(middle_elements(&[1, 2, 3]), &[2]);
}

#[test]
fn test_too_short() {
    assert_eq!(middle_elements(&[1, 2]), &[] as &[i32]);
}

#[test]
fn test_empty() {
    assert_eq!(middle_elements(&[]), &[] as &[i32]);
}
