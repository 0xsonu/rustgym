/// Parses a list of number strings and returns their sum.
pub fn sum_strings(inputs: &[&str]) -> Result<i32, String> {
    let mut total = 0;
    for input in inputs {
        let num: i32 = input
            .parse()
            .map_err(|_| format!("cannot parse '{}'", input))?;
        total += num;
    }
    Ok(total)
}
