/// Duplicates a Vec by cloning it. Returns both the original and the clone.
pub fn duplicate_vec(v: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let cloned = v.clone();
    (v, cloned)
}

/// Appends a value to a cloned Vec, returning both original and modified.
pub fn clone_and_push(v: Vec<i32>, value: i32) -> (Vec<i32>, Vec<i32>) {
    let mut cloned = v.clone();
    cloned.push(value);
    (v, cloned)
}
