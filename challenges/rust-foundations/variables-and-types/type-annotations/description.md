# Type Annotations

## Concept

Rust can usually infer types, but you can also explicitly annotate them:

```rust
let x: i32 = 42;
let pi: f64 = 3.14;
let active: bool = true;
let letter: char = 'R';
```

Common primitive types: `i32`, `f64`, `bool`, `char`, `&str`, `String`.

## Task

Implement the function `describe_types()` that returns a tuple containing one value of each type: an `i32`, an `f64`, a `bool`, and a `char`.

## Requirements

- Return a tuple `(i32, f64, bool, char)`
- The i32 should be `42`
- The f64 should be `3.14`
- The bool should be `true`
- The char should be `'R'`

## XP Reward

50 XP (Beginner)
