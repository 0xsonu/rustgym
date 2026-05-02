# Nested Loops

## Concept

You can nest loops inside each other to work with multi-dimensional data or generate combinations.

```rust
for row in 0..3 {
    for col in 0..3 {
        println!("({}, {})", row, col);
    }
}
```

## Task

Implement `multiplication_table(size: u32) -> Vec<String>` that generates a multiplication table. Each entry should be formatted as `"a x b = c"`.

## Requirements

- Use nested for loops
- Generate all combinations from 1 to size
- Return results as a Vec of formatted strings

## XP Reward

100 XP (Medium)
