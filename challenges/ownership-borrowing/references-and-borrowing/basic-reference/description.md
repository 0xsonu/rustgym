# Basic Reference

## Concept

References let you refer to a value without taking ownership. Use `&` to create a reference and `&` in the type annotation.

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

## Task

Implement `string_length(s: &String) -> usize` that returns the length of a borrowed string without taking ownership.

## Requirements

- Accept a reference to a String
- Return its length
- The caller should still own the string after the call

## XP Reward

50 XP (Beginner)
