# Swap Values

## Concept

Mutable references let you modify values through the reference. You can dereference with `*` to access or change the underlying value.

```rust
fn increment(n: &mut i32) {
    *n += 1;
}
```

## Task

Implement `swap(a: &mut i32, b: &mut i32)` that swaps the values of two mutable references.

## Requirements

- Use a temporary variable to hold one value
- Dereference with `*` to read and write values
- After the call, a should have b's original value and vice versa

## XP Reward

75 XP (Easy)
