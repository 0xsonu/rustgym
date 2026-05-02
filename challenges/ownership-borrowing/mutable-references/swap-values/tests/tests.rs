use swap_values::swap;

#[test]
fn test_swap_basic() {
    let mut a = 1;
    let mut b = 2;
    swap(&mut a, &mut b);
    assert_eq!(a, 2);
    assert_eq!(b, 1);
}

#[test]
fn test_swap_same() {
    let mut a = 5;
    let mut b = 5;
    swap(&mut a, &mut b);
    assert_eq!(a, 5);
    assert_eq!(b, 5);
}

#[test]
fn test_swap_negative() {
    let mut a = -10;
    let mut b = 20;
    swap(&mut a, &mut b);
    assert_eq!(a, 20);
    assert_eq!(b, -10);
}
