# Two Sum

## Concept

HashMaps enable O(1) lookups, making them ideal for problems that require finding complements or pairs.

```rust
let mut seen = HashMap::new();
for (i, &num) in numbers.iter().enumerate() {
    if let Some(&j) = seen.get(&(target - num)) {
        return Some((j, i));
    }
    seen.insert(num, i);
}
```

## Task

Implement `two_sum(numbers: &[i32], target: i32) -> Option<(usize, usize)>` that finds two indices whose values sum to target.

## Requirements

- Return the first pair of indices that sum to target
- Use a HashMap for O(n) time complexity
- Return None if no pair exists

## XP Reward

75 XP (Easy)
