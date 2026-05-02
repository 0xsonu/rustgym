# Transform String

## Concept

Mutable references to Strings let you transform their content in place using methods like `clear()`, `push_str()`, and character manipulation.

```rust
fn make_uppercase(s: &mut String) {
    *s = s.to_uppercase();
}
```

## Task

Implement `capitalize_first(s: &mut String)` that capitalizes the first letter and lowercases the rest.

## Requirements

- Modify the string in place via a mutable reference
- Capitalize only the first character
- Lowercase all remaining characters
- Handle empty strings gracefully

## XP Reward

100 XP (Medium)
