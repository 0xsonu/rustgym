use trait_as_parameter::{notify, Article, Tweet};

#[test]
fn test_notify_article() {
    let article = Article {
        title: String::from("Rust 2024"),
        author: String::from("The Rust Team"),
    };
    assert_eq!(notify(&article), "Breaking news! Rust 2024 by The Rust Team");
}

#[test]
fn test_notify_tweet() {
    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Rust is awesome!"),
    };
    assert_eq!(notify(&tweet), "Breaking news! @rustlang: Rust is awesome!");
}

#[test]
fn test_article_summarize() {
    let article = Article {
        title: String::from("Hello"),
        author: String::from("World"),
    };
    assert_eq!(article.summarize(), "Hello by World");
}
