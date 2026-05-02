use btreemap_basics::sorted_scores;

#[test]
fn test_sorted_output() {
    let scores = vec![
        (String::from("Charlie"), 80),
        (String::from("Alice"), 90),
        (String::from("Bob"), 85),
    ];
    let result = sorted_scores(&scores);
    assert_eq!(result[0].0, "Alice");
    assert_eq!(result[1].0, "Bob");
    assert_eq!(result[2].0, "Charlie");
}

#[test]
fn test_duplicate_names() {
    let scores = vec![
        (String::from("Alice"), 70),
        (String::from("Alice"), 95),
    ];
    let result = sorted_scores(&scores);
    assert_eq!(result, vec![(String::from("Alice"), 95)]);
}

#[test]
fn test_empty() {
    let scores: Vec<(String, u32)> = vec![];
    assert!(sorted_scores(&scores).is_empty());
}
