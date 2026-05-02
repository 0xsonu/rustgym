# Display Trait

## Concept

The `std::fmt::Display` trait lets you define how a type is printed with `{}`:

```rust
use std::fmt;

impl fmt::Display for MyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyType: {}", self.value)
    }
}
```

## Task

Implement `Display` for two types:

1. `Color` — displays as "rgb(r, g, b)"
2. `Point` — displays as "(x, y)"

## Requirements

- Use `write!` macro inside the `fmt` method
- Color format: `rgb(255, 128, 0)`
- Point format: `(3.5, -2.1)`

## XP Reward

50 XP (Beginner)
