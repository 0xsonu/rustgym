# Borrow in Function

## Concept

Passing references to functions is called "borrowing." The function borrows the value temporarily without taking ownership.

```rust
fn first_word(s: &str) -> &str {
    &s[..s.find(' ').unwrap_or(s.len())]
}
```

## Task

Implement `count_vowels(text: &str) -> usize` that counts the number of vowels (a, e, i, o, u) in a string.

## Requirements

- Accept a string slice reference
- Count both uppercase and lowercase vowels
- Return the count

## XP Reward

75 XP (Easy)
