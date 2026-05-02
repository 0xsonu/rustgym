use helper_functions::{square, distance};

#[test]
fn test_square() {
    assert_eq!(square(3.0), 9.0);
}

#[test]
fn test_distance_horizontal() {
    assert!((distance(0.0, 0.0, 3.0, 0.0) - 3.0).abs() < 1e-10);
}

#[test]
fn test_distance_diagonal() {
    assert!((distance(0.0, 0.0, 3.0, 4.0) - 5.0).abs() < 1e-10);
}

#[test]
fn test_distance_same_point() {
    assert!((distance(2.0, 3.0, 2.0, 3.0) - 0.0).abs() < 1e-10);
}
