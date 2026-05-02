use frequency_counter::char_frequency;

#[test]
fn test_basic() {
    let result = char_frequency("aabbc");
    assert_eq!(result[0], ('a', 2));
    assert_eq!(result[1], ('b', 2));
    assert_eq!(result[2], ('c', 1));
}

#[test]
fn test_skips_whitespace() {
    let result = char_frequency("a b c");
    assert_eq!(result.len(), 3);
}

#[test]
fn test_empty() {
    assert_eq!(char_frequency(""), Vec::<(char, usize)>::new());
}

#[test]
fn test_single_char() {
    assert_eq!(char_frequency("aaaa"), vec![('a', 4)]);
}
