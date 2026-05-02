# Error Context

## Concept

Good error types carry enough context to understand what went wrong. Use enum variants with data to provide specific failure information.

```rust
enum MathError {
    DivisionByZero,
    Overflow { operation: String },
}
```

## Task

Create a `CalcError` enum and implement `calculate(a: i64, op: char, b: i64) -> Result<i64, CalcError>`.

## Requirements

- Define variants: `DivisionByZero`, `Overflow`, `InvalidOperator(char)`
- Support +, -, \*, / operators
- Use checked arithmetic to detect overflow
- Return appropriate error variants

## XP Reward

75 XP (Easy)
