use vec_sorting::sort_by_length;

#[test]
fn test_basic_sort() {
    let mut words = vec![
        String::from("banana"),
        String::from("apple"),
        String::from("fig"),
        String::from("date"),
    ];
    sort_by_length(&mut words);
    assert_eq!(words, vec!["fig", "date", "apple", "banana"]);
}

#[test]
fn test_same_length() {
    let mut words = vec![
        String::from("cat"),
        String::from("bat"),
        String::from("ant"),
    ];
    sort_by_length(&mut words);
    assert_eq!(words, vec!["ant", "bat", "cat"]);
}

#[test]
fn test_empty() {
    let mut words: Vec<String> = vec![];
    sort_by_length(&mut words);
    assert_eq!(words, Vec::<String>::new());
}
