/// A string builder that constructs strings piece by piece.
pub struct StringBuilder {
    parts: Vec<String>,
}

impl StringBuilder {
    /// Creates a new empty StringBuilder.
    pub fn new() -> Self {
        StringBuilder { parts: Vec::new() }
    }

    /// Adds a string part.
    pub fn add(mut self, part: &str) -> Self {
        self.parts.push(String::from(part));
        self
    }

    /// Adds a newline.
    pub fn newline(mut self) -> Self {
        self.parts.push(String::from("\n"));
        self
    }

    /// Builds the final string by joining all parts.
    pub fn build(self) -> String {
        self.parts.join("")
    }
}
