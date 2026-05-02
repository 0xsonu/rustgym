# From Impl

## Concept

Implementing `From` for your error type enables automatic conversion with `?`. This eliminates the need for `map_err` in many cases.

```rust
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}
```

## Task

Create a `ConfigError` enum with `From<ParseIntError>` implementation, then implement `get_port` that reads a port from a config.

## Requirements

- Define `ConfigError` with `MissingField(String)` and `InvalidNumber(ParseIntError)` variants
- Implement `From<ParseIntError>` for automatic conversion
- Use `?` without `map_err` for parse errors

## XP Reward

50 XP (Beginner)
