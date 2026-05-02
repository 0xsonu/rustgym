/// A trait for things that can greet.
/// TODO: Define the trait with a hello(&self) -> String method.
pub trait Greet {
    // TODO: Add the method signature
}

/// A person who can greet.
pub struct Person {
    pub name: String,
}

/// TODO: Implement Greet for Person. Return "Hello, I'm {name}!"
impl Greet for Person {
    fn hello(&self) -> String {
        todo!()
    }
}

/// A robot who can greet.
pub struct Robot {
    pub id: u32,
}

/// TODO: Implement Greet for Robot. Return "Beep boop. Unit {id} online."
impl Greet for Robot {
    fn hello(&self) -> String {
        todo!()
    }
}
