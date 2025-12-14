# `qtty`

[![Crates.io](https://img.shields.io/crates/v/qtty.svg)](https://crates.io/crates/qtty)
[![Docs.rs](https://docs.rs/qtty/badge.svg)](https://docs.rs/qtty)

The user-facing crate that re-exports the `qtty-core` type system plus a curated set of astronomy-friendly units.

## Highlights

- Strongly typed quantities backed by `Quantity<U>` to prevent mixing incompatible dimensions.
- Conversion is explicit and type-checked via `to::<TargetUnit>()` with zero runtime overhead.
- Includes handy derived aliases (`velocity`, `frequency`, …) and astronomy staples (AU, light-year, solar mass/luminosity).
- Works in `no_std` environments (uses `libm`) and has optional `serde` support.

## Included units

Every unit module from `qtty-core` is re-exported at the crate root for convenience:

- Angular (`Degrees`, `Radian`, arcseconds, wrapping helpers)
- Time (`Seconds`, `Minutes`, `Days`, sidereal/astronomical variations)
- Length (`Meters`, `Kilometers`, `AstronomicalUnit`, `LightYear`, …)
- Mass (`Kilograms`, `SolarMass`)
- Power (`Watts`, `SolarLuminosity`)
- Velocity and frequency (`Per<Length, Time>`, `Per<Angular, Time>` aliases)
- `unitless` helpers for scalar quantities

## Install

```toml
[dependencies]
qtty = "0.1.0"
```

Disable default features for `no_std`:

```toml
[dependencies]
qtty = { version = "0.1.0", default-features = false }
```

## Quick start

```rust
use qtty::{Degrees, Radian};

let a = Degrees::new(90.0);
let r = a.to::<Radian>();
assert!((r.value() - core::f64::consts::FRAC_PI_2).abs() < 1e-12);
```

```rust
use qtty::{Kilometer, Kilometers, Second, Seconds};
use qtty::velocity::Velocity;

let d = Kilometers::new(1_000.0);
let t = Seconds::new(100.0);
let v: Velocity<Kilometer, Second> = d / t;
assert!((v.value() - 10.0).abs() < 1e-12);
```

## Feature flags

- `std` (default): enables `std` support in `qtty-core`.
- `serde`: serializes/deserializes `Quantity<U>` as bare `f64` values (unit is encoded by the type).

## Related crates

- `qtty-core`: the minimal zero-cost type system used underneath this facade.
- `qtty-ffi`: exposes the same quantities and conversions over a stable C ABI.

## License

AGPL-3.0 (see `../LICENSE`).
