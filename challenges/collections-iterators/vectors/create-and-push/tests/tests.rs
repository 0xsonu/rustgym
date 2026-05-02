use create_and_push::build_sequence;

#[test]
fn test_five() {
    assert_eq!(build_sequence(5), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_one() {
    assert_eq!(build_sequence(1), vec![1]);
}

#[test]
fn test_zero() {
    assert_eq!(build_sequence(0), Vec::<u32>::new());
}
