use std::collections::BTreeMap;
use range_queries::range_query;

#[test]
fn test_range() {
    let mut map = BTreeMap::new();
    map.insert(1, String::from("one"));
    map.insert(3, String::from("three"));
    map.insert(5, String::from("five"));
    map.insert(7, String::from("seven"));

    let result = range_query(&map, 2, 6);
    assert_eq!(result, vec![(3, String::from("three")), (5, String::from("five"))]);
}

#[test]
fn test_empty_range() {
    let mut map = BTreeMap::new();
    map.insert(1, String::from("one"));
    map.insert(10, String::from("ten"));

    let result = range_query(&map, 3, 5);
    assert!(result.is_empty());
}

#[test]
fn test_full_range() {
    let mut map = BTreeMap::new();
    map.insert(1, String::from("a"));
    map.insert(2, String::from("b"));

    let result = range_query(&map, 0, 10);
    assert_eq!(result.len(), 2);
}
