/// Returns true if the slice contains a sub-sequence that sums to the target.
pub fn has_subslice_sum(numbers: &[i32], target: i32) -> bool {
    for start in 0..numbers.len() {
        let mut sum = 0;
        for end in start..numbers.len() {
            sum += numbers[end];
            if sum == target {
                return true;
            }
        }
    }
    false
}
