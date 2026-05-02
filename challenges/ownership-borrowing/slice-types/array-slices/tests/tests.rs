use array_slices::sum_slice;

#[test]
fn test_full_array() {
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(sum_slice(&arr), 15);
}

#[test]
fn test_partial_slice() {
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(sum_slice(&arr[1..4]), 9);
}

#[test]
fn test_empty_slice() {
    let arr: [i32; 0] = [];
    assert_eq!(sum_slice(&arr), 0);
}

#[test]
fn test_single_element() {
    assert_eq!(sum_slice(&[42]), 42);
}
