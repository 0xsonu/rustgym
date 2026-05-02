use basic_impl::Circle;

#[test]
fn test_new() {
    let c = Circle::new(5.0);
    assert_eq!(c.radius, 5.0);
}

#[test]
fn test_area() {
    let c = Circle::new(1.0);
    assert!((c.area() - std::f64::consts::PI).abs() < 1e-10);
}

#[test]
fn test_area_radius_2() {
    let c = Circle::new(2.0);
    assert!((c.area() - 4.0 * std::f64::consts::PI).abs() < 1e-10);
}

#[test]
fn test_circumference() {
    let c = Circle::new(1.0);
    assert!((c.circumference() - 2.0 * std::f64::consts::PI).abs() < 1e-10);
}
