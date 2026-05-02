use nested_structs::{create_person, describe_person};

#[test]
fn test_create_person() {
    let p = create_person("Alice", 30, "123 Main St", "Portland", "97201");
    assert_eq!(p.name, "Alice");
    assert_eq!(p.age, 30);
    assert_eq!(p.address.street, "123 Main St");
    assert_eq!(p.address.city, "Portland");
    assert_eq!(p.address.zip, "97201");
}

#[test]
fn test_describe_person() {
    let p = create_person("Bob", 25, "456 Oak Ave", "Seattle", "98101");
    assert_eq!(describe_person(&p), "Bob lives in Seattle");
}

#[test]
fn test_describe_different_city() {
    let p = create_person("Charlie", 40, "789 Pine Rd", "Denver", "80201");
    assert_eq!(describe_person(&p), "Charlie lives in Denver");
}
