# Slice Search

## Concept

Slices support various search operations. You can iterate over sub-slices using windows or manual indexing.

```rust
let data = [1, 2, 3, 4, 5];
let found = data.windows(2).any(|w| w[0] + w[1] == 5);
```

## Task

Implement `has_subslice_sum(numbers: &[i32], target: i32) -> bool` that checks if any contiguous sub-sequence sums to the target.

## Requirements

- Check all contiguous sub-slices
- Return true if any sub-slice sums to target
- Handle empty slices (return false)

## XP Reward

75 XP (Easy)
