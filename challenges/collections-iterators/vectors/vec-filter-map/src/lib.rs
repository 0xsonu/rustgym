/// Returns positive numbers doubled.
pub fn extract_positives(numbers: &[i32]) -> Vec<i32> {
    numbers.iter()
        .filter(|&&n| n > 0)
        .map(|&n| n * 2)
        .collect()
}
