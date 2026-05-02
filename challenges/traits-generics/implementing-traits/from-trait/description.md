# From Trait

## Concept

The `From` trait enables type conversions:

```rust
impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

// Now you can use .into()
let f: Fahrenheit = Celsius(100.0).into();
```

Implementing `From<A> for B` automatically gives you `Into<B> for A`.

## Task

Implement bidirectional conversion between Celsius and Fahrenheit:

1. `From<Celsius> for Fahrenheit`
2. `From<Fahrenheit> for Celsius`

## Requirements

- Use tuple structs: `Celsius(f64)` and `Fahrenheit(f64)`
- Formulas: F = C × 9/5 + 32, C = (F - 32) × 5/9

## XP Reward

75 XP (Easy)
