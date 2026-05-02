# Multiple Borrows

## Concept

You can have multiple immutable references to the same data at the same time. This is safe because no one can modify the data.

```rust
let s = String::from("hello");
let r1 = &s;
let r2 = &s;
println!("{} and {}", r1, r2); // OK!
```

## Task

Implement `longer(s1: &str, s2: &str) -> &str` that returns the longer of two string slices.

## Requirements

- Accept two string slice references
- Return the one with greater length
- If equal, return the first

## XP Reward

50 XP (Beginner)
