/// Sums all integers from start to end (inclusive).
pub fn sum_range(start: i32, end: i32) -> i32 {
    if start > end {
        return 0;
    }
    let mut sum = 0;
    for i in start..=end {
        sum += i;
    }
    sum
}
