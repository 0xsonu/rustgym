# Mutable Variables

## Concept

By default, Rust variables are immutable. To make a variable mutable, add the `mut` keyword:

```rust
let mut count = 0;
count = count + 1; // This is allowed because count is mutable
```

## Task

Implement the function `count_up()` that starts a counter at 0, increments it 3 times, and returns the final value.

## Requirements

- Declare a mutable variable starting at `0`
- Increment it exactly 3 times (by 1 each time)
- Return the final value (should be `3`)

## XP Reward

50 XP (Beginner)
