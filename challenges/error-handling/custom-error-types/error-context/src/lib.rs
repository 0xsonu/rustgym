use std::fmt;

/// Error type for a calculator that provides context.
#[derive(Debug, PartialEq)]
pub enum CalcError {
    DivisionByZero,
    Overflow,
    InvalidOperator(char),
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::DivisionByZero => write!(f, "cannot divide by zero"),
            CalcError::Overflow => write!(f, "arithmetic overflow"),
            CalcError::InvalidOperator(op) => write!(f, "invalid operator: '{}'", op),
        }
    }
}

/// Performs a basic calculation.
pub fn calculate(a: i64, op: char, b: i64) -> Result<i64, CalcError> {
    match op {
        '+' => a.checked_add(b).ok_or(CalcError::Overflow),
        '-' => a.checked_sub(b).ok_or(CalcError::Overflow),
        '*' => a.checked_mul(b).ok_or(CalcError::Overflow),
        '/' => {
            if b == 0 {
                Err(CalcError::DivisionByZero)
            } else {
                a.checked_div(b).ok_or(CalcError::Overflow)
            }
        }
        other => Err(CalcError::InvalidOperator(other)),
    }
}
