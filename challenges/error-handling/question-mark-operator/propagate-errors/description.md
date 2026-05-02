# Propagate Errors

## Concept

The `?` operator in loops lets you fail fast — the first error encountered stops processing and returns immediately.

```rust
fn process_all(items: &[&str]) -> Result<Vec<i32>, String> {
    let mut results = Vec::new();
    for item in items {
        let n = item.parse::<i32>().map_err(|e| e.to_string())?;
        results.push(n);
    }
    Ok(results)
}
```

## Task

Implement `sum_positive_numbers(inputs: &[&str]) -> Result<i32, String>` that parses strings, filters positive numbers, and sums them.

## Requirements

- Parse each string as i32 using `?`
- Only sum positive numbers (> 0)
- Return error immediately if any string fails to parse

## XP Reward

100 XP (Medium)
