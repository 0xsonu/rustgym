# Multi-Step Parsing

## Concept

The `?` operator shines when you have multiple parsing steps that can each fail. Each step propagates its error automatically.

```rust
fn parse_pair(s: &str) -> Result<(i32, i32), String> {
    let parts: Vec<&str> = s.split(',').collect();
    let a = parts[0].parse().map_err(|e| format!("{}", e))?;
    let b = parts[1].parse().map_err(|e| format!("{}", e))?;
    Ok((a, b))
}
```

## Task

Implement `parse_point(input: &str) -> Result<Point, String>` that parses a "x,y" string into a Point struct.

## Requirements

- Split the input at ','
- Parse both parts as f64
- Use `?` with `map_err` for each parse step
- Return descriptive errors

## XP Reward

75 XP (Easy)
