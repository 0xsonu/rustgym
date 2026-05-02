# Basic Loop

## Concept

The `loop` keyword creates an infinite loop. Use `break` to exit and optionally return a value from the loop.

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
```

## Task

Implement `find_first_power_of_two_above(threshold: u32) -> u32` that finds the first power of 2 greater than the threshold.

## Requirements

- Start from 1 and keep doubling
- Use `loop` with `break` to return the answer
- Return the first power of 2 that exceeds the threshold

## XP Reward

50 XP (Beginner)
