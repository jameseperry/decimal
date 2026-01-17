use decimal::Decimal;

#[test]
fn parse_valid_formats() {
    let cases = [
        ("0", "0.00"),
        ("-0", "0.00"),
        ("1.23", "1.23"),
        ("1234.", "1234.00"),
        (".5", "0.50"),
        ("-.25", "-0.25"),
        ("+7.1", "7.10"),
    ];

    for (input, expected) in cases {
        let value = input.parse::<Decimal<2>>().unwrap();
        assert_eq!(value.to_string(), expected);
    }
}

#[test]
fn parse_rejects_invalid_formats() {
    let cases = [
        "",
        "+",
        "-",
        "1.2.3",
        "1_000",
        "1.-2",
        "+-1",
        "--1",
        " 1.0",
        "1.0 ",
    ];

    for input in cases {
        assert!(input.parse::<Decimal<2>>().is_err(), "input {input} should fail");
    }
}

#[test]
fn parse_rejects_too_many_fractional_digits() {
    assert!("1.234".parse::<Decimal<2>>().is_err());
    assert!(".001".parse::<Decimal<2>>().is_err());
}

#[test]
fn parse_rejects_overflow() {
    assert!("92233720368547758.08".parse::<Decimal<2>>().is_err());
    assert!("-92233720368547758.08".parse::<Decimal<2>>().is_err());
}
