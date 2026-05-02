# Vec Dedup

## Concept

`dedup()` removes consecutive duplicates from a sorted vector. Combined with `sort()`, it removes all duplicates.

```rust
let mut v = vec![1, 3, 2, 1, 3];
v.sort();
v.dedup();
// v = [1, 2, 3]
```

## Task

Implement `unique_sorted(numbers: Vec<i32>) -> Vec<i32>` that returns a sorted vector with duplicates removed.

## Requirements

- Sort the vector
- Remove all duplicates
- Return the result

## XP Reward

100 XP (Medium)
