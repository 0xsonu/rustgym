/// Returns true if value is within the range [min, max] (inclusive).
pub fn in_range(value: i32, min: i32, max: i32) -> bool {
    value >= min && value <= max
}

/// Returns true if the year is a leap year.
pub fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
