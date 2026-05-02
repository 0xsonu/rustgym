# Function Parameters

## Concept

Functions can take multiple parameters of different types. Each parameter must have a type annotation.

```rust
fn describe(name: &str, age: u32) -> String {
    format!("{} is {} years old", name, age)
}
```

## Task

Implement the function `format_price(item: &str, price: f64) -> String` that returns a formatted price string like `"Apple: $1.50"`.

## Requirements

- Accept a string slice and a float parameter
- Format the price to exactly 2 decimal places
- Return in the format `"{item}: ${price}"`

## XP Reward

50 XP (Beginner)
