use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::decimal::Decimal;

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
