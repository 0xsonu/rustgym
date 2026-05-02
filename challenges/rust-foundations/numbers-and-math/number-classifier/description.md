# Number Classifier

## Concept

Combining arithmetic with comparisons lets you classify numbers:

```rust
let n = 7;
let is_positive = n > 0;
let is_even = n % 2 == 0;
```

## Task

Implement two functions:

1. `classify(n: i32)` — returns `"positive"`, `"negative"`, or `"zero"`
2. `is_even(n: i32)` — returns `true` if the number is even

## Requirements

- `classify` should return a `String` with the classification
- `is_even` should use the remainder operator `%`

## XP Reward

100 XP (Medium)
