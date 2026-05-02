# Shared Access

## Concept

Multiple immutable references allow shared read access. This is useful for comparing or combining data from multiple sources.

```rust
fn contains_any(haystack: &[i32], needles: &[i32]) -> bool {
    needles.iter().any(|n| haystack.contains(n))
}
```

## Task

Implement `common_elements(a: &[i32], b: &[i32]) -> Vec<i32>` that finds elements present in both slices.

## Requirements

- Accept two immutable slice references
- Return a Vec of elements found in both
- Preserve order from the first slice

## XP Reward

100 XP (Medium)
