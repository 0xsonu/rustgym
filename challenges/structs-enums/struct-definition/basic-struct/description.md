# Basic Struct

## Concept

Structs let you group related data together:

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}
```

You create instances by providing values for each field:

```rust
let user = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    active: true,
};
```

## Task

1. Define a `User` struct with fields: `username`, `email`, and `active`
2. Implement `create_user(username, email)` that creates a new User (active defaults to true)

## Requirements

- All fields should be `pub`
- `username` and `email` are `String`
- `active` is `bool`, defaulting to `true`

## XP Reward

50 XP (Beginner)
