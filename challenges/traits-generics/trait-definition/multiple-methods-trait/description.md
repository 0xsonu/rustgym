# Multiple Methods Trait

## Concept

Traits can require multiple methods:

```rust
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}
```

Each implementor must provide all required methods.

## Task

1. Define a `Shape` trait with `area()`, `perimeter()`, and `description()` methods
2. Implement it for `Square` (side)
3. Implement it for `Triangle` (base, height, three sides)

## Requirements

- Square area = side², perimeter = 4 × side
- Triangle area = 0.5 × base × height, perimeter = sum of sides
- Description: "Square with side {s}" or "Triangle with base {b} and height {h}"

## XP Reward

75 XP (Easy)
