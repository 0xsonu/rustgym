# Nested Structs

## Concept

Structs can contain other structs as fields:

```rust
struct Address {
    street: String,
    city: String,
}

struct Person {
    name: String,
    address: Address,
}
```

Access nested fields with dot notation: `person.address.city`

## Task

1. Define an `Address` struct with `street`, `city`, and `zip` fields
2. Define a `Person` struct with `name`, `age`, and `address` fields
3. Implement `create_person(...)` — creates a Person with nested Address
4. Implement `describe_person(person)` — returns "Name lives in City"

## Requirements

- All fields should be `pub`
- Use `&Person` reference in `describe_person`

## XP Reward

100 XP (Medium)
