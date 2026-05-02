use struct_with_methods::{area, is_square, new_rectangle};

#[test]
fn test_new_rectangle() {
    let r = new_rectangle(5.0, 3.0);
    assert_eq!(r.width, 5.0);
    assert_eq!(r.height, 3.0);
}

#[test]
fn test_area() {
    let r = new_rectangle(4.0, 6.0);
    assert_eq!(area(&r), 24.0);
}

#[test]
fn test_area_square() {
    let r = new_rectangle(5.0, 5.0);
    assert_eq!(area(&r), 25.0);
}

#[test]
fn test_is_square_true() {
    let r = new_rectangle(4.0, 4.0);
    assert!(is_square(&r));
}

#[test]
fn test_is_square_false() {
    let r = new_rectangle(4.0, 5.0);
    assert!(!is_square(&r));
}
