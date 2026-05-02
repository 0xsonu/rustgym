use std::collections::HashSet;

/// Returns the intersection of two sets.
pub fn intersection(a: &[i32], b: &[i32]) -> Vec<i32> {
    let set_a: HashSet<i32> = a.iter().copied().collect();
    let set_b: HashSet<i32> = b.iter().copied().collect();
    let mut result: Vec<i32> = set_a.intersection(&set_b).copied().collect();
    result.sort();
    result
}

/// Returns the union of two sets.
pub fn union(a: &[i32], b: &[i32]) -> Vec<i32> {
    let set_a: HashSet<i32> = a.iter().copied().collect();
    let set_b: HashSet<i32> = b.iter().copied().collect();
    let mut result: Vec<i32> = set_a.union(&set_b).copied().collect();
    result.sort();
    result
}
