# Comparison Chain

## Concept

You can chain comparisons to check ranges:

```rust
let in_range = x >= 0 && x <= 100;
```

Comparison operators: `==`, `!=`, `<`, `>`, `<=`, `>=`

## Task

Implement the function `grade(score: u32)` that returns a letter grade based on the score:

- 90-100: `"A"`
- 80-89: `"B"`
- 70-79: `"C"`
- 60-69: `"D"`
- Below 60: `"F"`

## Requirements

- Use comparison operators to determine the grade
- Return a `&str` (string slice)
- Assume score is between 0 and 100

## XP Reward

75 XP (Easy)
