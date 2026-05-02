use std::fmt;

/// Error type for a calculator.
/// TODO: Define DivisionByZero, Overflow, and InvalidOperator(char) variants.
#[derive(Debug, PartialEq)]
pub enum CalcError {
    // Add variants here
}

// TODO: Implement Display for CalcError.
impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// Performs a basic calculation.
/// TODO: Support +, -, *, / with checked arithmetic.
pub fn calculate(a: i64, op: char, b: i64) -> Result<i64, CalcError> {
    todo!()
}
