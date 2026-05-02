# String Slices

## Concept

A string slice (`&str`) is a reference to a portion of a String. You create slices using range syntax.

```rust
let s = String::from("hello world");
let hello = &s[0..5]; // "hello"
let world = &s[6..];  // "world"
```

## Task

Implement `first_word(s: &str) -> &str` that returns the first word of a string (everything before the first space).

## Requirements

- Return a string slice (not a new String)
- If there's no space, return the entire string
- Use `find(' ')` to locate the space

## XP Reward

50 XP (Beginner)
