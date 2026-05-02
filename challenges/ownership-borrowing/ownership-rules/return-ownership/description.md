# Return Ownership

## Concept

Functions can give ownership to the caller by returning values:

```rust
fn give_string() -> String {
    String::from("hello") // ownership moves to caller
}

fn take_and_return(s: String) -> String {
    s // give it back
}
```

## Task

Implement two functions:

1. `give_ownership()` — creates and returns a new String `"yours now"`
2. `take_and_give_back(s)` — takes a String, returns it along with its length

## Requirements

- `give_ownership` returns a new `String`
- `take_and_give_back` returns `(String, usize)` — the original string and its length

## XP Reward

75 XP (Easy)
