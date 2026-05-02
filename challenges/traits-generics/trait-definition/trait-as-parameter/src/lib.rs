/// A trait for things that can be summarized.
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Article {
    pub title: String,
    pub author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

/// Takes anything that implements Summary and wraps it in a notification.
pub fn notify(item: &impl Summary) -> String {
    format!("Breaking news! {}", item.summarize())
}
