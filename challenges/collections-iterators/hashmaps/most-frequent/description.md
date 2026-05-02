# Most Frequent

## Concept

Finding the most frequent element combines counting with finding the maximum. Use `max_by_key` on the HashMap entries.

```rust
counts.iter().max_by_key(|(_, &count)| count)
```

## Task

Implement `most_frequent(items: &[&str]) -> Option<String>` that finds the most frequently occurring element.

## Requirements

- Count occurrences of each item
- Return the item with the highest count
- Return None for empty input
- If there's a tie, return any of the most frequent

## XP Reward

100 XP (Medium)
