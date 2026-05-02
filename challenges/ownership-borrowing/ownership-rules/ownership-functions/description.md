# Ownership in Functions

## Concept

When you pass heap-allocated values (like `String` or `Vec`) to functions, ownership moves:

```rust
fn process(data: Vec<i32>) -> Vec<i32> {
    // data is owned here, we can transform and return it
    data.into_iter().map(|x| x + 1).collect()
}
```

## Task

Implement two functions that demonstrate ownership transfer:

1. `double_values(values: Vec<i32>)` — takes a Vec, doubles each element, returns the new Vec
2. `join_strings(a: String, b: String)` — takes two Strings, returns them joined with a space

## Requirements

- `double_values` takes ownership of the Vec and returns a new one
- `join_strings` takes ownership of both Strings and returns a new combined String

## XP Reward

100 XP (Medium)
