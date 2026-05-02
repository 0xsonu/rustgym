/// Builds a string by appending each word from the list with a space separator.
pub fn build_sentence(words: Vec<String>) -> String {
    let mut result = String::new();
    for (i, word) in words.into_iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        result.push_str(&word);
    }
    result
}
