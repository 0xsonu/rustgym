use clone_vs_copy::{clone_and_push, duplicate_vec};

#[test]
fn test_duplicate_vec() {
    let v = vec![1, 2, 3];
    let (orig, cloned) = duplicate_vec(v);
    assert_eq!(orig, vec![1, 2, 3]);
    assert_eq!(cloned, vec![1, 2, 3]);
}

#[test]
fn test_duplicate_empty() {
    let v: Vec<i32> = vec![];
    let (orig, cloned) = duplicate_vec(v);
    assert!(orig.is_empty());
    assert!(cloned.is_empty());
}

#[test]
fn test_clone_and_push() {
    let v = vec![1, 2, 3];
    let (orig, modified) = clone_and_push(v, 4);
    assert_eq!(orig, vec![1, 2, 3]);
    assert_eq!(modified, vec![1, 2, 3, 4]);
}

#[test]
fn test_clone_and_push_empty() {
    let v: Vec<i32> = vec![];
    let (orig, modified) = clone_and_push(v, 42);
    assert!(orig.is_empty());
    assert_eq!(modified, vec![42]);
}
