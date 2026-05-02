/// Capitalizes the first letter and lowercases the rest.
pub fn capitalize_first(s: &mut String) {
    if s.is_empty() {
        return;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap().to_uppercase().to_string();
    let rest: String = chars.map(|c| c.to_lowercase().next().unwrap()).collect();
    s.clear();
    s.push_str(&first);
    s.push_str(&rest);
}
