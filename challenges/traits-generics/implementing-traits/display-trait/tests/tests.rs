use display_trait::{Color, Point};

#[test]
fn test_color_display() {
    let c = Color { r: 255, g: 128, b: 0 };
    assert_eq!(format!("{}", c), "rgb(255, 128, 0)");
}

#[test]
fn test_color_black() {
    let c = Color { r: 0, g: 0, b: 0 };
    assert_eq!(format!("{}", c), "rgb(0, 0, 0)");
}

#[test]
fn test_point_display() {
    let p = Point { x: 3.5, y: -2.1 };
    assert_eq!(format!("{}", p), "(3.5, -2.1)");
}

#[test]
fn test_point_origin() {
    let p = Point { x: 0.0, y: 0.0 };
    assert_eq!(format!("{}", p), "(0, 0)");
}
