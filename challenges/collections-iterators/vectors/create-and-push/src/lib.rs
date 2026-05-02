/// Creates a vector containing numbers from 1 to n.
pub fn build_sequence(n: u32) -> Vec<u32> {
    let mut v = Vec::new();
    for i in 1..=n {
        v.push(i);
    }
    v
}
