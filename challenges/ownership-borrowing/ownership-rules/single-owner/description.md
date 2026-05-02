# Single Owner

## Concept

In Rust, every value has exactly one owner. When the owner goes out of scope, the value is dropped:

```rust
{
    let s = String::from("hello"); // s owns the String
} // s goes out of scope, String is dropped
```

## Task

Implement the function `create_owned_string()` that creates and returns an owned `String`. The caller becomes the new owner.

## Requirements

- Create a `String` with the value `"I am owned!"`
- Return it (transferring ownership to the caller)

## XP Reward

50 XP (Beginner)
