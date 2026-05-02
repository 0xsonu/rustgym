/// Takes a Vec, adds an element, and returns the modified Vec.
pub fn push_and_return(mut vec: Vec<i32>, value: i32) -> Vec<i32> {
    vec.push(value);
    vec
}

/// Takes a Vec and returns its sum and the Vec itself.
pub fn sum_and_return(vec: Vec<i32>) -> (i32, Vec<i32>) {
    let sum: i32 = vec.iter().sum();
    (sum, vec)
}
