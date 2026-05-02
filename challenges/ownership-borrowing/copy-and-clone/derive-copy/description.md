# Derive Copy

## Concept

You can make your own types `Copy` by deriving it, as long as all fields are also `Copy`:

```rust
#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}
```

Now `Point` values are copied instead of moved!

## Task

1. Define a `Point` struct with `x: f64` and `y: f64` that derives `Copy`
2. Implement `translate(point, dx, dy)` — returns both the original and a translated copy
3. Implement `distance(a, b)` — returns the Euclidean distance between two points

## Requirements

- `Point` must derive `Debug`, `Clone`, `Copy`, and `PartialEq`
- `translate` returns `(original, translated)` — both are valid because Point is Copy
- `distance` uses the formula: `sqrt((x2-x1)² + (y2-y1)²)`

## XP Reward

75 XP (Easy)
