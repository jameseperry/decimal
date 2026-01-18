use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecimalError {
    Empty,
    Invalid,
    TooManyFractionalDigits { provided: usize, allowed: usize },
    DivisionByZero,
    Overflow,
}

impl fmt::Display for DecimalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecimalError::Empty => write!(f, "empty input"),
            DecimalError::Invalid => write!(f, "invalid format"),
            DecimalError::TooManyFractionalDigits { provided, allowed } => {
                write!(
                    f,
                    "too many fractional digits (provided {}, allowed {})",
                    provided, allowed
                )
            }
            DecimalError::DivisionByZero => write!(f, "division by zero"),
            DecimalError::Overflow => write!(f, "value out of range"),
        }
    }
}

impl std::error::Error for DecimalError {}
