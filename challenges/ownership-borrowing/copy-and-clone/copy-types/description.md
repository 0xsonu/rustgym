# Copy Types

## Concept

Some types in Rust implement the `Copy` trait, meaning they are copied instead of moved:

```rust
let x = 5;
let y = x; // x is copied, both x and y are valid!
```

Types that implement `Copy`: all integers, floats, `bool`, `char`, tuples of Copy types.

Types that do NOT implement `Copy`: `String`, `Vec<T>`, any heap-allocated type.

## Task

Implement the function `demonstrate_copy(x: i32)` that shows Copy behavior by using the same value multiple times.

## Requirements

- Take an `i32` parameter
- Return a tuple `(i32, i32, i32)` containing: the original, the original doubled, and the original tripled
- The key insight: you can use `x` multiple times because `i32` is `Copy`

## XP Reward

50 XP (Beginner)
