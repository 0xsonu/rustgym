# Multiple Traits

## Concept

A single type can implement many traits:

```rust
impl Display for MyType { ... }
impl PartialEq for MyType { ... }
impl PartialOrd for MyType { ... }
```

## Task

Implement three standard library traits for a `Duration` struct:

1. `Display` — format as "HH:MM:SS"
2. `PartialEq` — compare by total seconds
3. `PartialOrd` — order by total seconds

## Requirements

- Duration stores total seconds as `u64`
- Display format: `{:02}:{:02}:{:02}` (zero-padded)
- Provide `new(seconds)`, `from_minutes(m)`, `from_hours(h)` constructors

## XP Reward

100 XP (Medium)
