/// A trait for things that have a name and can introduce themselves.
pub trait Introduce {
    /// Required: returns the name.
    fn name(&self) -> &str;

    /// Default: returns a full introduction using name().
    /// TODO: Provide a default implementation that returns "Hi, my name is {name}."
    fn introduce(&self) -> String {
        todo!()
    }
}

pub struct Student {
    pub name: String,
}

/// TODO: Implement Introduce for Student (only implement name(), use default introduce).
impl Introduce for Student {
    fn name(&self) -> &str {
        todo!()
    }
}

pub struct Teacher {
    pub name: String,
    pub subject: String,
}

/// TODO: Implement Introduce for Teacher (implement both name() and override introduce()).
impl Introduce for Teacher {
    fn name(&self) -> &str {
        todo!()
    }

    fn introduce(&self) -> String {
        todo!()
    }
}
