/// Classifies a number as "positive", "negative", or "zero".
pub fn classify(n: i32) -> String {
    if n > 0 {
        String::from("positive")
    } else if n < 0 {
        String::from("negative")
    } else {
        String::from("zero")
    }
}

/// Returns true if the number is even.
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}
