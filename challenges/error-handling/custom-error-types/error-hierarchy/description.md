# Error Hierarchy

## Concept

Real applications often have a top-level error type that wraps errors from different subsystems. This creates an error hierarchy.

```rust
enum AppError {
    Io(std::io::Error),
    Parse(ParseIntError),
    Custom(String),
}
```

## Task

Create an `AppError` enum that wraps multiple error sources and implement `process_user_record`.

## Requirements

- Define `AppError` with `Parse`, `Validation`, and `NotFound` variants
- Implement `From<ParseIntError>` for automatic conversion
- Implement `Display` for user-friendly messages
- Process a user record with multiple validation steps

## XP Reward

100 XP (Medium)
