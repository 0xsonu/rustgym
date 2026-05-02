# Boolean Puzzle

## Concept

Boolean logic can be combined in interesting ways. XOR (exclusive or) is true when exactly one operand is true:

```rust
let xor = (a || b) && !(a && b);
```

## Task

Implement three boolean logic functions:

1. `exclusive_or(a, b)` — true if exactly one is true
2. `all_same(a, b, c)` — true if all three are the same value
3. `majority(a, b, c)` — true if at least two of three are true

## Requirements

- Use only `&&`, `||`, and `!` operators
- Do NOT use the `^` XOR operator for `exclusive_or`

## XP Reward

100 XP (Medium)
