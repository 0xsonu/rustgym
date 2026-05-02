use basic_trait::{Greet, Person, Robot};

#[test]
fn test_person_greet() {
    let p = Person { name: String::from("Alice") };
    assert_eq!(p.hello(), "Hello, I'm Alice!");
}

#[test]
fn test_robot_greet() {
    let r = Robot { id: 42 };
    assert_eq!(r.hello(), "Beep boop. Unit 42 online.");
}

#[test]
fn test_different_person() {
    let p = Person { name: String::from("Bob") };
    assert_eq!(p.hello(), "Hello, I'm Bob!");
}

#[test]
fn test_different_robot() {
    let r = Robot { id: 1 };
    assert_eq!(r.hello(), "Beep boop. Unit 1 online.");
}
