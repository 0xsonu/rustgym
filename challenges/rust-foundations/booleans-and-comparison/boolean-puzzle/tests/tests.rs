use boolean_puzzle::{all_same, exclusive_or, majority};

#[test]
fn test_xor_true_cases() {
    assert!(exclusive_or(true, false));
    assert!(exclusive_or(false, true));
}

#[test]
fn test_xor_false_cases() {
    assert!(!exclusive_or(true, true));
    assert!(!exclusive_or(false, false));
}

#[test]
fn test_all_same_true() {
    assert!(all_same(true, true, true));
    assert!(all_same(false, false, false));
}

#[test]
fn test_all_same_false() {
    assert!(!all_same(true, true, false));
    assert!(!all_same(true, false, true));
}

#[test]
fn test_majority_true() {
    assert!(majority(true, true, false));
    assert!(majority(true, true, true));
}

#[test]
fn test_majority_false() {
    assert!(!majority(true, false, false));
    assert!(!majority(false, false, false));
}
