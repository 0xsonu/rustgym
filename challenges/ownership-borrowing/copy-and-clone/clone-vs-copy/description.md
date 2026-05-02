# Clone vs Copy

## Concept

- `Copy`: Implicit, cheap bit-for-bit copy (stack only). Used for simple types.
- `Clone`: Explicit, potentially expensive deep copy. Used for heap-allocated types.

```rust
// Copy (implicit)
let x = 5;
let y = x; // copied automatically

// Clone (explicit)
let v1 = vec![1, 2, 3];
let v2 = v1.clone(); // must call .clone() explicitly
```

## Task

Implement two functions that use `.clone()` on Vec:

1. `duplicate_vec(v)` — clones a Vec and returns both
2. `clone_and_push(v, value)` — clones a Vec, pushes to the clone, returns both

## Requirements

- Use `.clone()` to create independent copies
- The original Vec should remain unchanged
- Return tuples of `(original, clone)`

## XP Reward

100 XP (Medium)
