use set_operations::{intersection, union};

#[test]
fn test_intersection() {
    assert_eq!(intersection(&[1, 2, 3], &[2, 3, 4]), vec![2, 3]);
}

#[test]
fn test_intersection_empty() {
    assert_eq!(intersection(&[1, 2], &[3, 4]), Vec::<i32>::new());
}

#[test]
fn test_union() {
    assert_eq!(union(&[1, 2, 3], &[2, 3, 4]), vec![1, 2, 3, 4]);
}

#[test]
fn test_union_same() {
    assert_eq!(union(&[1, 2], &[1, 2]), vec![1, 2]);
}
