# Basic Question Mark

## Concept

The `?` operator propagates errors automatically. If the expression is `Err`, the function returns early with that error. If it's `Ok`, the value is unwrapped.

```rust
fn read_number(s: &str) -> Result<i32, ParseIntError> {
    let n = s.parse::<i32>()?;
    Ok(n * 2)
}
```

## Task

Implement `add_strings(a: &str, b: &str) -> Result<i32, ParseIntError>` that parses two strings and returns their sum.

## Requirements

- Use `?` to propagate parse errors
- Parse both strings as i32
- Return their sum

## XP Reward

50 XP (Beginner)
