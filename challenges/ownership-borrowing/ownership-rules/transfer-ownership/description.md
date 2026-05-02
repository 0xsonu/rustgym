# Transfer Ownership

## Concept

When you pass a value to a function, ownership is transferred (moved) to that function:

```rust
fn take_string(s: String) {
    println!("{}", s);
} // s is dropped here

let my_string = String::from("hello");
take_string(my_string);
// my_string is no longer valid here!
```

To give ownership back, return the value from the function.

## Task

Implement two functions that take ownership of a String:

1. `take_and_uppercase(s)` — takes a String, returns its uppercase version
2. `take_and_append(s, suffix)` — takes a String, appends a suffix, returns it

## Requirements

- Both functions take ownership of the String parameter
- Return the modified String (transferring ownership back to the caller)

## XP Reward

75 XP (Easy)
