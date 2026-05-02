/// Generates a multiplication table as a vector of strings.
/// Each string is in the format "a x b = c".
pub fn multiplication_table(size: u32) -> Vec<String> {
    let mut results = Vec::new();
    for i in 1..=size {
        for j in 1..=size {
            results.push(format!("{} x {} = {}", i, j, i * j));
        }
    }
    results
}
