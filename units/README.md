# `unit`

User-facing crate providing strongly typed units and conversions.

This crate re-exports:

- the core type system from `unit-core` (`Quantity`, `Unit`, `Per`, …)
- predefined units grouped by module (`angular`, `time`, `length`, `mass`, `power`, `velocity`, `frequency`, `unitless`)

## Install

```toml
[dependencies]
unit = "0.1.0"
```

## Example

```rust
use unit::{Degrees, Radian};

let a = Degrees::new(90.0);
let r = a.to::<Radian>();
assert!((r.value() - core::f64::consts::FRAC_PI_2).abs() < 1e-12);
```

## Features

- `std` (default): enables `std` support in `unit-core`.
- `serde`: enables `serde` support for `Quantity<U>`.

## License

AGPL-3.0 (see `../LICENSE`).
