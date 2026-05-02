/// A trait for things that have a name and can introduce themselves.
pub trait Introduce {
    /// Required: returns the name.
    fn name(&self) -> &str;

    /// Default: returns a full introduction using name().
    fn introduce(&self) -> String {
        format!("Hi, my name is {}.", self.name())
    }
}

pub struct Student {
    pub name: String,
}

impl Introduce for Student {
    fn name(&self) -> &str {
        &self.name
    }
    // Uses default introduce()
}

pub struct Teacher {
    pub name: String,
    pub subject: String,
}

impl Introduce for Teacher {
    fn name(&self) -> &str {
        &self.name
    }

    // Overrides default introduce()
    fn introduce(&self) -> String {
        format!("I'm {}, and I teach {}.", self.name, self.subject)
    }
}
