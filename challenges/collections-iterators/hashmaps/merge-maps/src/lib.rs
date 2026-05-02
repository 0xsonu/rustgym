use std::collections::HashMap;

/// Merges two HashMaps, summing values for duplicate keys.
pub fn merge_maps(a: &HashMap<String, i32>, b: &HashMap<String, i32>) -> HashMap<String, i32> {
    let mut result = a.clone();
    for (key, value) in b {
        *result.entry(key.clone()).or_insert(0) += value;
    }
    result
}
