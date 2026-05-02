# BTreeMap Basics

## Concept

`BTreeMap<K, V>` is like HashMap but keeps keys sorted. Iteration always yields entries in key order.

```rust
use std::collections::BTreeMap;
let mut map = BTreeMap::new();
map.insert("banana", 2);
map.insert("apple", 5);
// Iterates in order: apple, banana
```

## Task

Implement `sorted_scores(scores: &[(String, u32)]) -> Vec<(String, u32)>` that returns scores sorted by name.

## Requirements

- Insert all scores into a BTreeMap
- If a name appears multiple times, keep the highest score
- Return entries sorted by name

## XP Reward

75 XP (Easy)
