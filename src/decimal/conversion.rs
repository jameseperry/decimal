use crate::decimal::{Decimal, DecimalError, DecimalInt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundingMode {
    Truncate,
    HalfUp,
    HalfEven,
}

impl<T: DecimalInt, const SCALE: u32> Decimal<T, SCALE> {
    pub fn to_f64(self) -> f64 {
        let scale = 10_f64.powi(SCALE as i32);
        (self.minor_units.to_i128() as f64) / scale
    }

    pub fn from_f64(value: f64, mode: RoundingMode) -> Result<Self, DecimalError> {
        if !value.is_finite() {
            return Err(DecimalError::Invalid);
        }

        let scale = 10_f64.powi(SCALE as i32);
        let scaled = value * scale;
        let abs = scaled.abs();

        let rounded_abs = match mode {
            RoundingMode::Truncate => abs.trunc(),
            RoundingMode::HalfUp => abs.round(),
            RoundingMode::HalfEven => {
                let floor = abs.floor();
                let frac = abs - floor;
                let tie = (frac - 0.5).abs() <= 1e-12;
                if tie {
                    if (floor as i128) % 2 == 0 {
                        floor
                    } else {
                        floor + 1.0
                    }
                } else if frac < 0.5 {
                    floor
                } else {
                    floor + 1.0
                }
            }
        };

        if rounded_abs > i128::MAX as f64 {
            return Err(DecimalError::Overflow);
        }

        let signed = if scaled.is_sign_negative() {
            -(rounded_abs as i128)
        } else {
            rounded_abs as i128
        };
        Self::from_i128(signed)
    }
}

impl<T: DecimalInt, const FROM: u32> Decimal<T, FROM> {
    pub fn try_rescale<const TO: u32>(self) -> Result<Decimal<T, TO>, DecimalError> {
        if FROM == TO {
            return Ok(Decimal {
                minor_units: self.minor_units,
            });
        }

        if TO > FROM {
            let factor = 10_i128.pow(TO - FROM);
            let minor_units = self
                .minor_units
                .to_i128()
                .checked_mul(factor)
                .ok_or(DecimalError::Overflow)?;
            return Decimal::<T, TO>::from_i128(minor_units);
        }

        let factor = 10_i128.pow(FROM - TO);
        let minor_units = self.minor_units.to_i128();
        if minor_units % factor != 0 {
            return Err(DecimalError::Invalid);
        }
        Decimal::<T, TO>::from_i128(minor_units / factor)
    }

    pub fn rescale<const TO: u32>(
        self,
        mode: RoundingMode,
    ) -> Result<Decimal<T, TO>, DecimalError> {
        if FROM == TO {
            return Ok(Decimal {
                minor_units: self.minor_units,
            });
        }

        if TO > FROM {
            let factor = 10_i128.pow(TO - FROM);
            let minor_units = self
                .minor_units
                .to_i128()
                .checked_mul(factor)
                .ok_or(DecimalError::Overflow)?;
            return Decimal::<T, TO>::from_i128(minor_units);
        }

        let factor = 10_i128.pow(FROM - TO);
        let minor_units = self.minor_units.to_i128();
        let base = minor_units / factor;
        let rem = minor_units % factor;
        if rem == 0 {
            return Decimal::<T, TO>::from_i128(base);
        }

        let abs_rem = rem.abs();
        let abs_factor = factor.abs();
        let should_round = match mode {
            RoundingMode::Truncate => false,
            RoundingMode::HalfUp => abs_rem * 2 >= abs_factor,
            RoundingMode::HalfEven => {
                let twice = abs_rem * 2;
                if twice > abs_factor {
                    true
                } else if twice < abs_factor {
                    false
                } else {
                    base % 2 != 0
                }
            }
        };

        if !should_round {
            return Decimal::<T, TO>::from_i128(base);
        }

        let adjusted = if minor_units.is_negative() {
            base.checked_sub(1)
        } else {
            base.checked_add(1)
        }
        .ok_or(DecimalError::Overflow)?;

        Decimal::<T, TO>::from_i128(adjusted)
    }
}

macro_rules! impl_try_from_signed {
    ($($t:ty),+ $(,)?) => {
        $(
            impl<T: DecimalInt, const SCALE: u32> TryFrom<$t> for Decimal<T, SCALE> {
                type Error = DecimalError;

                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    let value_i128: i128 =
                        value.try_into().map_err(|_| DecimalError::Overflow)?;
                    Self::checked_from_i128(value_i128)
                }
            }
        )+
    };
}

macro_rules! impl_try_from_unsigned {
    ($($t:ty),+ $(,)?) => {
        $(
            impl<T: DecimalInt, const SCALE: u32> TryFrom<$t> for Decimal<T, SCALE> {
                type Error = DecimalError;

                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    let value_i128: i128 =
                        value.try_into().map_err(|_| DecimalError::Overflow)?;
                    Self::checked_from_i128(value_i128)
                }
            }
        )+
    };
}

impl_try_from_signed!(i8, i16, i32, i64, i128, isize);
impl_try_from_unsigned!(u8, u16, u32, u64, u128, usize);

impl<const SCALE: u32> TryFrom<Decimal<i64, SCALE>> for Decimal<i128, SCALE> {
    type Error = DecimalError;

    fn try_from(value: Decimal<i64, SCALE>) -> Result<Self, Self::Error> {
        Decimal::<i128, SCALE>::from_i128(value.minor_units as i128)
    }
}

impl<const SCALE: u32> TryFrom<Decimal<i128, SCALE>> for Decimal<i64, SCALE> {
    type Error = DecimalError;

    fn try_from(value: Decimal<i128, SCALE>) -> Result<Self, Self::Error> {
        Decimal::<i64, SCALE>::from_i128(value.minor_units)
    }
}
