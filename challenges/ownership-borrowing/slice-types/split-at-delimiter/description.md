# Split at Delimiter

## Concept

String slices can be split into sub-slices at specific positions. This is a common pattern for parsing text.

```rust
let email = "user@example.com";
let at_pos = email.find('@').unwrap();
let (name, domain) = (&email[..at_pos], &email[at_pos+1..]);
```

## Task

Implement `split_once_at(text: &str, delimiter: char) -> (&str, &str)` that splits a string at the first occurrence of a delimiter.

## Requirements

- Return a tuple of two string slices
- Split at the first occurrence of the delimiter
- If delimiter is not found, return (full string, empty string)

## XP Reward

100 XP (Medium)
