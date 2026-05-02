# Compound Logic

## Concept

You can combine conditions with `&&` (and) and `||` (or) operators inside if expressions.

```rust
if age >= 18 && has_id {
    "allowed"
} else {
    "denied"
}
```

## Task

Implement `is_leap_year(year: u32) -> bool` that determines if a year is a leap year.

## Requirements

- Divisible by 400 → leap year
- Divisible by 100 but not 400 → not a leap year
- Divisible by 4 but not 100 → leap year
- Otherwise → not a leap year

## XP Reward

75 XP (Easy)
