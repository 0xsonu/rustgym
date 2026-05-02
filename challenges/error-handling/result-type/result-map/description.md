# Result Map

## Concept

The `map` and `map_err` methods transform the success or error value inside a Result without unwrapping it.

```rust
let result: Result<i32, _> = "5".parse::<i32>();
let doubled = result.map(|n| n * 2); // Ok(10)
```

## Task

Implement `sum_strings(inputs: &[&str]) -> Result<i32, String>` that parses a list of number strings and returns their sum.

## Requirements

- Parse each string to an i32
- Return the sum if all parse successfully
- Return an error message for the first unparseable string

## XP Reward

75 XP (Easy)
