/// Returns true if exactly one of a or b is true (XOR).
pub fn exclusive_or(a: bool, b: bool) -> bool {
    (a || b) && !(a && b)
}

/// Returns true if all three values are the same.
pub fn all_same(a: bool, b: bool, c: bool) -> bool {
    (a && b && c) || (!a && !b && !c)
}

/// Returns true if at least two of the three values are true.
pub fn majority(a: bool, b: bool, c: bool) -> bool {
    (a && b) || (a && c) || (b && c)
}
