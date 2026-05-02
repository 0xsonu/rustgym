use fizzbuzz::fizzbuzz;

#[test]
fn test_fizzbuzz() {
    assert_eq!(fizzbuzz(15), "FizzBuzz");
}

#[test]
fn test_fizz() {
    assert_eq!(fizzbuzz(9), "Fizz");
}

#[test]
fn test_buzz() {
    assert_eq!(fizzbuzz(10), "Buzz");
}

#[test]
fn test_number() {
    assert_eq!(fizzbuzz(7), "7");
}

#[test]
fn test_fizzbuzz_30() {
    assert_eq!(fizzbuzz(30), "FizzBuzz");
}
