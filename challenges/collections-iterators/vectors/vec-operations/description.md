# Vec Operations

## Concept

Vectors support many operations: `pop()`, `remove()`, `insert()`, `contains()`, `len()`, and indexing.

```rust
let mut v = vec![1, 2, 3];
v.pop();        // removes last: Some(3)
v.insert(0, 0); // insert at index
```

## Task

Implement `rotate_left(v: &mut Vec<i32>, n: usize)` that rotates a vector left by n positions.

## Requirements

- Move the first n elements to the end
- Handle n >= length (use modulo)
- Modify the vector in place

## XP Reward

50 XP (Beginner)
