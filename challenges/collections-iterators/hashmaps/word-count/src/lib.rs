use std::collections::HashMap;

/// Counts the frequency of each word in the text.
pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let entry = counts.entry(word.to_lowercase()).or_insert(0);
        *entry += 1;
    }
    counts
}
