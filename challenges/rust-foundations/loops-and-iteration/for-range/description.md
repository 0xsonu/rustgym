# For Range

## Concept

The `for` loop iterates over ranges and collections. Ranges use `..` (exclusive) or `..=` (inclusive).

```rust
for i in 1..=5 {
    println!("{}", i); // 1, 2, 3, 4, 5
}
```

## Task

Implement `sum_range(start: i32, end: i32) -> i32` that sums all integers from start to end (inclusive).

## Requirements

- Use a `for` loop with an inclusive range (`..=`)
- Return the sum of all numbers in the range
- If start > end, return 0

## XP Reward

75 XP (Easy)
