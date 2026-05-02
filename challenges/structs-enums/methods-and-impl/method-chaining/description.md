# Method Chaining

## Concept

Methods that return `Self` enable fluent interfaces (method chaining):

```rust
let result = Builder::new()
    .step_one()
    .step_two()
    .build();
```

The key is that each method takes `self` (by value) and returns `Self`.

## Task

Implement a `StringBuilder` with chainable methods:

1. `new()` — creates an empty builder
2. `add(self, part)` — adds a string part
3. `newline(self)` — adds a newline character
4. `build(self)` — joins all parts into a final String

## Requirements

- Methods take `self` (by value) and return `Self`
- `build` consumes the builder and returns the final String
- Parts are joined with no separator

## XP Reward

100 XP (Medium)
