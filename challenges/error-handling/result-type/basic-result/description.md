# Basic Result

## Concept

`Result<T, E>` is Rust's way of handling recoverable errors. It has two variants: `Ok(T)` for success and `Err(E)` for failure.

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}
```

## Task

Implement `parse_age(input: &str) -> Result<u8, String>` that parses a string into a valid age (0-150).

## Requirements

- Return `Ok(age)` for valid numeric input in range 0-150
- Return `Err` with a descriptive message for non-numeric input
- Return `Err` for ages outside the valid range

## XP Reward

50 XP (Beginner)
