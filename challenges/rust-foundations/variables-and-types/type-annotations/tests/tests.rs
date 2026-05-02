use type_annotations::describe_types;

#[test]
fn test_integer_value() {
    let (integer, _, _, _) = describe_types();
    assert_eq!(integer, 42);
}

#[test]
fn test_float_value() {
    let (_, float, _, _) = describe_types();
    assert!((float - 3.14).abs() < f64::EPSILON);
}

#[test]
fn test_boolean_value() {
    let (_, _, boolean, _) = describe_types();
    assert!(boolean);
}

#[test]
fn test_char_value() {
    let (_, _, _, character) = describe_types();
    assert_eq!(character, 'R');
}
