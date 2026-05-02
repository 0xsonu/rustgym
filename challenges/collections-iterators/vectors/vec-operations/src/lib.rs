/// Rotates a vector left by n positions.
pub fn rotate_left(v: &mut Vec<i32>, n: usize) {
    if v.is_empty() {
        return;
    }
    let n = n % v.len();
    let drained: Vec<i32> = v.drain(..n).collect();
    v.extend(drained);
}
