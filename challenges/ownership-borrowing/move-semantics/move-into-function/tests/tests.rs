use move_into_function::{push_and_return, sum_and_return};

#[test]
fn test_push_and_return() {
    let v = vec![1, 2, 3];
    let result = push_and_return(v, 4);
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn test_push_to_empty() {
    let v: Vec<i32> = vec![];
    let result = push_and_return(v, 42);
    assert_eq!(result, vec![42]);
}

#[test]
fn test_sum_and_return() {
    let v = vec![1, 2, 3, 4, 5];
    let (sum, returned) = sum_and_return(v);
    assert_eq!(sum, 15);
    assert_eq!(returned, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_sum_empty() {
    let v: Vec<i32> = vec![];
    let (sum, returned) = sum_and_return(v);
    assert_eq!(sum, 0);
    assert_eq!(returned, Vec::<i32>::new());
}
