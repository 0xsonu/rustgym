use iterator_impl::Counter;

#[test]
fn test_counter_basic() {
    let counter = Counter::new(3);
    let values: Vec<u32> = counter.collect();
    assert_eq!(values, vec![1, 2, 3]);
}

#[test]
fn test_counter_one() {
    let counter = Counter::new(1);
    let values: Vec<u32> = counter.collect();
    assert_eq!(values, vec![1]);
}

#[test]
fn test_counter_zero() {
    let counter = Counter::new(0);
    let values: Vec<u32> = counter.collect();
    assert!(values.is_empty());
}

#[test]
fn test_counter_sum() {
    let sum: u32 = Counter::new(5).sum();
    assert_eq!(sum, 15);
}
