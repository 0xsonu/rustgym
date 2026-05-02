# Multiple Methods

## Concept

A struct can have many methods in its `impl` block, combining constructors, getters, mutators, and formatters:

```rust
impl BankAccount {
    fn new(owner: &str) -> Self { ... }
    fn deposit(&mut self, amount: f64) { ... }
    fn withdraw(&mut self, amount: f64) -> bool { ... }
    fn summary(&self) -> String { ... }
}
```

## Task

Implement a `BankAccount` struct with:

1. `new(owner)` — creates an account with zero balance
2. `deposit(&mut self, amount)` — adds money (ignore negative amounts)
3. `withdraw(&mut self, amount)` — removes money if sufficient balance, returns bool
4. `summary(&self)` — returns "owner: $balance" with 2 decimal places

## Requirements

- Ignore negative deposit amounts
- Withdraw fails (returns false) if balance is insufficient
- Use `{:.2}` format specifier for the balance

## XP Reward

100 XP (Medium)
