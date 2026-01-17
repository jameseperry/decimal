mod conversion;
mod error;
mod arithmetic;
mod display;
mod parsing;

#[allow(dead_code)]
const MAX_SCALE: u32 = 18;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Decimal<const SCALE: u32> {
    minor_units: i64,
}

impl<const SCALE: u32> Decimal<SCALE> {
    const _ASSERT_SCALE: () = assert!(SCALE <= MAX_SCALE);

    pub(crate) fn checked_from_i64(value: i64) -> Result<Self, DecimalError> {
        let scale = 10_i64.pow(SCALE);
        let minor_units = value
            .checked_mul(scale)
            .ok_or(DecimalError::Overflow)?;
        Ok(Self { minor_units })
    }
}

pub use self::error::DecimalError;
pub use self::conversion::RoundingMode;
