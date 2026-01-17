use decimal::Decimal;

#[test]
fn display_fixed_scale() {
    let cases = [
        ("0", "0.00"),
        ("-0", "0.00"),
        ("1.20", "1.20"),
        ("0.05", "0.05"),
        ("-0.05", "-0.05"),
        ("1234.", "1234.00"),
    ];

    for (input, expected) in cases {
        let value = input.parse::<Decimal<2>>().unwrap();
        assert_eq!(value.to_string(), expected);
    }
}

#[test]
fn display_different_scales() {
    let value = "123".parse::<Decimal<0>>().unwrap();
    assert_eq!(value.to_string(), "123");

    let value = "-.25".parse::<Decimal<3>>().unwrap();
    assert_eq!(value.to_string(), "-0.250");
}
