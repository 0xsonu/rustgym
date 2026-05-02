# Custom Clone

## Concept

You can derive `Clone` for structs that contain non-Copy fields:

```rust
#[derive(Clone)]
struct Config {
    name: String,
    value: i32,
}
```

Or implement it manually for custom behavior.

## Task

1. Define a `Document` struct with `title: String` and `content: String`
2. Derive `Clone` for it
3. Implement `create_draft(doc)` — clones a document and adds " (Draft)" to the title

## Requirements

- `Document` should derive `Clone` and `Debug`
- `create_draft` returns both the original and the draft copy
- The draft's title should have " (Draft)" appended

## XP Reward

100 XP (Medium)
