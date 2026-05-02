# Basic Trait

## Concept

Traits define shared behavior. They're like interfaces in other languages:

```rust
trait Greet {
    fn hello(&self) -> String;
}
```

Any type can implement a trait:

```rust
impl Greet for Person {
    fn hello(&self) -> String {
        format!("Hi, I'm {}", self.name)
    }
}
```

## Task

1. Define a `Greet` trait with a `hello(&self) -> String` method
2. Implement it for `Person` (returns "Hello, I'm {name}!")
3. Implement it for `Robot` (returns "Beep boop. Unit {id} online.")

## Requirements

- The trait has one required method: `hello`
- Person has a `name: String` field
- Robot has an `id: u32` field

## XP Reward

50 XP (Beginner)
