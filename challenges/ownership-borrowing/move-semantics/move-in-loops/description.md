# Move in Loops

## Concept

When iterating over a collection with `into_iter()`, each element is moved out of the collection:

```rust
let names = vec![String::from("Alice"), String::from("Bob")];
for name in names.into_iter() {
    println!("{}", name); // name is moved into the loop body
}
// names is no longer valid here
```

## Task

Implement the function `build_sentence(words: Vec<String>)` that joins all words with spaces.

## Requirements

- Use `into_iter()` to consume the Vec
- Join words with a single space between them
- Return the resulting String

## XP Reward

75 XP (Easy)
