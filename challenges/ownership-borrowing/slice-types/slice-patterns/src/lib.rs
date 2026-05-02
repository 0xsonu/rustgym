/// Returns the middle portion of a slice (excluding first and last).
pub fn middle_elements(slice: &[i32]) -> &[i32] {
    if slice.len() < 3 {
        &[]
    } else {
        &slice[1..slice.len() - 1]
    }
}
