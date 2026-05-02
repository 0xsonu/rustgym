# Slice Patterns

## Concept

Slices can be split and combined in various ways. Methods like `split_at()`, `chunks()`, and `windows()` provide different views.

```rust
let data = [1, 2, 3, 4, 5, 6];
let (left, right) = data.split_at(3);
// left = [1, 2, 3], right = [4, 5, 6]
```

## Task

Implement `middle_elements(slice: &[i32]) -> &[i32]` that returns the middle portion of a slice (excluding first and last elements).

## Requirements

- Return a sub-slice excluding the first and last elements
- If the slice has fewer than 3 elements, return an empty slice
- Use range indexing to create the sub-slice

## XP Reward

75 XP (Easy)
