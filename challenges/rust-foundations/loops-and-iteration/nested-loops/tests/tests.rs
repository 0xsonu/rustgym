use nested_loops::multiplication_table;

#[test]
fn test_size_2() {
    let table = multiplication_table(2);
    assert_eq!(table.len(), 4);
    assert_eq!(table[0], "1 x 1 = 1");
    assert_eq!(table[3], "2 x 2 = 4");
}

#[test]
fn test_size_3() {
    let table = multiplication_table(3);
    assert_eq!(table.len(), 9);
    assert_eq!(table[8], "3 x 3 = 9");
}

#[test]
fn test_size_1() {
    let table = multiplication_table(1);
    assert_eq!(table, vec!["1 x 1 = 1"]);
}
