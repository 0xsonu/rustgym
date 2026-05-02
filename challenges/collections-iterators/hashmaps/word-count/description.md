# Word Count

## Concept

`HashMap<K, V>` stores key-value pairs. The `entry` API provides an efficient way to insert or update values.

```rust
use std::collections::HashMap;
let mut map = HashMap::new();
*map.entry("key").or_insert(0) += 1;
```

## Task

Implement `word_count(text: &str) -> HashMap<String, usize>` that counts the frequency of each word.

## Requirements

- Split text by whitespace
- Count occurrences of each word (case-insensitive)
- Use the `entry` API for efficient counting

## XP Reward

50 XP (Beginner)
