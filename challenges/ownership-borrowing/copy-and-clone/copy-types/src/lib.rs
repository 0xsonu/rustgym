/// Demonstrates that Copy types can be used multiple times.
pub fn demonstrate_copy(x: i32) -> (i32, i32, i32) {
    let doubled = x * 2;
    let tripled = x * 3;
    (x, doubled, tripled)
}
