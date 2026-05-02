/// Removes all negative numbers from the vector in place.
pub fn remove_negatives(numbers: &mut Vec<i32>) {
    numbers.retain(|&n| n >= 0);
}
