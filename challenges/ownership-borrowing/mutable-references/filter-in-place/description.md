# Filter In Place

## Concept

With a mutable reference to a Vec, you can use methods like `retain()` to filter elements in place without creating a new vector.

```rust
fn remove_zeros(nums: &mut Vec<i32>) {
    nums.retain(|&n| n != 0);
}
```

## Task

Implement `remove_negatives(numbers: &mut Vec<i32>)` that removes all negative numbers from the vector in place.

## Requirements

- Use `retain()` on the mutable vector reference
- Keep only numbers >= 0
- Modify the vector in place (don't return a new one)

## XP Reward

75 XP (Easy)
