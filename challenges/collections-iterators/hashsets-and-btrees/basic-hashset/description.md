# Basic HashSet

## Concept

`HashSet<T>` stores unique values. It's useful for deduplication and membership testing.

```rust
use std::collections::HashSet;
let mut set = HashSet::new();
set.insert("apple");
set.contains("apple"); // true
```

## Task

Implement `unique_words(text: &str) -> Vec<String>` that returns all unique words from a text, sorted alphabetically.

## Requirements

- Use a HashSet to collect unique words
- Convert to lowercase for comparison
- Return sorted results as a Vec

## XP Reward

50 XP (Beginner)
