use std::fmt;
use std::num::IntErrorKind;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Decimal<const SCALE: u32> {
    minor_units: i64,
}

#[allow(dead_code)]
const MAX_SCALE: u32 = 18;

impl<const SCALE: u32> Decimal<SCALE> {
    const _ASSERT_SCALE: () = assert!(SCALE <= MAX_SCALE);

    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.minor_units
            .checked_add(rhs.minor_units)
            .map(|minor_units| Self { minor_units })
    }

    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.minor_units
            .checked_sub(rhs.minor_units)
            .map(|minor_units| Self { minor_units })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecimalParseError {
    Empty,
    Invalid,
    TooManyFractionalDigits { provided: usize, allowed: usize },
    Overflow,
}

impl fmt::Display for DecimalParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecimalParseError::Empty => write!(f, "empty input"),
            DecimalParseError::Invalid => write!(f, "invalid format"),
            DecimalParseError::TooManyFractionalDigits { provided, allowed } => {
                write!(
                    f,
                    "too many fractional digits (provided {}, allowed {})",
                    provided, allowed
                )
            }
            DecimalParseError::Overflow => write!(f, "value out of range"),
        }
    }
}

impl std::error::Error for DecimalParseError {}

impl<const SCALE: u32> FromStr for Decimal<SCALE> {
    type Err = DecimalParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(DecimalParseError::Empty);
        }

        let (negative, unsigned) = if let Some(rest) = s.strip_prefix('-') {
            (true, rest)
        } else if let Some(rest) = s.strip_prefix('+') {
            (false, rest)
        } else {
            (false, s)
        };

        let (int_part, frac_part) = match unsigned.split_once('.') {
            Some((int_part, frac_part)) => (int_part, frac_part),
            None => (unsigned, ""),
        };

        if int_part.is_empty() && frac_part.is_empty() {
            return Err(DecimalParseError::Invalid);
        }

        if int_part.starts_with(['+', '-']) || frac_part.starts_with(['+', '-']) {
            return Err(DecimalParseError::Invalid);
        }

        let decimals = SCALE as usize;
        if frac_part.len() > decimals {
            return Err(DecimalParseError::TooManyFractionalDigits {
                provided: frac_part.len(),
                allowed: decimals,
            });
        }

        let parse_i64 = |input: &str| {
            input.parse::<i64>().map_err(|err| match err.kind() {
                IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => {
                    DecimalParseError::Overflow
                }
                _ => DecimalParseError::Invalid,
            })
        };

        let int_val = if int_part.is_empty() { 0 } else { parse_i64(int_part)? };
        let frac_val = if frac_part.is_empty() { 0 } else { parse_i64(frac_part)? };

        let int_scale = 10_i64.pow(SCALE);
        let frac_scale = 10_i64.pow(SCALE - (frac_part.len() as u32));

        let scaled_int_part = int_val
            .checked_mul(int_scale)
            .ok_or(DecimalParseError::Overflow)?;

        let scaled_frac_part = frac_val
            .checked_mul(frac_scale)
            .ok_or(DecimalParseError::Overflow)?;

        let minor = scaled_int_part
            .checked_add(scaled_frac_part)
            .ok_or(DecimalParseError::Overflow)?;
    
        let signed = if negative {
            minor.checked_neg().ok_or(DecimalParseError::Overflow)?
        } else {
            minor
        };
        Ok(Self { minor_units: signed })
    }
}

impl<const SCALE: u32> fmt::Display for Decimal<SCALE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if SCALE == 0 {
            return write!(f, "{}", self.minor_units);
        }

        let decimals = SCALE as usize;
        let scale = 10_i128.pow(decimals as u32);

        let minor = self.minor_units as i128;
        let negative = minor < 0;
        let abs = if negative { -minor } else { minor };
        let int_part = abs / scale;
        let frac_part = abs % scale;

        if negative {
            write!(f, "-{}.{:0width$}", int_part, frac_part, width = decimals)
        } else {
            write!(f, "{}.{:0width$}", int_part, frac_part, width = decimals)
        }
    }
}

impl<const SCALE: u32> Add for Decimal<SCALE> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            minor_units: self.minor_units + rhs.minor_units,
        }
    }
}

impl<const SCALE: u32> Sub for Decimal<SCALE> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            minor_units: self.minor_units - rhs.minor_units,
        }
    }
}
impl<const SCALE: u32> AddAssign for Decimal<SCALE> {
    fn add_assign(&mut self, rhs: Self) {
        self.minor_units += rhs.minor_units;
    }
}

impl<const SCALE: u32> SubAssign for Decimal<SCALE> {
    fn sub_assign(&mut self, rhs: Self) {
        self.minor_units -= rhs.minor_units;
    }
}
