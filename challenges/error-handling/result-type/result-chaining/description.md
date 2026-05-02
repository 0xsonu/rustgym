# Result Chaining

## Concept

The `and_then` method chains operations that each return a Result. If any step fails, the chain short-circuits.

```rust
let result = "5"
    .parse::<i32>()
    .and_then(|n| if n > 0 { Ok(n) } else { Err("must be positive".into()) });
```

## Task

Implement `validate_username(input: &str) -> Result<String, String>` that validates a username through multiple checks.

## Requirements

- Must be at least 3 characters long
- Must be at most 20 characters long
- Must contain only alphanumeric characters and underscores
- Return the trimmed, lowercased username on success

## XP Reward

100 XP (Medium)
