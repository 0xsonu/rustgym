# Partial Move

## Concept

When you move a field out of a struct, the entire struct becomes partially moved and can no longer be used as a whole:

```rust
struct Point { x: String, y: String }

let p = Point { x: String::from("1"), y: String::from("2") };
let x = p.x; // x field is moved out
// p.y is still valid, but p as a whole is not
```

## Task

Implement two functions that demonstrate moving fields out of a struct:

1. `extract_name(person)` — moves the name out of a Person
2. `split_person(person)` — moves both name and email out

## Requirements

- Define a `Person` struct with `name: String` and `email: String`
- `extract_name` returns just the name
- `split_person` returns `(name, email)` as a tuple

## XP Reward

100 XP (Medium)
