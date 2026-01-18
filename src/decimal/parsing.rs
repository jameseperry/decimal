use std::num::IntErrorKind;
use std::str::FromStr;

use crate::decimal::{Decimal, DecimalError, DecimalInt};

impl<T: DecimalInt, const SCALE: u32> FromStr for Decimal<T, SCALE> {
    type Err = DecimalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(DecimalError::Empty);
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
            return Err(DecimalError::Invalid);
        }

        if int_part.starts_with(['+', '-']) || frac_part.starts_with(['+', '-']) {
            return Err(DecimalError::Invalid);
        }

        let decimals = SCALE as usize;
        if frac_part.len() > decimals {
            return Err(DecimalError::TooManyFractionalDigits {
                provided: frac_part.len(),
                allowed: decimals,
            });
        }

        let parse_i128 = |input: &str| {
            input.parse::<i128>().map_err(|err| match err.kind() {
                IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => DecimalError::Overflow,
                _ => DecimalError::Invalid,
            })
        };

        let int_val = if int_part.is_empty() { 0 } else { parse_i128(int_part)? };
        let frac_val = if frac_part.is_empty() { 0 } else { parse_i128(frac_part)? };

        let int_scale = 10_i128.pow(SCALE);
        let frac_scale = 10_i128.pow(SCALE - (frac_part.len() as u32));

        let scaled_int_part = int_val
            .checked_mul(int_scale)
            .ok_or(DecimalError::Overflow)?;

        let scaled_frac_part = frac_val
            .checked_mul(frac_scale)
            .ok_or(DecimalError::Overflow)?;

        let minor = scaled_int_part
            .checked_add(scaled_frac_part)
            .ok_or(DecimalError::Overflow)?;

        let signed = if negative {
            minor.checked_neg().ok_or(DecimalError::Overflow)?
        } else {
            minor
        };
        Self::from_i128(signed)
    }
}
