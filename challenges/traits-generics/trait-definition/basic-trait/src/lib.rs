/// A trait for things that can greet.
pub trait Greet {
    fn hello(&self) -> String;
}

/// A person who can greet.
pub struct Person {
    pub name: String,
}

impl Greet for Person {
    fn hello(&self) -> String {
        format!("Hello, I'm {}!", self.name)
    }
}

/// A robot who can greet.
pub struct Robot {
    pub id: u32,
}

impl Greet for Robot {
    fn hello(&self) -> String {
        format!("Beep boop. Unit {} online.", self.id)
    }
}
