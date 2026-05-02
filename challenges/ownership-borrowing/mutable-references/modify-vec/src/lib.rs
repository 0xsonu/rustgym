/// Doubles every element in the vector in place.
pub fn double_elements(numbers: &mut Vec<i32>) {
    for num in numbers.iter_mut() {
        *num *= 2;
    }
}
