# `qtty`

User-facing crate providing strongly typed units and conversions.

This crate re-exports:

- the core type system from `qtty-core` (`Quantity`, `Unit`, `Per`, …)
- predefined units grouped by module (`angular`, `time`, `length`, `mass`, `power`, `velocity`, `frequency`, `unitless`)

## Install

```toml
[dependencies]
qtty = "0.1.0"
```

## Example

```rust
use qtty::{Degrees, Radian};

let a = Degrees::new(90.0);
let r = a.to::<Radian>();
assert!((r.value() - core::f64::consts::FRAC_PI_2).abs() < 1e-12);
```

## Features

- `std` (default): enables `std` support in `qtty-core`.
- `serde`: enables `serde` support for `Quantity<U>` (serialize/deserialize as a bare `f64`).

Disable default features for `no_std`:

```toml
[dependencies]
qtty = { version = "0.1.0", default-features = false }
```

## License

AGPL-3.0 (see `../LICENSE`).
