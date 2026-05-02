# Shadowing

## Concept

In Rust, you can declare a new variable with the same name as a previous one. The new variable **shadows** the old one:

```rust
let x = 5;
let x = x + 1; // x is now 6
let x = x * 2; // x is now 12
```

Shadowing is different from `mut` because you're creating a new variable each time. You can even change the type!

## Task

Implement the function `shadow_transform()` that demonstrates shadowing by:

1. Starting with the number `5`
2. Shadowing it by doubling the value
3. Shadowing it again by converting to a string

Return the final string.

## Requirements

- Use shadowing (not mutation) to transform the value
- The final result should be the string `"10"`

## XP Reward

75 XP (Easy)
