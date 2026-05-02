# While Loop

## Concept

A `while` loop runs as long as its condition is true. It's useful when you don't know how many iterations you need.

```rust
let mut n = 10;
while n > 0 {
    n -= 1;
}
```

## Task

Implement `count_digits(n: u32) -> u32` that counts the number of digits in a positive integer.

## Requirements

- Use a `while` loop
- Handle the special case of 0 (which has 1 digit)
- Divide by 10 repeatedly to count digits

## XP Reward

50 XP (Beginner)
