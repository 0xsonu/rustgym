use std::collections::HashMap;

/// Groups words by their length.
pub fn group_by_length(words: &[&str]) -> HashMap<usize, Vec<String>> {
    let mut groups: HashMap<usize, Vec<String>> = HashMap::new();
    for word in words {
        groups.entry(word.len())
            .or_insert_with(Vec::new)
            .push(word.to_string());
    }
    groups
}
