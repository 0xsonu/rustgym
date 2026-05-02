use move_in_loops::build_sentence;

#[test]
fn test_basic_sentence() {
    let words = vec![
        String::from("Hello"),
        String::from("World"),
    ];
    assert_eq!(build_sentence(words), "Hello World");
}

#[test]
fn test_single_word() {
    let words = vec![String::from("Rust")];
    assert_eq!(build_sentence(words), "Rust");
}

#[test]
fn test_empty() {
    let words: Vec<String> = vec![];
    assert_eq!(build_sentence(words), "");
}

#[test]
fn test_multiple_words() {
    let words = vec![
        String::from("I"),
        String::from("love"),
        String::from("Rust"),
    ];
    assert_eq!(build_sentence(words), "I love Rust");
}
