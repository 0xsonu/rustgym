use std::collections::HashMap;
use merge_maps::merge_maps;

#[test]
fn test_no_overlap() {
    let a: HashMap<String, i32> = [("x".into(), 1)].into_iter().collect();
    let b: HashMap<String, i32> = [("y".into(), 2)].into_iter().collect();
    let result = merge_maps(&a, &b);
    assert_eq!(result.get("x"), Some(&1));
    assert_eq!(result.get("y"), Some(&2));
}

#[test]
fn test_overlap() {
    let a: HashMap<String, i32> = [("x".into(), 1)].into_iter().collect();
    let b: HashMap<String, i32> = [("x".into(), 5)].into_iter().collect();
    let result = merge_maps(&a, &b);
    assert_eq!(result.get("x"), Some(&6));
}

#[test]
fn test_empty_maps() {
    let a: HashMap<String, i32> = HashMap::new();
    let b: HashMap<String, i32> = HashMap::new();
    assert!(merge_maps(&a, &b).is_empty());
}
