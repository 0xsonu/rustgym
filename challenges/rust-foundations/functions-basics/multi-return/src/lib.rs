/// Analyzes text and returns (char_count, word_count, sentence_count).
pub fn analyze_text(text: &str) -> (usize, usize, usize) {
    let char_count = text.chars().filter(|c| !c.is_whitespace()).count();
    let word_count = text.split_whitespace().count();
    let sentence_count = text.chars().filter(|c| *c == '.' || *c == '!' || *c == '?').count();
    (char_count, word_count, sentence_count)
}
