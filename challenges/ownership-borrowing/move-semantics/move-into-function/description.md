# Move Into Function

## Concept

Passing a value to a function moves ownership into that function. To keep using the value after the call, the function must return it:

```rust
fn process(mut v: Vec<i32>) -> Vec<i32> {
    v.push(42);
    v // return ownership
}
```

## Task

Implement two functions:

1. `push_and_return(vec, value)` — takes a Vec, pushes a value, returns the Vec
2. `sum_and_return(vec)` — takes a Vec, computes the sum, returns both

## Requirements

- `push_and_return` should add the value and return the modified Vec
- `sum_and_return` should return `(sum, vec)` as a tuple

## XP Reward

75 XP (Easy)
