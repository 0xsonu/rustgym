# Result Matching

## Concept

You can use `match` to handle both `Ok` and `Err` variants of a Result.

```rust
match "42".parse::<i32>() {
    Ok(n) => println!("Parsed: {}", n),
    Err(e) => println!("Error: {}", e),
}
```

## Task

Implement `safe_divide(a: f64, b: f64) -> Result<f64, String>` that divides two numbers safely.

## Requirements

- Return `Ok(result)` for valid division
- Return `Err("division by zero")` when b is 0.0
- Handle the result properly

## XP Reward

50 XP (Beginner)
