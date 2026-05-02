/// Finds the first number in the array greater than the threshold.
/// Returns None if no such number exists.
pub fn find_first_above(numbers: &[i32], threshold: i32) -> Option<i32> {
    for &num in numbers {
        if num > threshold {
            return Some(num);
        }
    }
    None
}
