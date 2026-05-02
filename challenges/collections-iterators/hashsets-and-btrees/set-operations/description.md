# Set Operations

## Concept

HashSet supports mathematical set operations: `intersection`, `union`, `difference`, and `symmetric_difference`.

```rust
let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
let b: HashSet<i32> = [2, 3, 4].into_iter().collect();
let common: HashSet<&i32> = a.intersection(&b).collect();
```

## Task

Implement `intersection` and `union` functions that perform set operations on slices.

## Requirements

- Convert slices to HashSets
- Use built-in set operations
- Return sorted results as Vec

## XP Reward

50 XP (Beginner)
