use group_by::group_by_length;

#[test]
fn test_basic() {
    let groups = group_by_length(&["hi", "hey", "hello", "yo"]);
    assert_eq!(groups.get(&2).unwrap(), &vec!["hi", "yo"]);
    assert_eq!(groups.get(&3).unwrap(), &vec!["hey"]);
    assert_eq!(groups.get(&5).unwrap(), &vec!["hello"]);
}

#[test]
fn test_empty() {
    let groups = group_by_length(&[]);
    assert!(groups.is_empty());
}

#[test]
fn test_same_length() {
    let groups = group_by_length(&["cat", "dog", "bat"]);
    assert_eq!(groups.len(), 1);
    assert_eq!(groups.get(&3).unwrap().len(), 3);
}
