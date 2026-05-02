# Scope and Drop

## Concept

When a variable goes out of scope, Rust automatically calls `drop` to free the memory:

```rust
{
    let s = String::from("hello");
    // use s here
} // s is dropped here, memory is freed
```

You can still use a value before it's dropped — just make sure to extract what you need.

## Task

Implement two functions:

1. `string_length()` — creates a String and returns its length
2. `create_and_measure(text)` — creates a String from the input, returns both the String and its length

## Requirements

- `string_length` should create `"Rust ownership"` and return its `.len()`
- `create_and_measure` should return a tuple `(String, usize)`

## XP Reward

50 XP (Beginner)
