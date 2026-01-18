use decimal::{Decimal, DecimalError, RoundingMode};

#[test]
fn convert_from_signed() {
    let value = Decimal::<i64, 2>::try_from(12_i64).unwrap();
    assert_eq!(value.to_string(), "12.00");

    let value = Decimal::<i64, 3>::try_from(-7_i32).unwrap();
    assert_eq!(value.to_string(), "-7.000");
}

#[test]
fn convert_from_unsigned() {
    let value = Decimal::<i64, 1>::try_from(5_u8).unwrap();
    assert_eq!(value.to_string(), "5.0");
}

#[test]
fn convert_from_i128_backing() {
    let value = Decimal::<i128, 2>::try_from(12_i64).unwrap();
    assert_eq!(value.to_string(), "12.00");
}

#[test]
fn convert_between_backing_types() {
    let value_i64 = "123.45".parse::<Decimal<i64, 2>>().unwrap();
    let value_i128 = Decimal::<i128, 2>::from(value_i64);
    assert_eq!(value_i128.to_string(), "123.45");

    let back = Decimal::<i64, 2>::try_from(value_i128).unwrap();
    assert_eq!(back.to_string(), "123.45");
}

#[test]
fn convert_i128_to_i64_overflow() {
    let value_i128 = "92233720368547758.08"
        .parse::<Decimal<i128, 2>>()
        .unwrap();
    assert!(Decimal::<i64, 2>::try_from(value_i128).is_err());
}

#[test]
fn convert_overflow() {
    assert!(Decimal::<i64, 2>::try_from(i64::MAX).is_err());
    assert!(Decimal::<i64, 18>::try_from(10_i64).is_err());
    assert!(Decimal::<i64, 2>::try_from(u128::MAX).is_err());
}

#[test]
fn convert_scale_exact() {
    let value = "1.23".parse::<Decimal<i64, 2>>().unwrap();
    let up = value.try_rescale::<4>().unwrap();
    assert_eq!(up.to_string(), "1.2300");

    let down = up.try_rescale::<2>().unwrap();
    assert_eq!(down.to_string(), "1.23");
}

#[test]
fn convert_scale_inexact() {
    let value = "1.234".parse::<Decimal<i64, 3>>().unwrap();
    assert!(value.try_rescale::<2>().is_err());
}

#[test]
fn rescale_truncate() {
    let value = "1.239".parse::<Decimal<i64, 3>>().unwrap();
    let down = value.rescale::<2>(RoundingMode::Truncate).unwrap();
    assert_eq!(down.to_string(), "1.23");

    let value = "-1.239".parse::<Decimal<i64, 3>>().unwrap();
    let down = value.rescale::<2>(RoundingMode::Truncate).unwrap();
    assert_eq!(down.to_string(), "-1.23");
}

#[test]
fn rescale_half_up() {
    let value = "1.235".parse::<Decimal<i64, 3>>().unwrap();
    let down = value.rescale::<2>(RoundingMode::HalfUp).unwrap();
    assert_eq!(down.to_string(), "1.24");

    let value = "-1.235".parse::<Decimal<i64, 3>>().unwrap();
    let down = value.rescale::<2>(RoundingMode::HalfUp).unwrap();
    assert_eq!(down.to_string(), "-1.24");
}

#[test]
fn rescale_half_even() {
    let value = "1.245".parse::<Decimal<i64, 3>>().unwrap();
    let down = value.rescale::<2>(RoundingMode::HalfEven).unwrap();
    assert_eq!(down.to_string(), "1.24");

    let value = "1.255".parse::<Decimal<i64, 3>>().unwrap();
    let down = value.rescale::<2>(RoundingMode::HalfEven).unwrap();
    assert_eq!(down.to_string(), "1.26");

    let value = "-1.245".parse::<Decimal<i64, 3>>().unwrap();
    let down = value.rescale::<2>(RoundingMode::HalfEven).unwrap();
    assert_eq!(down.to_string(), "-1.24");
}

#[test]
fn convert_to_f64() {
    let value = "1.25".parse::<Decimal<i64, 2>>().unwrap();
    let as_f64 = value.to_f64();
    assert!((as_f64 - 1.25).abs() < 1e-12);
}

#[test]
fn convert_from_f64_rounding() {
    let value = Decimal::<i64, 2>::from_f64(1.125, RoundingMode::HalfUp).unwrap();
    assert_eq!(value.to_string(), "1.13");

    let value = Decimal::<i64, 2>::from_f64(1.125, RoundingMode::HalfEven).unwrap();
    assert_eq!(value.to_string(), "1.12");
}

#[test]
fn convert_from_f64_invalid() {
    assert_eq!(
        Decimal::<i64, 2>::from_f64(f64::NAN, RoundingMode::Truncate).unwrap_err(),
        DecimalError::Invalid
    );
    assert_eq!(
        Decimal::<i64, 2>::from_f64(f64::INFINITY, RoundingMode::Truncate).unwrap_err(),
        DecimalError::Invalid
    );
    assert_eq!(
        Decimal::<i64, 2>::from_f64(f64::NEG_INFINITY, RoundingMode::Truncate).unwrap_err(),
        DecimalError::Invalid
    );
}

#[test]
fn convert_from_f64_truncate_and_negative() {
    let value = Decimal::<i64, 3>::from_f64(2.9999, RoundingMode::Truncate).unwrap();
    assert_eq!(value.to_string(), "2.999");

    let value = Decimal::<i64, 2>::from_f64(-1.235, RoundingMode::HalfUp).unwrap();
    assert_eq!(value.to_string(), "-1.24");
}

#[test]
fn convert_from_f64_overflow() {
    let err = Decimal::<i64, 2>::from_f64(1e30, RoundingMode::HalfUp).unwrap_err();
    assert_eq!(err, DecimalError::Overflow);
}
