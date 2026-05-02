use std::collections::BTreeMap;

/// Returns all entries with keys in the given range [from, to] inclusive.
pub fn range_query(map: &BTreeMap<i32, String>, from: i32, to: i32) -> Vec<(i32, String)> {
    map.range(from..=to)
        .map(|(&k, v)| (k, v.clone()))
        .collect()
}
