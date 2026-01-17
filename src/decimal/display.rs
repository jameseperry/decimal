use std::fmt;

use crate::decimal::Decimal;

impl<const SCALE: u32> fmt::Display for Decimal<SCALE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if SCALE == 0 {
            return write!(f, "{}", self.minor_units);
        }

        let decimals = SCALE as usize;
        let scale = 10_i128.pow(decimals as u32);

        let minor = self.minor_units as i128;
        let negative = minor < 0;
        let abs = if negative { -minor } else { minor };
        let int_part = abs / scale;
        let frac_part = abs % scale;

        if negative {
            write!(f, "-{}.{:0width$}", int_part, frac_part, width = decimals)
        } else {
            write!(f, "{}.{:0width$}", int_part, frac_part, width = decimals)
        }
    }
}
