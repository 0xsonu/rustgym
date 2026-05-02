/// Processes a list of number strings: parses, filters positives, and sums them.
pub fn sum_positive_numbers(inputs: &[&str]) -> Result<i32, String> {
    let mut sum = 0;
    for input in inputs {
        let num: i32 = input
            .trim()
            .parse()
            .map_err(|_| format!("failed to parse '{}'", input))?;
        if num > 0 {
            sum += num;
        }
    }
    Ok(sum)
}
