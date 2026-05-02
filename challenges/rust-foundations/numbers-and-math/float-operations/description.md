# Float Operations

## Concept

Rust has two floating-point types: `f32` and `f64` (default). They support standard math operations and useful methods:

```rust
let x: f64 = 4.0;
let sqrt = x.sqrt();    // 2.0
let abs = (-3.5_f64).abs(); // 3.5
let rounded = 3.7_f64.round(); // 4.0
```

## Task

Implement the function `float_math(x: f64)` that returns a tuple with:

1. The absolute value of `x`
2. The square root of the absolute value of `x`
3. `x` rounded to the nearest integer (as f64)

## Requirements

- Use `abs()`, `sqrt()`, and `round()` methods
- Return `(abs, sqrt_of_abs, rounded)` as `(f64, f64, f64)`

## XP Reward

75 XP (Easy)
