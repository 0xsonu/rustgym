use std::collections::HashMap;

/// Returns character frequencies sorted by count (desc), then alphabetically.
pub fn char_frequency(text: &str) -> Vec<(char, usize)> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in text.chars() {
        if !c.is_whitespace() {
            *counts.entry(c).or_insert(0) += 1;
        }
    }
    let mut result: Vec<(char, usize)> = counts.into_iter().collect();
    result.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    result
}
