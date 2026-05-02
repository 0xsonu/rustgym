# Clone Strings

## Concept

Types that don't implement `Copy` (like `String`) can still be duplicated using `.clone()`:

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // explicit deep copy
// Both s1 and s2 are valid!
```

Cloning creates a deep copy — it allocates new memory and copies the data.

## Task

Implement two functions:

1. `clone_and_modify(original)` — clones a String and returns both the original and clone
2. `double_greeting(name)` — uses a name in two different greetings

## Requirements

- `clone_and_modify` returns `(original, clone)` as a tuple
- `double_greeting` returns `("Hello, {name}!", "Hey, {name}!")` as a tuple
- Use `.clone()` to keep the original valid

## XP Reward

75 XP (Easy)
