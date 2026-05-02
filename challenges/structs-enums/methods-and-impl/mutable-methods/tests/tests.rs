use mutable_methods::Counter;

#[test]
fn test_new() {
    let c = Counter::new();
    assert_eq!(c.get(), 0);
}

#[test]
fn test_increment() {
    let mut c = Counter::new();
    c.increment();
    c.increment();
    assert_eq!(c.get(), 2);
}

#[test]
fn test_decrement() {
    let mut c = Counter::new();
    c.decrement();
    assert_eq!(c.get(), -1);
}

#[test]
fn test_reset() {
    let mut c = Counter::new();
    c.increment();
    c.increment();
    c.increment();
    c.reset();
    assert_eq!(c.get(), 0);
}
