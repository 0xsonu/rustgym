# Trait as Parameter

## Concept

You can use traits as function parameters with `impl Trait` syntax:

```rust
fn notify(item: &impl Summary) {
    println!("News: {}", item.summarize());
}
```

This accepts any type that implements the `Summary` trait.

## Task

1. Define a `Summary` trait with `summarize(&self) -> String`
2. Implement it for `Article` and `Tweet`
3. Write a `notify(item: &impl Summary)` function that wraps the summary

## Requirements

- Article summarize: "{title} by {author}"
- Tweet summarize: "@{username}: {content}"
- notify returns: "Breaking news! {summarize()}"

## XP Reward

100 XP (Medium)
