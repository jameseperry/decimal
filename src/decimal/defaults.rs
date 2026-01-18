use crate::decimal::{Decimal, DecimalInt};

impl<T: DecimalInt, const SCALE: u32> Decimal<T, SCALE> {
    pub fn zero() -> Self {
        Self {
            minor_units: T::try_from_i128(0).unwrap_or_else(|| unreachable!()),
        }
    }

    pub fn one() -> Self {
        Self::from_i128(10_i128.pow(SCALE)).unwrap_or_else(|_| {
            unreachable!("Decimal one() overflowed for SCALE {}", SCALE)
        })
    }
}
