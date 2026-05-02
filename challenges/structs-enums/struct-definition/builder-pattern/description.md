# Builder Pattern

## Concept

The builder pattern lets you construct complex structs step by step:

```rust
let email = EmailBuilder::new()
    .to("alice@example.com")
    .subject("Hello")
    .body("Hi there!")
    .build();
```

Each method takes `self` and returns `Self`, enabling method chaining.

## Task

1. Define an `Email` struct with `to`, `subject`, and `body` fields
2. Define an `EmailBuilder` struct
3. Implement builder methods: `new()`, `to()`, `subject()`, `body()`, `build()`

## Requirements

- Builder methods should take `self` (not `&mut self`) and return `Self`
- `build()` should consume the builder and return an `Email`
- All fields are `String`

## XP Reward

100 XP (Medium)
