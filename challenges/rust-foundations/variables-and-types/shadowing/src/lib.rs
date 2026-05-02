/// Demonstrates variable shadowing by transforming a value.
pub fn shadow_transform() -> String {
    let x = 5;
    let x = x * 2;
    let x = x.to_string();
    x
}
