# Multi Return

## Concept

Rust functions can return multiple values using tuples. This is useful when you need to compute several related results.

```rust
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b { (a, b) } else { (b, a) }
}
```

## Task

Implement `analyze_text(text: &str) -> (usize, usize, usize)` that returns a tuple of (character count, word count, sentence count).

## Requirements

- Count characters (excluding spaces)
- Count words (split by whitespace)
- Count sentences (split by '.', '!', or '?')
- Return all three as a tuple

## XP Reward

100 XP (Medium)
