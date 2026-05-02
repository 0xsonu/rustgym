# Error Display

## Concept

Implementing `Display` for error types provides user-friendly error messages. Struct variants in enums can carry context data.

```rust
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(id) => write!(f, "item {} not found", id),
        }
    }
}
```

## Task

Create a `ValidationError` enum with struct variants that carry context, and implement `validate_email`.

## Requirements

- Define variants with named fields (field, min/max/expected)
- Implement `Display` with descriptive messages
- Validate email length and format

## XP Reward

75 XP (Easy)
