use crate::decimal::{Decimal, DecimalError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundingMode {
    Truncate,
    HalfUp,
    HalfEven,
}

impl<const SCALE: u32> Decimal<SCALE> {
    pub fn to_f64(self) -> f64 {
        let scale = 10_f64.powi(SCALE as i32);
        (self.minor_units as f64) / scale
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

        if rounded_abs > i64::MAX as f64 {
            return Err(DecimalError::Overflow);
        }

        let signed = if scaled.is_sign_negative() {
            -(rounded_abs as i64)
        } else {
            rounded_abs as i64
        };
        Ok(Self {
            minor_units: signed,
        })
    }
}

impl<const FROM: u32> Decimal<FROM> {
    pub fn try_rescale<const TO: u32>(self) -> Result<Decimal<TO>, DecimalError> {
        if FROM == TO {
            return Ok(Decimal {
                minor_units: self.minor_units,
            });
        }

        if TO > FROM {
            let factor = 10_i64.pow(TO - FROM);
            let minor_units = self
                .minor_units
                .checked_mul(factor)
                .ok_or(DecimalError::Overflow)?;
            return Ok(Decimal { minor_units });
        }

        let factor = 10_i64.pow(FROM - TO);
        if self.minor_units % factor != 0 {
            return Err(DecimalError::Invalid);
        }
        Ok(Decimal {
            minor_units: self.minor_units / factor,
        })
    }

    pub fn rescale<const TO: u32>(self, mode: RoundingMode) -> Result<Decimal<TO>, DecimalError> {
        if FROM == TO {
            return Ok(Decimal {
                minor_units: self.minor_units,
            });
        }

        if TO > FROM {
            let factor = 10_i64.pow(TO - FROM);
            let minor_units = self
                .minor_units
                .checked_mul(factor)
                .ok_or(DecimalError::Overflow)?;
            return Ok(Decimal { minor_units });
        }

        let factor = 10_i64.pow(FROM - TO);
        let base = self.minor_units / factor;
        let rem = self.minor_units % factor;
        if rem == 0 {
            return Ok(Decimal {
                minor_units: base,
            });
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
            return Ok(Decimal {
                minor_units: base,
            });
        }

        let adjusted = if self.minor_units.is_negative() {
            base.checked_sub(1)
        } else {
            base.checked_add(1)
        }
        .ok_or(DecimalError::Overflow)?;

        Ok(Decimal {
            minor_units: adjusted,
        })
    }
}

macro_rules! impl_try_from_signed {
    ($($t:ty),+ $(,)?) => {
        $(
            impl<const SCALE: u32> TryFrom<$t> for Decimal<SCALE> {
                type Error = DecimalError;

                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    let value_i64: i64 = value.try_into().map_err(|_| DecimalError::Overflow)?;
                    Self::checked_from_i64(value_i64)
                }
            }
        )+
    };
}

macro_rules! impl_try_from_unsigned {
    ($($t:ty),+ $(,)?) => {
        $(
            impl<const SCALE: u32> TryFrom<$t> for Decimal<SCALE> {
                type Error = DecimalError;

                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    let value_i64: i64 = value.try_into().map_err(|_| DecimalError::Overflow)?;
                    Self::checked_from_i64(value_i64)
                }
            }
        )+
    };
}

impl_try_from_signed!(i8, i16, i32, i64, i128, isize);
impl_try_from_unsigned!(u8, u16, u32, u64, u128, usize);
