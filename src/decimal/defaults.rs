use crate::decimal::{Decimal, DecimalError, RoundingMode};

impl<const SCALE: u32> Decimal<SCALE> {
    pub fn zero() -> Self {
        Self { minor_units: 0 }
    }

    pub fn one() -> Self {
        Self {
            minor_units: 10_i64.pow(SCALE),
        }
    }
}