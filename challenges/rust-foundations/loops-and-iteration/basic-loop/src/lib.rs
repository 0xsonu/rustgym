/// Finds the first power of 2 greater than the threshold.
pub fn find_first_power_of_two_above(threshold: u32) -> u32 {
    let mut value = 1;
    loop {
        if value > threshold {
            break value;
        }
        value *= 2;
    }
}
