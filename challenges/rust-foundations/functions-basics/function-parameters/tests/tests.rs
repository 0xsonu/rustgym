use function_parameters::format_price;

#[test]
fn test_basic_price() {
    assert_eq!(format_price("Apple", 1.50), "Apple: $1.50");
}

#[test]
fn test_whole_number() {
    assert_eq!(format_price("Book", 10.00), "Book: $10.00");
}

#[test]
fn test_long_decimal() {
    assert_eq!(format_price("Gas", 3.999), "Gas: $4.00");
}

#[test]
fn test_zero_price() {
    assert_eq!(format_price("Free Item", 0.0), "Free Item: $0.00");
}
