# Array Slices

## Concept

Array slices (`&[T]`) reference a contiguous sequence of elements. They work with both arrays and vectors.

```rust
let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..3]; // [2, 3]
```

## Task

Implement `sum_slice(numbers: &[i32]) -> i32` that sums all elements in a slice.

## Requirements

- Accept a slice reference (works with arrays and vectors)
- Return the sum of all elements
- Handle empty slices (return 0)

## XP Reward

50 XP (Beginner)
