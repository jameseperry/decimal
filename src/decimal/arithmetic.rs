use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::decimal::{Decimal, DecimalError, DecimalInt, RoundingMode};

impl<T: DecimalInt, const SCALE: u32> Add for Decimal<T, SCALE> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            minor_units: self.minor_units + rhs.minor_units,
        }
    }
}

impl<T: DecimalInt, const SCALE: u32> Decimal<T, SCALE> {
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

    pub fn is_zero(&self) -> bool {
        self.minor_units == T::try_from_i128(0).unwrap_or_else(|| unreachable!())
    }

    pub fn mul_rescale<const RHS: u32, const OUT: u32>(
        self,
        rhs: Decimal<T, RHS>,
        mode: RoundingMode,
    ) -> Result<Decimal<T, OUT>, DecimalError> {
        let product = self
            .minor_units
            .to_i128()
            .checked_mul(rhs.minor_units.to_i128())
            .ok_or(DecimalError::Overflow)?;
        let in_scale = SCALE + RHS;

        if OUT == in_scale {
            return Decimal::<T, OUT>::from_i128(product);
        }

        if OUT > in_scale {
            let factor = 10_i128.pow(OUT - in_scale);
            let scaled = product
                .checked_mul(factor)
                .ok_or(DecimalError::Overflow)?;
            return Decimal::<T, OUT>::from_i128(scaled);
        }

        let factor = 10_i128.pow(in_scale - OUT);
        let base = product / factor;
        let rem = product % factor;
        if rem == 0 {
            return Decimal::<T, OUT>::from_i128(base);
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

        Decimal::<T, OUT>::from_i128(rounded)
    }

    pub fn mul<const RATE: u32>(
        self,
        rate: Decimal<T, RATE>,
        mode: RoundingMode,
    ) -> Result<Self, DecimalError> {
        self.mul_rescale::<RATE, SCALE>(rate, mode)
    }

    pub fn div_rescale<const RHS: u32, const OUT: u32>(
        self,
        rhs: Decimal<T, RHS>,
        mode: RoundingMode,
    ) -> Result<Decimal<T, OUT>, DecimalError> {
        if rhs.minor_units.to_i128() == 0 {
            return Err(DecimalError::DivisionByZero);
        }

        let numer_factor = 10_i128.pow(RHS + OUT);
        let denom_factor = 10_i128.pow(SCALE);
        let numerator = self
            .minor_units
            .to_i128()
            .checked_mul(numer_factor)
            .ok_or(DecimalError::Overflow)?;
        let denominator = rhs
            .minor_units
            .to_i128()
            .checked_mul(denom_factor)
            .ok_or(DecimalError::Overflow)?;

        let base = numerator / denominator;
        let rem = numerator % denominator;
        if rem == 0 {
            return Decimal::<T, OUT>::from_i128(base);
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

        Decimal::<T, OUT>::from_i128(rounded)
    }

    pub fn div<const RATE: u32>(
        self,
        rate: Decimal<T, RATE>,
        mode: RoundingMode,
    ) -> Result<Self, DecimalError> {
        self.div_rescale::<RATE, SCALE>(rate, mode)
    }
}

impl<T: DecimalInt, const SCALE: u32> Sub for Decimal<T, SCALE> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            minor_units: self.minor_units - rhs.minor_units,
        }
    }
}

impl<T: DecimalInt, const SCALE: u32> AddAssign for Decimal<T, SCALE> {
    fn add_assign(&mut self, rhs: Self) {
        self.minor_units += rhs.minor_units;
    }
}

impl<T: DecimalInt, const SCALE: u32> SubAssign for Decimal<T, SCALE> {
    fn sub_assign(&mut self, rhs: Self) {
        self.minor_units -= rhs.minor_units;
    }
}
