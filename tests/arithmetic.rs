use decimal::{Decimal, DecimalError, RoundingMode};

#[test]
fn arithmetic_smoke() {
    let a = "1.00".parse::<Decimal<2>>().unwrap();
    let b = "2.00".parse::<Decimal<2>>().unwrap();
    let _ = a + b;
}

#[test]
fn arithmetic_negative_values() {
    let a = "-1.25".parse::<Decimal<2>>().unwrap();
    let b = "0.75".parse::<Decimal<2>>().unwrap();
    assert_eq!((a + b).to_string(), "-0.50");
    assert_eq!((b - a).to_string(), "2.00");
}

#[test]
fn checked_ops_overflow() {
    let max = "92233720368547758.07".parse::<Decimal<2>>().unwrap();
    let one = "0.01".parse::<Decimal<2>>().unwrap();
    assert!(max.checked_add(one).is_none());
    let min_plus = "-92233720368547758.07".parse::<Decimal<2>>().unwrap();
    let two = "0.02".parse::<Decimal<2>>().unwrap();
    assert!(min_plus.checked_sub(two).is_none());
}

#[test]
#[should_panic]
fn arithmetic_overflow_panics() {
    let max = "92233720368547758.07".parse::<Decimal<2>>().unwrap();
    let one = "0.01".parse::<Decimal<2>>().unwrap();
    let _ = max + one;
}

#[test]
fn multiply_exact_scale() {
    let amount = "10.00".parse::<Decimal<2>>().unwrap();
    let rate = "0.0125".parse::<Decimal<4>>().unwrap();
    let value = amount
        .mul_rescale::<4, 6>(rate, RoundingMode::Truncate)
        .unwrap();
    assert_eq!(value.to_string(), "0.125000");
}

#[test]
fn multiply_rounding_modes() {
    let amount = "10.00".parse::<Decimal<2>>().unwrap();
    let rate = "0.0125".parse::<Decimal<4>>().unwrap();

    let truncate = amount
        .mul_rescale::<4, 2>(rate, RoundingMode::Truncate)
        .unwrap();
    assert_eq!(truncate.to_string(), "0.12");

    let half_up = amount
        .mul_rescale::<4, 2>(rate, RoundingMode::HalfUp)
        .unwrap();
    assert_eq!(half_up.to_string(), "0.13");

    let half_even = amount
        .mul_rescale::<4, 2>(rate, RoundingMode::HalfEven)
        .unwrap();
    assert_eq!(half_even.to_string(), "0.12");
}

#[test]
fn multiply_rate_helper() {
    let amount = "10.00".parse::<Decimal<2>>().unwrap();
    let rate = "0.0125".parse::<Decimal<4>>().unwrap();
    let value = amount.mul::<4>(rate, RoundingMode::HalfUp).unwrap();
    assert_eq!(value.to_string(), "0.13");
}

#[test]
fn divide_exact_scale() {
    let amount = "10.00".parse::<Decimal<2>>().unwrap();
    let rate = "4.0000".parse::<Decimal<4>>().unwrap();
    let value = amount
        .div_rescale::<4, 6>(rate, RoundingMode::Truncate)
        .unwrap();
    assert_eq!(value.to_string(), "2.500000");
}

#[test]
fn divide_rounding_modes() {
    let amount = "1.00".parse::<Decimal<2>>().unwrap();
    let rate = "8.0000".parse::<Decimal<4>>().unwrap();

    let truncate = amount
        .div_rescale::<4, 2>(rate, RoundingMode::Truncate)
        .unwrap();
    assert_eq!(truncate.to_string(), "0.12");

    let half_up = amount
        .div_rescale::<4, 2>(rate, RoundingMode::HalfUp)
        .unwrap();
    assert_eq!(half_up.to_string(), "0.13");

    let half_even = amount
        .div_rescale::<4, 2>(rate, RoundingMode::HalfEven)
        .unwrap();
    assert_eq!(half_even.to_string(), "0.12");
}

#[test]
fn divide_by_zero() {
    let amount = "10.00".parse::<Decimal<2>>().unwrap();
    let zero = "0.0000".parse::<Decimal<4>>().unwrap();
    let err = amount
        .div_rescale::<4, 2>(zero, RoundingMode::Truncate)
        .unwrap_err();
    assert_eq!(err, DecimalError::DivisionByZero);
}

#[test]
fn divide_rate_helper() {
    let amount = "10.00".parse::<Decimal<2>>().unwrap();
    let rate = "4.0000".parse::<Decimal<4>>().unwrap();
    let value = amount.div::<4>(rate, RoundingMode::HalfUp).unwrap();
    assert_eq!(value.to_string(), "2.50");
}

#[test]
fn divide_exact_integer_result() {
    let amount = "12.00".parse::<Decimal<2>>().unwrap();
    let rate = "3.0000".parse::<Decimal<4>>().unwrap();
    let value = amount
        .div_rescale::<4, 2>(rate, RoundingMode::HalfEven)
        .unwrap();
    assert_eq!(value.to_string(), "4.00");
}

#[test]
fn divide_rounding_negative() {
    let amount = "-1.00".parse::<Decimal<2>>().unwrap();
    let rate = "8.0000".parse::<Decimal<4>>().unwrap();
    let half_up = amount
        .div_rescale::<4, 2>(rate, RoundingMode::HalfUp)
        .unwrap();
    assert_eq!(half_up.to_string(), "-0.13");

    let half_even = amount
        .div_rescale::<4, 2>(rate, RoundingMode::HalfEven)
        .unwrap();
    assert_eq!(half_even.to_string(), "-0.12");
}

#[test]
fn divide_negative_rounding_modes() {
    let amount = "-1.00".parse::<Decimal<2>>().unwrap();
    let rate = "8.0000".parse::<Decimal<4>>().unwrap();

    let truncate = amount
        .div_rescale::<4, 2>(rate, RoundingMode::Truncate)
        .unwrap();
    assert_eq!(truncate.to_string(), "-0.12");

    let half_up = amount
        .div_rescale::<4, 2>(rate, RoundingMode::HalfUp)
        .unwrap();
    assert_eq!(half_up.to_string(), "-0.13");

    let half_even = amount
        .div_rescale::<4, 2>(rate, RoundingMode::HalfEven)
        .unwrap();
    assert_eq!(half_even.to_string(), "-0.12");
}

#[test]
fn divide_scale_up() {
    let amount = "1.00".parse::<Decimal<2>>().unwrap();
    let rate = "8.0000".parse::<Decimal<4>>().unwrap();
    let value = amount
        .div_rescale::<4, 6>(rate, RoundingMode::HalfUp)
        .unwrap();
    assert_eq!(value.to_string(), "0.125000");
}

#[test]
fn divide_half_even_tie() {
    let amount = "1.00".parse::<Decimal<2>>().unwrap();
    let rate = "8.0000".parse::<Decimal<4>>().unwrap();
    let value = amount
        .div_rescale::<4, 2>(rate, RoundingMode::HalfEven)
        .unwrap();
    assert_eq!(value.to_string(), "0.12");
}
