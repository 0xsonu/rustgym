# Basic Move

## Concept

When you assign a heap-allocated value to another variable, ownership **moves**:

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is moved to s2
// s1 is no longer valid!
```

This prevents double-free errors. Only one variable owns the data at a time.

## Task

Implement the function `move_and_return(input: String)` that takes ownership of a String, creates a new String by adding a prefix, and returns the new String.

## Requirements

- Take ownership of the input String
- Return a new String with `"Moved: "` prepended to the input

## XP Reward

50 XP (Beginner)
