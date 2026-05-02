# Merge Maps

## Concept

Merging HashMaps requires handling duplicate keys. You can iterate over one map and insert/update entries in another.

```rust
for (key, value) in &other_map {
    *merged.entry(key.clone()).or_insert(0) += value;
}
```

## Task

Implement `merge_maps(a: &HashMap<String, i32>, b: &HashMap<String, i32>) -> HashMap<String, i32>` that merges two maps, summing values for duplicate keys.

## Requirements

- Include all keys from both maps
- Sum values when a key exists in both
- Return a new HashMap

## XP Reward

75 XP (Easy)
