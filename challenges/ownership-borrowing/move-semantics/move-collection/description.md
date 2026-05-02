# Move Collection

## Concept

When you call `into_iter()` on a collection, it consumes the collection and moves each element out:

```rust
let names = vec![String::from("Alice"), String::from("Bob")];
let upper: Vec<String> = names.into_iter()
    .map(|s| s.to_uppercase())
    .collect();
// names is no longer valid
```

## Task

Implement two functions that consume and transform collections:

1. `filter_long_strings(strings, min_length)` — keeps only strings longer than min_length
2. `uppercase_all(strings)` — converts all strings to uppercase

## Requirements

- Use `into_iter()` to consume the input Vec
- Use `filter()` and `map()` respectively
- Return a new `Vec<String>`

## XP Reward

100 XP (Medium)
