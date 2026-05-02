# Loop with Break

## Concept

You can use `break` and `continue` to control loop flow. `break` exits the loop, and `continue` skips to the next iteration.

```rust
for item in &list {
    if *item == target {
        break;
    }
}
```

## Task

Implement `find_first_above(numbers: &[i32], threshold: i32) -> Option<i32>` that finds the first number in a slice that exceeds the threshold.

## Requirements

- Iterate over the slice with a for loop
- Return `Some(number)` for the first match
- Return `None` if no number exceeds the threshold

## XP Reward

75 XP (Easy)
