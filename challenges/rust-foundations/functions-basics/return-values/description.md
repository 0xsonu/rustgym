# Return Values

## Concept

Functions in Rust return the value of their last expression (without a semicolon). You can also use `return` for early returns.

```rust
fn absolute(n: i32) -> i32 {
    if n < 0 { -n } else { n }
}
```

## Task

Implement two functions:

1. `max_of_two(a: i32, b: i32) -> i32` — returns the larger value
2. `is_even(n: i32) -> bool` — returns true if n is even

## Requirements

- Use expressions (not `return` keyword) where possible
- Handle equal values in `max_of_two`

## XP Reward

75 XP (Easy)
