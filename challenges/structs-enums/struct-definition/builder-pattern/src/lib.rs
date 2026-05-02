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
    pub fn new() -> Self {
        EmailBuilder {
            to: String::new(),
            subject: String::new(),
            body: String::new(),
        }
    }

    /// Sets the recipient.
    pub fn to(mut self, to: &str) -> Self {
        self.to = String::from(to);
        self
    }

    /// Sets the subject.
    pub fn subject(mut self, subject: &str) -> Self {
        self.subject = String::from(subject);
        self
    }

    /// Sets the body.
    pub fn body(mut self, body: &str) -> Self {
        self.body = String::from(body);
        self
    }

    /// Builds the final Email.
    pub fn build(self) -> Email {
        Email {
            to: self.to,
            subject: self.subject,
            body: self.body,
        }
    }
}
