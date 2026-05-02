/// Finds the common elements between two slices.
pub fn common_elements<'a>(a: &'a [i32], b: &[i32]) -> Vec<i32> {
    a.iter()
        .filter(|item| b.contains(item))
        .copied()
        .collect()
}
