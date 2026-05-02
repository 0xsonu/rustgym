/// An email message.
pub struct Email {
    pub to: String,
    pub subject: String,
    pub body: String,
}

/// A builder for constructing Email instances step by step.
pub struct EmailBuilder {
    to: String,
    subject: String,
    body: String,
}

impl EmailBuilder {
    /// Creates a new EmailBuilder with empty fields.
    /// TODO: Return an EmailBuilder with empty strings.
    pub fn new() -> Self {
        todo!()
    }

    /// Sets the recipient.
    /// TODO: Set self.to and return self for chaining.
    pub fn to(mut self, to: &str) -> Self {
        todo!()
    }

    /// Sets the subject.
    /// TODO: Set self.subject and return self for chaining.
    pub fn subject(mut self, subject: &str) -> Self {
        todo!()
    }

    /// Sets the body.
    /// TODO: Set self.body and return self for chaining.
    pub fn body(mut self, body: &str) -> Self {
        todo!()
    }

    /// Builds the final Email.
    /// TODO: Create and return an Email from the builder fields.
    pub fn build(self) -> Email {
        todo!()
    }
}
