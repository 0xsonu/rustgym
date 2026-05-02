/// Returns the absolute value of n using an if expression.
pub fn absolute_value(n: i32) -> i32 {
    let result = if n < 0 { -n } else { n };
    result
}
