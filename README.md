# decimal

Fixed-scale decimal type with parsing, display, conversions, and rescaling.

## Usage

```rust
use decimal::{Decimal, RoundingMode};

let amount = "10.00".parse::<Decimal<i64, 2>>().unwrap();
let rate = "0.0125".parse::<Decimal<i64, 4>>().unwrap();

let exact = amount.mul_rescale::<4, 6>(rate, RoundingMode::Truncate).unwrap();
assert_eq!(exact.to_string(), "0.125000");

let rounded = amount.mul::<4>(rate, RoundingMode::HalfUp).unwrap();
assert_eq!(rounded.to_string(), "0.13");
```

## License

MIT. See `LICENSE`.
