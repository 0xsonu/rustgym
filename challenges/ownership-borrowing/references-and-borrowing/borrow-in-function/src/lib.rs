/// Counts the number of vowels in a string.
pub fn count_vowels(text: &str) -> usize {
    text.chars()
        .filter(|c| "aeiouAEIOU".contains(*c))
        .count()
}
