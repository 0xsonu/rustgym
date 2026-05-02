use multiple_methods_trait::{Shape, Square, Triangle};

#[test]
fn test_square_area() {
    let s = Square { side: 5.0 };
    assert_eq!(s.area(), 25.0);
}

#[test]
fn test_square_perimeter() {
    let s = Square { side: 3.0 };
    assert_eq!(s.perimeter(), 12.0);
}

#[test]
fn test_square_description() {
    let s = Square { side: 4.0 };
    assert_eq!(s.description(), "Square with side 4");
}

#[test]
fn test_triangle_area() {
    let t = Triangle { base: 6.0, height: 4.0, side_a: 5.0, side_b: 5.0, side_c: 6.0 };
    assert_eq!(t.area(), 12.0);
}

#[test]
fn test_triangle_perimeter() {
    let t = Triangle { base: 3.0, height: 4.0, side_a: 3.0, side_b: 4.0, side_c: 5.0 };
    assert_eq!(t.perimeter(), 12.0);
}
