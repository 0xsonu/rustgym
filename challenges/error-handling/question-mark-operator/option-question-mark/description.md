# Option Question Mark

## Concept

The `?` operator also works with `Option`. If the value is `None`, the function returns `None` early.

```rust
fn get_middle(s: &str) -> Option<char> {
    let len = s.len();
    if len == 0 { return None; }
    s.chars().nth(len / 2)
}
```

## Task

Implement `extract_domain(email: &str) -> Option<&str>` that extracts the domain from an email address.

## Requirements

- Use `?` with Option-returning methods
- Return `None` if there's no '@' symbol
- Return `None` if the domain part is empty
- Return `Some(domain)` otherwise

## XP Reward

75 XP (Easy)
