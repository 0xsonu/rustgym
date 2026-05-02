use vec_operations::rotate_left;

#[test]
fn test_rotate_by_2() {
    let mut v = vec![1, 2, 3, 4, 5];
    rotate_left(&mut v, 2);
    assert_eq!(v, vec![3, 4, 5, 1, 2]);
}

#[test]
fn test_rotate_by_0() {
    let mut v = vec![1, 2, 3];
    rotate_left(&mut v, 0);
    assert_eq!(v, vec![1, 2, 3]);
}

#[test]
fn test_rotate_full() {
    let mut v = vec![1, 2, 3];
    rotate_left(&mut v, 3);
    assert_eq!(v, vec![1, 2, 3]);
}

#[test]
fn test_rotate_empty() {
    let mut v: Vec<i32> = vec![];
    rotate_left(&mut v, 2);
    assert_eq!(v, Vec::<i32>::new());
}
