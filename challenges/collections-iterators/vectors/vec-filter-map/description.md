# Vec Filter Map

## Concept

Vectors work seamlessly with iterators. Use `iter()`, `filter()`, `map()`, and `collect()` to transform data.

```rust
let evens: Vec<i32> = numbers.iter()
    .filter(|&&n| n % 2 == 0)
    .copied()
    .collect();
```

## Task

Implement `extract_positives(numbers: &[i32]) -> Vec<i32>` that returns only the positive numbers, doubled.

## Requirements

- Filter to keep only positive numbers (> 0)
- Double each remaining number
- Return as a new Vec

## XP Reward

75 XP (Easy)
