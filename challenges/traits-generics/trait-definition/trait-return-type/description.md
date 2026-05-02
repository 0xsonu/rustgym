# Trait Return Type

## Concept

Functions can return `impl Trait` to hide the concrete type:

```rust
fn make_thing() -> impl Display {
    42 // returns an i32, but caller only knows it implements Display
}
```

## Task

1. Define a `Displayable` trait with `display(&self) -> String`
2. Implement it for `Celsius` and `Fahrenheit` tuple structs
3. Write factory functions that return `impl Displayable`

## Requirements

- Celsius display: "{value:.1}°C"
- Fahrenheit display: "{value:.1}°F"
- Factory functions return `impl Displayable`

## XP Reward

100 XP (Medium)
