# Range Queries

## Concept

BTreeMap supports efficient range queries. The `range()` method returns entries within a key range.

```rust
let in_range: Vec<_> = map.range(10..=20).collect();
```

## Task

Implement `range_query(map: &BTreeMap<i32, String>, from: i32, to: i32) -> Vec<(i32, String)>` that returns entries within a key range.

## Requirements

- Use BTreeMap's `range()` method
- Include both endpoints (inclusive range)
- Return entries as a Vec of tuples

## XP Reward

75 XP (Easy)
