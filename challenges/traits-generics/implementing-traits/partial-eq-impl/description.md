# PartialEq & PartialOrd

## Concept

`PartialEq` enables `==` and `!=` comparisons. `PartialOrd` enables `<`, `>`, `<=`, `>=`:

```rust
impl PartialEq for MyType {
    fn eq(&self, other: &Self) -> bool { ... }
}

impl PartialOrd for MyType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { ... }
}
```

## Task

Implement `PartialEq` and `PartialOrd` for a `Version` struct (semantic versioning):

- Two versions are equal if all components match
- Compare major first, then minor, then patch

## Requirements

- Version has `major`, `minor`, `patch` fields (all `u32`)
- Implement both `PartialEq` and `PartialOrd` manually (don't derive)

## XP Reward

100 XP (Medium)
