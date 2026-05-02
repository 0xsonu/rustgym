# Struct Update Syntax

## Concept

Rust has a shorthand for creating a new struct instance from an existing one, changing only some fields:

```rust
let user2 = User {
    email: String::from("new@example.com"),
    ..user1 // take remaining fields from user1
};
```

## Task

1. Define a `Config` struct with `host: String`, `port: u16`, and `debug: bool`
2. Implement `default_config()` — returns a config with default values
3. Implement `with_port(config, port)` — returns a new config with a different port

## Requirements

- Use struct update syntax (`..config`) in `with_port`
- Default: host="localhost", port=8080, debug=false

## XP Reward

75 XP (Easy)
