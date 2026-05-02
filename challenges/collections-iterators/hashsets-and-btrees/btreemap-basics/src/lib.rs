use std::collections::BTreeMap;

/// Returns scores sorted by name, keeping highest score per name.
pub fn sorted_scores(scores: &[(String, u32)]) -> Vec<(String, u32)> {
    let mut map: BTreeMap<String, u32> = BTreeMap::new();
    for (name, score) in scores {
        let entry = map.entry(name.clone()).or_insert(0);
        if *score > *entry {
            *entry = *score;
        }
    }
    map.into_iter().collect()
}
