# Modify Vec

## Concept

With a mutable reference to a vector, you can modify its elements using `iter_mut()`.

```rust
fn increment_all(nums: &mut Vec<i32>) {
    for n in nums.iter_mut() {
        *n += 1;
    }
}
```

## Task

Implement `double_elements(numbers: &mut Vec<i32>)` that doubles every element in the vector in place.

## Requirements

- Accept a mutable reference to a Vec<i32>
- Multiply each element by 2 in place
- Use `iter_mut()` to get mutable references to elements

## XP Reward

50 XP (Beginner)
