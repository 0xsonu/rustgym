/// Returns (sum, difference, product, quotient, remainder) of a and b.
pub fn calculate(a: i32, b: i32) -> (i32, i32, i32, i32, i32) {
    (a + b, a - b, a * b, a / b, a % b)
}
