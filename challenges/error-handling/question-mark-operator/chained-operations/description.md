# Chained Operations

## Concept

The `?` operator works with `map_err` to convert between error types, enabling chaining of operations that may fail differently.

```rust
fn parse_config(s: &str) -> Result<Config, String> {
    let port = s.parse::<u16>().map_err(|e| e.to_string())?;
    Ok(Config { port })
}
```

## Task

Implement `parse_key_value(input: &str) -> Result<i32, String>` that parses a "key=value" string and returns the value as an integer.

## Requirements

- Split the input at '='
- Return an error if no '=' is found
- Parse the value portion as i32
- Use `map_err` with `?` to convert parse errors

## XP Reward

50 XP (Beginner)
