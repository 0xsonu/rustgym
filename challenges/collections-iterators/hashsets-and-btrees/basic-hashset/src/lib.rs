use std::collections::HashSet;

/// Returns all unique words from text, sorted alphabetically.
pub fn unique_words(text: &str) -> Vec<String> {
    let set: HashSet<String> = text
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .collect();
    let mut words: Vec<String> = set.into_iter().collect();
    words.sort();
    words
}
