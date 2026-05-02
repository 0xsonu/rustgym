# Iterator Implementation

## Concept

The `Iterator` trait requires implementing one method: `next()`:

```rust
impl Iterator for MyType {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // Return Some(value) or None when done
    }
}
```

Once implemented, you get `map`, `filter`, `sum`, `collect`, etc. for free!

## Task

Implement a `Counter` struct that counts from 1 to a maximum value:

1. `Counter::new(max)` — creates a counter
2. Implement `Iterator` — returns 1, 2, 3, ..., max, then None

## Requirements

- `type Item = u32`
- Returns `Some(n)` for each value from 1 to max
- Returns `None` after reaching max

## XP Reward

100 XP (Medium)
