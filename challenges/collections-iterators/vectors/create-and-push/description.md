# Create and Push

## Concept

`Vec<T>` is Rust's growable array type. Create vectors with `Vec::new()` or `vec![]` and add elements with `push()`.

```rust
let mut v = Vec::new();
v.push(1);
v.push(2);
// or
let v = vec![1, 2, 3];
```

## Task

Implement `build_sequence(n: u32) -> Vec<u32>` that creates a vector containing numbers from 1 to n.

## Requirements

- Create an empty vector
- Push numbers 1 through n
- Return the completed vector

## XP Reward

50 XP (Beginner)
