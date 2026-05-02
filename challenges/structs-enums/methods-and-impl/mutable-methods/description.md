# Mutable Methods

## Concept

Methods that modify the struct take `&mut self`:

```rust
impl Counter {
    fn increment(&mut self) {
        self.value += 1;
    }
}
```

- `&self` — read-only access
- `&mut self` — mutable access (can modify fields)
- `self` — takes ownership (consumes the struct)

## Task

Implement a `Counter` struct with methods:

1. `new()` — creates a counter at 0
2. `increment(&mut self)` — adds 1
3. `decrement(&mut self)` — subtracts 1
4. `reset(&mut self)` — sets to 0
5. `get(&self)` — returns current value

## Requirements

- Mutating methods take `&mut self`
- `get` takes `&self` (read-only)

## XP Reward

75 XP (Easy)
