use multiple_traits::Duration;

#[test]
fn test_display_simple() {
    let d = Duration::new(3661);
    assert_eq!(format!("{}", d), "01:01:01");
}

#[test]
fn test_display_hours() {
    let d = Duration::from_hours(2);
    assert_eq!(format!("{}", d), "02:00:00");
}

#[test]
fn test_display_minutes() {
    let d = Duration::from_minutes(90);
    assert_eq!(format!("{}", d), "01:30:00");
}

#[test]
fn test_equality() {
    let d1 = Duration::from_minutes(60);
    let d2 = Duration::from_hours(1);
    assert_eq!(d1, d2);
}

#[test]
fn test_ordering() {
    let d1 = Duration::new(100);
    let d2 = Duration::new(200);
    assert!(d1 < d2);
}
