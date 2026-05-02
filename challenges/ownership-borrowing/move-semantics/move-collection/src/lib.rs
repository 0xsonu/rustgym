/// Filters a Vec of Strings, keeping only those longer than min_length.
pub fn filter_long_strings(strings: Vec<String>, min_length: usize) -> Vec<String> {
    strings.into_iter().filter(|s| s.len() > min_length).collect()
}

/// Transforms a Vec of Strings to uppercase.
pub fn uppercase_all(strings: Vec<String>) -> Vec<String> {
    strings.into_iter().map(|s| s.to_uppercase()).collect()
}
