use integer_types::integer_limits;

#[test]
fn test_u8_max() {
    let (u8_max, _, _) = integer_limits();
    assert_eq!(u8_max, 255);
}

#[test]
fn test_i8_max() {
    let (_, i8_max, _) = integer_limits();
    assert_eq!(i8_max, 127);
}

#[test]
fn test_u16_max() {
    let (_, _, u16_max) = integer_limits();
    assert_eq!(u16_max, 65535);
}
