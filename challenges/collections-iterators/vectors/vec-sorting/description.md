# Vec Sorting

## Concept

Vectors can be sorted with `sort()`, `sort_by()`, and `sort_by_key()`. Custom comparators enable complex sorting logic.

```rust
let mut words = vec!["banana", "apple", "cherry"];
words.sort_by_key(|w| w.len());
```

## Task

Implement `sort_by_length(words: &mut Vec<String>)` that sorts strings by length, then alphabetically for equal lengths.

## Requirements

- Primary sort: by string length (shortest first)
- Secondary sort: alphabetically for same-length strings
- Sort in place

## XP Reward

75 XP (Easy)
