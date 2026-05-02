/// Takes ownership of a String and returns its uppercase version.
pub fn take_and_uppercase(s: String) -> String {
    s.to_uppercase()
}

/// Takes ownership of a String, appends a suffix, and returns it.
pub fn take_and_append(mut s: String, suffix: &str) -> String {
    s.push_str(suffix);
    s
}
