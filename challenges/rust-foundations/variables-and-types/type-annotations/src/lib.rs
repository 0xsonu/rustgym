/// Returns a tuple with one value of each primitive type.
pub fn describe_types() -> (i32, f64, bool, char) {
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'R';
    (integer, float, boolean, character)
}
