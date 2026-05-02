/// Sorts strings by length, then alphabetically for equal lengths.
pub fn sort_by_length(words: &mut Vec<String>) {
    words.sort_by(|a, b| {
        a.len().cmp(&b.len()).then_with(|| a.cmp(b))
    });
}
