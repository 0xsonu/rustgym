/// Splits a string at the first occurrence of a delimiter and returns both parts.
pub fn split_once_at(text: &str, delimiter: char) -> (&str, &str) {
    match text.find(delimiter) {
        Some(pos) => (&text[..pos], &text[pos + delimiter.len_utf8()..]),
        None => (text, ""),
    }
}
