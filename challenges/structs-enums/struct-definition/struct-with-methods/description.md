# Struct with Fields

## Concept

Structs can have multiple fields of different types:

```rust
struct Rectangle {
    width: f64,
    height: f64,
}
```

## Task

1. Define a `Rectangle` struct with `width: f64` and `height: f64`
2. Implement `new_rectangle(width, height)` — creates a Rectangle
3. Implement `area(rect)` — returns the area
4. Implement `is_square(rect)` — returns true if width equals height

## Requirements

- All fields should be `pub`
- Functions take `&Rectangle` references (not ownership)

## XP Reward

75 XP (Easy)
