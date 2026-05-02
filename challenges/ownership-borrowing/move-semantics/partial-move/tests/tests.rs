use partial_move::{extract_name, split_person, Person};

#[test]
fn test_extract_name() {
    let p = Person {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
    };
    assert_eq!(extract_name(p), "Alice");
}

#[test]
fn test_split_person() {
    let p = Person {
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
    };
    let (name, email) = split_person(p);
    assert_eq!(name, "Bob");
    assert_eq!(email, "bob@example.com");
}

#[test]
fn test_split_empty_email() {
    let p = Person {
        name: String::from("Charlie"),
        email: String::from(""),
    };
    let (name, email) = split_person(p);
    assert_eq!(name, "Charlie");
    assert_eq!(email, "");
}
