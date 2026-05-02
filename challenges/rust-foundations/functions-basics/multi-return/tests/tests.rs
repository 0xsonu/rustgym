use multi_return::analyze_text;

#[test]
fn test_simple_sentence() {
    assert_eq!(analyze_text("Hello world."), (10, 2, 1));
}

#[test]
fn test_multiple_sentences() {
    assert_eq!(analyze_text("Hi there! How are you?"), (17, 4, 2));
}

#[test]
fn test_empty_string() {
    assert_eq!(analyze_text(""), (0, 0, 0));
}

#[test]
fn test_no_punctuation() {
    assert_eq!(analyze_text("just words here"), (13, 3, 0));
}
