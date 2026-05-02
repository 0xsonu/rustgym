use associated_functions::Color;

#[test]
fn test_new() {
    let c = Color::new(128, 64, 32);
    assert_eq!(c.r, 128);
    assert_eq!(c.g, 64);
    assert_eq!(c.b, 32);
}

#[test]
fn test_red() {
    let c = Color::red();
    assert_eq!((c.r, c.g, c.b), (255, 0, 0));
}

#[test]
fn test_green() {
    let c = Color::green();
    assert_eq!((c.r, c.g, c.b), (0, 255, 0));
}

#[test]
fn test_blue() {
    let c = Color::blue();
    assert_eq!((c.r, c.g, c.b), (0, 0, 255));
}

#[test]
fn test_to_hex() {
    assert_eq!(Color::red().to_hex(), "#FF0000");
    assert_eq!(Color::new(0, 0, 0).to_hex(), "#000000");
    assert_eq!(Color::new(171, 205, 239).to_hex(), "#ABCDEF");
}
