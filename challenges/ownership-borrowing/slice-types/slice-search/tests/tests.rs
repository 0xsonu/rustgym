use slice_search::has_subslice_sum;

#[test]
fn test_found() {
    assert!(has_subslice_sum(&[1, 2, 3, 4, 5], 9)); // 2+3+4
}

#[test]
fn test_single_element() {
    assert!(has_subslice_sum(&[1, 2, 3], 2));
}

#[test]
fn test_not_found() {
    assert!(!has_subslice_sum(&[1, 2, 3], 7));
}

#[test]
fn test_empty() {
    assert!(!has_subslice_sum(&[], 0));
}

#[test]
fn test_full_sum() {
    assert!(has_subslice_sum(&[1, 2, 3], 6));
}
