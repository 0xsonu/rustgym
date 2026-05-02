# Helper Functions

## Concept

You can break complex logic into smaller helper functions. Functions can call other functions to build up behavior.

```rust
fn square(n: i32) -> i32 {
    n * n
}

fn sum_of_squares(a: i32, b: i32) -> i32 {
    square(a) + square(b)
}
```

## Task

Implement a `distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64` function that calculates the Euclidean distance between two points. Use a helper function `square(n: f64) -> f64`.

## Requirements

- Create a `square` helper function
- Use it inside `distance` to compute the Euclidean distance
- Formula: sqrt((x2-x1)² + (y2-y1)²)

## XP Reward

75 XP (Easy)
