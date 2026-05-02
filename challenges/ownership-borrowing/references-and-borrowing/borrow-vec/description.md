# Borrow Vec

## Concept

You can borrow vectors to read their contents without taking ownership. This lets the caller keep using the vector after the function returns.

```rust
fn sum(numbers: &Vec<i32>) -> i32 {
    numbers.iter().sum()
}
```

## Task

Implement `sum_and_average(numbers: &Vec<f64>) -> (f64, f64)` that returns both the sum and average of a borrowed vector.

## Requirements

- Accept a reference to a Vec<f64>
- Return a tuple of (sum, average)
- Handle empty vectors by returning (0.0, 0.0)

## XP Reward

75 XP (Easy)
