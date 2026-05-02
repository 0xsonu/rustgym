# Range Checker

## Concept

Combining comparisons with logical operators lets you check complex conditions:

```rust
let in_range = x >= 0 && x <= 100;
let special = (x % 4 == 0 && x % 100 != 0) || (x % 400 == 0);
```

## Task

Implement two functions:

1. `in_range(value, min, max)` — returns `true` if value is between min and max (inclusive)
2. `is_leap_year(year)` — returns `true` if the year is a leap year

## Requirements

- `in_range` should check `value >= min && value <= max`
- A leap year is divisible by 4, but not by 100, unless also divisible by 400

## XP Reward

100 XP (Medium)
