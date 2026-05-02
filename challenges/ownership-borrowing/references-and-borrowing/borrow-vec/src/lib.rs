/// Returns the sum and average of a borrowed vector.
pub fn sum_and_average(numbers: &Vec<f64>) -> (f64, f64) {
    if numbers.is_empty() {
        return (0.0, 0.0);
    }
    let sum: f64 = numbers.iter().sum();
    let avg = sum / numbers.len() as f64;
    (sum, avg)
}
