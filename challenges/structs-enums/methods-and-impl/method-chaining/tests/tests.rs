use method_chaining::StringBuilder;

#[test]
fn test_basic_build() {
    let result = StringBuilder::new()
        .add("Hello")
        .add(" ")
        .add("World")
        .build();
    assert_eq!(result, "Hello World");
}

#[test]
fn test_with_newline() {
    let result = StringBuilder::new()
        .add("Line 1")
        .newline()
        .add("Line 2")
        .build();
    assert_eq!(result, "Line 1\nLine 2");
}

#[test]
fn test_empty() {
    let result = StringBuilder::new().build();
    assert_eq!(result, "");
}

#[test]
fn test_single_part() {
    let result = StringBuilder::new().add("only").build();
    assert_eq!(result, "only");
}
