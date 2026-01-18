mod conversion;
mod error;
mod arithmetic;
mod display;
mod parsing;
mod defaults;

#[allow(dead_code)]
const MAX_SCALE: u32 = 18;

pub trait DecimalInt:
    Copy
    + Eq
    + Ord
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::AddAssign
    + std::ops::SubAssign
{
    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn to_i128(self) -> i128;
    fn try_from_i128(value: i128) -> Option<Self>;
}

impl DecimalInt for i64 {
    fn checked_add(self, rhs: Self) -> Option<Self> {
        i64::checked_add(self, rhs)
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        i64::checked_sub(self, rhs)
    }

    fn to_i128(self) -> i128 {
        i128::from(self)
    }

    fn try_from_i128(value: i128) -> Option<Self> {
        i64::try_from(value).ok()
    }
}

impl DecimalInt for i128 {
    fn checked_add(self, rhs: Self) -> Option<Self> {
        i128::checked_add(self, rhs)
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        i128::checked_sub(self, rhs)
    }

    fn to_i128(self) -> i128 {
        self
    }

    fn try_from_i128(value: i128) -> Option<Self> {
        Some(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Decimal<T, const SCALE: u32> {
    minor_units: T,
}

impl<T: DecimalInt, const SCALE: u32> Decimal<T, SCALE> {
    const _ASSERT_SCALE: () = assert!(SCALE <= MAX_SCALE);

    pub(crate) fn checked_from_i128(value: i128) -> Result<Self, DecimalError> {
        let scale = 10_i128.pow(SCALE);
        let minor_units = value
            .checked_mul(scale)
            .ok_or(DecimalError::Overflow)?;
        Self::from_i128(minor_units)
    }

    pub(crate) fn from_i128(value: i128) -> Result<Self, DecimalError> {
        let minor_units = T::try_from_i128(value).ok_or(DecimalError::Overflow)?;
        Ok(Self { minor_units })
    }
}

pub use self::error::DecimalError;
pub use self::conversion::RoundingMode;
