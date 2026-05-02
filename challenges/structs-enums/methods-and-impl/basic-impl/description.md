# Basic Impl

## Concept

You add methods to a struct using an `impl` block:

```rust
impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}
```

- `&self` borrows the struct immutably
- `Self` refers to the type being implemented

## Task

1. Define a `Circle` struct with `radius: f64`
2. Implement `new(radius)` — an associated function (constructor)
3. Implement `area(&self)` — returns π × r²
4. Implement `circumference(&self)` — returns 2 × π × r

## Requirements

- Use `std::f64::consts::PI` for π
- `new` is an associated function (no `self` parameter)
- `area` and `circumference` take `&self`

## XP Reward

50 XP (Beginner)
