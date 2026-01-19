//! Fixed-scale decimal arithmetic with configurable backing integer types.
//!
//! The `Decimal<T, SCALE>` type stores an integer `T` representing
//! `value / 10^SCALE`.
//!
//! # Examples
//! ```
//! use decimal::{Decimal, RoundingMode};
//!
//! let amount = "10.00".parse::<Decimal<i64, 2>>().unwrap();
//! let rate = "0.0125".parse::<Decimal<i64, 4>>().unwrap();
//! let rounded = amount.mul::<4>(rate, RoundingMode::HalfUp).unwrap();
//! assert_eq!(rounded.to_string(), "0.13");
//! ```

mod decimal;

pub use crate::decimal::{Decimal, DecimalError, DecimalInt, RoundingMode};
