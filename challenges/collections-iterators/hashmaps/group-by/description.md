# Group By

## Concept

HashMaps are great for grouping items by a key. Use `entry().or_insert_with(Vec::new)` to build groups.

```rust
let mut groups: HashMap<char, Vec<&str>> = HashMap::new();
groups.entry('a').or_insert_with(Vec::new).push("apple");
```

## Task

Implement `group_by_length(words: &[&str]) -> HashMap<usize, Vec<String>>` that groups words by their length.

## Requirements

- Group words into buckets by string length
- Each bucket is a Vec of words with that length
- Preserve insertion order within each group

## XP Reward

50 XP (Beginner)
