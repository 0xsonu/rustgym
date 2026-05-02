# Integer Types

## Concept

Rust has signed and unsigned integers of various sizes:

- Signed: `i8`, `i16`, `i32`, `i64`, `i128`
- Unsigned: `u8`, `u16`, `u32`, `u64`, `u128`

Each type has a minimum and maximum value:

```rust
let max_u8: u8 = 255;
let min_i8: i8 = -128;
```

## Task

Implement the function `integer_limits()` that returns the maximum values for `u8`, `i8`, and `u16` as a tuple.

## Requirements

- Return `(u8::MAX, i8::MAX, u16::MAX)` converted to a common type `(u32, u32, u32)`
- Use the `MAX` associated constants

## XP Reward

50 XP (Beginner)
