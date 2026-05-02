/// A trait for things that can be summarized.
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Article {
    pub title: String,
    pub author: String,
}

/// TODO: Implement Summary for Article. Return "{title} by {author}".
impl Summary for Article {
    fn summarize(&self) -> String {
        todo!()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

/// TODO: Implement Summary for Tweet. Return "@{username}: {content}".
impl Summary for Tweet {
    fn summarize(&self) -> String {
        todo!()
    }
}

/// Takes anything that implements Summary and wraps it in a notification.
/// TODO: Return "Breaking news! {item.summarize()}"
pub fn notify(item: &impl Summary) -> String {
    todo!()
}
