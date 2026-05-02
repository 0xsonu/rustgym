use basic_loop::find_first_power_of_two_above;

#[test]
fn test_above_10() {
    assert_eq!(find_first_power_of_two_above(10), 16);
}

#[test]
fn test_above_1() {
    assert_eq!(find_first_power_of_two_above(1), 2);
}

#[test]
fn test_above_100() {
    assert_eq!(find_first_power_of_two_above(100), 128);
}

#[test]
fn test_above_0() {
    assert_eq!(find_first_power_of_two_above(0), 1);
}
