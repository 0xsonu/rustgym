use derive_copy::{distance, translate, Point};

#[test]
fn test_translate() {
    let p = Point { x: 1.0, y: 2.0 };
    let (original, translated) = translate(p, 3.0, 4.0);
    assert_eq!(original, Point { x: 1.0, y: 2.0 });
    assert_eq!(translated, Point { x: 4.0, y: 6.0 });
}

#[test]
fn test_translate_zero() {
    let p = Point { x: 5.0, y: 5.0 };
    let (original, translated) = translate(p, 0.0, 0.0);
    assert_eq!(original, translated);
}

#[test]
fn test_distance_basic() {
    let a = Point { x: 0.0, y: 0.0 };
    let b = Point { x: 3.0, y: 4.0 };
    assert!((distance(a, b) - 5.0).abs() < 1e-10);
}

#[test]
fn test_distance_same_point() {
    let a = Point { x: 2.0, y: 3.0 };
    assert!((distance(a, a) - 0.0).abs() < 1e-10);
}
