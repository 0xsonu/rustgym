# Unwrap Or

## Concept

`unwrap_or` and `unwrap_or_else` provide fallback values when a Result is Err, without panicking.

```rust
let port: u16 = env_var.parse().unwrap_or(8080);
```

## Task

Implement two functions:

1. `get_config_value(input: &str, default: i32) -> i32` — parses or returns default
2. `get_config_or_else(input: &str, compute_default: fn() -> i32) -> i32` — parses or computes default

## Requirements

- Use `unwrap_or` for the first function
- Use `unwrap_or_else` for the second function
- Never panic on invalid input

## XP Reward

75 XP (Easy)
