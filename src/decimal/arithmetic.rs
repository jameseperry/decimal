use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::decimal::{Decimal, DecimalError, RoundingMode};

impl<const SCALE: u32> Add for Decimal<SCALE> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            minor_units: self.minor_units + rhs.minor_units,
        }
    }
}

impl<const SCALE: u32> Decimal<SCALE> {
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

    pub fn mul_rescale<const RHS: u32, const OUT: u32>(
        self,
        rhs: Decimal<RHS>,
        mode: RoundingMode,
    ) -> Result<Decimal<OUT>, DecimalError> {
        let product = (self.minor_units as i128)
            .checked_mul(rhs.minor_units as i128)
            .ok_or(DecimalError::Overflow)?;
        let in_scale = SCALE + RHS;

        if OUT == in_scale {
            let minor_units = i64::try_from(product).map_err(|_| DecimalError::Overflow)?;
            return Ok(Decimal { minor_units });
        }

        if OUT > in_scale {
            let factor = 10_i128.pow(OUT - in_scale);
            let scaled = product
                .checked_mul(factor)
                .ok_or(DecimalError::Overflow)?;
            let minor_units = i64::try_from(scaled).map_err(|_| DecimalError::Overflow)?;
            return Ok(Decimal { minor_units });
        }

        let factor = 10_i128.pow(in_scale - OUT);
        let base = product / factor;
        let rem = product % factor;
        if rem == 0 {
            let minor_units = i64::try_from(base).map_err(|_| DecimalError::Overflow)?;
            return Ok(Decimal { minor_units });
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

        let rounded = if should_round {
            if product.is_negative() {
                base.checked_sub(1).ok_or(DecimalError::Overflow)?
            } else {
                base.checked_add(1).ok_or(DecimalError::Overflow)?
            }
        } else {
            base
        };

        let minor_units = i64::try_from(rounded).map_err(|_| DecimalError::Overflow)?;
        Ok(Decimal { minor_units })
    }

    pub fn mul<const RATE: u32>(
        self,
        rate: Decimal<RATE>,
        mode: RoundingMode,
    ) -> Result<Self, DecimalError> {
        self.mul_rescale::<RATE, SCALE>(rate, mode)
    }

    pub fn div_rescale<const RHS: u32, const OUT: u32>(
        self,
        rhs: Decimal<RHS>,
        mode: RoundingMode,
    ) -> Result<Decimal<OUT>, DecimalError> {
        if rhs.minor_units == 0 {
            return Err(DecimalError::DivisionByZero);
        }

        let numer_factor = 10_i128.pow(RHS + OUT);
        let denom_factor = 10_i128.pow(SCALE);
        let numerator = (self.minor_units as i128)
            .checked_mul(numer_factor)
            .ok_or(DecimalError::Overflow)?;
        let denominator = (rhs.minor_units as i128)
            .checked_mul(denom_factor)
            .ok_or(DecimalError::Overflow)?;

        let base = numerator / denominator;
        let rem = numerator % denominator;
        if rem == 0 {
            let minor_units = i64::try_from(base).map_err(|_| DecimalError::Overflow)?;
            return Ok(Decimal { minor_units });
        }

        let abs_rem = rem.abs();
        let abs_den = denominator.abs();
        let should_round = match mode {
            RoundingMode::Truncate => false,
            RoundingMode::HalfUp => abs_rem * 2 >= abs_den,
            RoundingMode::HalfEven => {
                let twice = abs_rem * 2;
                if twice > abs_den {
                    true
                } else if twice < abs_den {
                    false
                } else {
                    base % 2 != 0
                }
            }
        };

        let rounded = if should_round {
            if (numerator < 0) ^ (denominator < 0) {
                base.checked_sub(1).ok_or(DecimalError::Overflow)?
            } else {
                base.checked_add(1).ok_or(DecimalError::Overflow)?
            }
        } else {
            base
        };

        let minor_units = i64::try_from(rounded).map_err(|_| DecimalError::Overflow)?;
        Ok(Decimal { minor_units })
    }

    pub fn div<const RATE: u32>(
        self,
        rate: Decimal<RATE>,
        mode: RoundingMode,
    ) -> Result<Self, DecimalError> {
        self.div_rescale::<RATE, SCALE>(rate, mode)
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
