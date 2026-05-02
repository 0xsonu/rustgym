# Associated Functions

## Concept

Associated functions don't take `self` — they're called on the type itself:

```rust
impl Color {
    fn red() -> Self {
        Color { r: 255, g: 0, b: 0 }
    }
}

let red = Color::red(); // called with ::
```

These are often used as constructors or factory methods.

## Task

Implement a `Color` struct with:

1. `new(r, g, b)` — creates a color from RGB values
2. `red()`, `green()`, `blue()` — factory methods for common colors
3. `to_hex(&self)` — returns the hex string like "#FF0000"

## Requirements

- Factory methods are associated functions (no `self`)
- `to_hex` uses `format!` with `{:02X}` for hex formatting

## XP Reward

75 XP (Easy)
