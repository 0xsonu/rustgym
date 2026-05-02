# Nested Conditions

## Concept

You can chain multiple `else if` branches to handle several conditions. The first matching branch executes.

```rust
if temp > 30 { "hot" } else if temp > 20 { "warm" } else { "cold" }
```

## Task

Implement `letter_grade(score: u32) -> &'static str` that converts a numeric score to a letter grade.

## Requirements

- 90+ → "A"
- 80-89 → "B"
- 70-79 → "C"
- 60-69 → "D"
- Below 60 → "F"

## XP Reward

75 XP (Easy)
