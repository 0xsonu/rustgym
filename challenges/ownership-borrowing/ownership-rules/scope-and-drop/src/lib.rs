/// Returns the length of a string created inside the function.
/// Demonstrates that values are dropped when they go out of scope.
pub fn string_length() -> usize {
    let s = String::from("Rust ownership");
    s.len()
}

/// Creates a string, gets its length, then returns both.
pub fn create_and_measure(text: &str) -> (String, usize) {
    let s = String::from(text);
    let len = s.len();
    (s, len)
}
