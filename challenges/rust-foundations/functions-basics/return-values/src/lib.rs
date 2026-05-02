/// Returns the larger of two values.
pub fn max_of_two(a: i32, b: i32) -> i32 {
    if a >= b { a } else { b }
}

/// Returns true if the number is even.
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}
