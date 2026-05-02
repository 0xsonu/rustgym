/// A string builder that constructs strings piece by piece.
pub struct StringBuilder {
    parts: Vec<String>,
}

impl StringBuilder {
    /// Creates a new empty StringBuilder.
    /// TODO: Return a StringBuilder with an empty Vec.
    pub fn new() -> Self {
        todo!()
    }

    /// Adds a string part.
    /// TODO: Push the part to self.parts and return self for chaining.
    pub fn add(mut self, part: &str) -> Self {
        todo!()
    }

    /// Adds a newline.
    /// TODO: Push "\n" to self.parts and return self for chaining.
    pub fn newline(mut self) -> Self {
        todo!()
    }

    /// Builds the final string by joining all parts.
    /// TODO: Join all parts with "" separator and return the result.
    pub fn build(self) -> String {
        todo!()
    }
}
