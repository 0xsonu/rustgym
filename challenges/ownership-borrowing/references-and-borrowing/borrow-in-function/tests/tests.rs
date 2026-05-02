use borrow_in_function::count_vowels;

#[test]
fn test_hello_world() {
    assert_eq!(count_vowels("hello world"), 3);
}

#[test]
fn test_no_vowels() {
    assert_eq!(count_vowels("rhythm"), 0);
}

#[test]
fn test_all_vowels() {
    assert_eq!(count_vowels("aeiou"), 5);
}

#[test]
fn test_uppercase() {
    assert_eq!(count_vowels("HELLO"), 2);
}
