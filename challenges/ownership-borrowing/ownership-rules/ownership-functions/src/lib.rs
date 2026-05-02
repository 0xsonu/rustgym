/// Takes ownership of a Vec, doubles each element, and returns the new Vec.
pub fn double_values(values: Vec<i32>) -> Vec<i32> {
    values.into_iter().map(|v| v * 2).collect()
}

/// Takes ownership of two Strings and concatenates them with a space.
pub fn join_strings(a: String, b: String) -> String {
    format!("{} {}", a, b)
}
