use std::collections::HashMap;

/// Finds the most frequently occurring element.
pub fn most_frequent(items: &[&str]) -> Option<String> {
    if items.is_empty() {
        return None;
    }
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for &item in items {
        *counts.entry(item).or_insert(0) += 1;
    }
    counts.into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(item, _)| item.to_string())
}
