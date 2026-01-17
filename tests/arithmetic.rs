use decimal::Decimal;

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
