/// Returns a sorted vector with duplicates removed.
pub fn unique_sorted(mut numbers: Vec<i32>) -> Vec<i32> {
    numbers.sort();
    numbers.dedup();
    numbers
}
