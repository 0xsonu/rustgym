# Default Methods

## Concept

Traits can provide default implementations that types can override:

```rust
trait Summary {
    fn title(&self) -> &str;

    fn summarize(&self) -> String {
        format!("Read more about {}...", self.title())
    }
}
```

Types can use the default or provide their own implementation.

## Task

1. Define an `Introduce` trait with:
   - Required method: `name(&self) -> &str`
   - Default method: `introduce(&self) -> String` that returns "Hi, my name is {name}."
2. Implement for `Student` (uses default `introduce`)
3. Implement for `Teacher` (overrides `introduce` to include subject)

## Requirements

- `Student` uses the default `introduce` method
- `Teacher` overrides it to return "I'm {name}, and I teach {subject}."

## XP Reward

75 XP (Easy)
