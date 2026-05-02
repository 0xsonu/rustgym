# Basic Mut Ref

## Concept

Mutable references (`&mut`) allow you to modify borrowed data. You can only have one mutable reference to a value at a time.

```rust
fn add_exclamation(s: &mut String) {
    s.push('!');
}
```

## Task

Implement `append_world(s: &mut String)` that appends `" world"` to the given string.

## Requirements

- Accept a mutable reference to a String
- Modify the string in place using `push_str`
- The function returns nothing (unit type)

## XP Reward

50 XP (Beginner)
