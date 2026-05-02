/// Returns (absolute value, sqrt of absolute value, rounded value) for x.
pub fn float_math(x: f64) -> (f64, f64, f64) {
    let abs = x.abs();
    let sqrt_of_abs = abs.sqrt();
    let rounded = x.round();
    (abs, sqrt_of_abs, rounded)
}
