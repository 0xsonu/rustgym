# Error Enum

## Concept

Custom error types let you represent different failure modes with an enum. Each variant describes a specific error condition.

```rust
#[derive(Debug)]
enum AppError {
    NotFound,
    InvalidInput(String),
    DatabaseError(String),
}
```

## Task

Create a `TemperatureError` enum and implement `celsius_to_fahrenheit(input: &str) -> Result<f64, TemperatureError>`.

## Requirements

- Define `TemperatureError` with variants: `BelowAbsoluteZero` and `ParseError(String)`
- Implement `Display` for the error type
- Return appropriate error variants for each failure case

## XP Reward

50 XP (Beginner)
