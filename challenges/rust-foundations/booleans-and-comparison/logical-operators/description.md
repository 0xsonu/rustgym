# Logical Operators

## Concept

Rust provides logical operators to combine boolean values:

- `&&` — logical AND (both must be true)
- `||` — logical OR (at least one must be true)
- `!` — logical NOT (inverts the value)

```rust
let a = true && false; // false
let b = true || false; // true
let c = !true;         // false
```

## Task

Implement the function `can_enter(age: u32, has_ticket: bool)` that returns `true` only if the person is 18 or older AND has a ticket.

## Requirements

- Use the `&&` operator to combine both conditions
- Must be 18+ AND have a ticket to enter

## XP Reward

50 XP (Beginner)
