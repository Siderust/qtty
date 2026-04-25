# qtty

[![Crates.io](https://img.shields.io/crates/v/qtty.svg)](https://crates.io/crates/qtty)
[![Docs.rs](https://docs.rs/qtty/badge.svg)](https://docs.rs/qtty)

The user-facing crate for strongly typed physical quantities, conversions, and astronomy-friendly units.

`qtty` re-exports the `qtty-core` type system plus a curated set of built-in units so you can write dimensionally safe
code without giving up ergonomics.

## Highlights

- Zero-cost `Quantity<U, S>` model with compile-time unit safety
- Explicit conversions via `.to::<TargetUnit>()`
- Dimensional arithmetic for area, volume, velocity, angular rate, and other derived quantities
- Built-in astronomy-oriented units such as `AstronomicalUnit`, `LightYear`, `Parsec`, `SolarMass`, and `SolarLuminosity`
- `no_std` support, optional `serde`, optional PyO3/SQL integrations, and integer scalar families

## Install

```toml
[dependencies]
qtty = "0.6.0"
```

Disable default features for `no_std`:

```toml
[dependencies]
qtty = { version = "0.6.0", default-features = false }
```

Enable heap-backed helpers in `no_std`:

```toml
[dependencies]
qtty = { version = "0.6.0", default-features = false, features = ["alloc"] }
```

## Quick start

```rust
use qtty::{Degree, Radian};

let angle = Degree::new(90.0);
let radians = angle.to::<qtty::unit::Radian>();
assert!((radians.value() - core::f64::consts::FRAC_PI_2).abs() < 1e-12);
```

```rust
use qtty::{Kilometer, Second};
use qtty::velocity::Velocity;

let distance = Kilometer::new(1_000.0);
let elapsed = Second::new(100.0);
let speed: Velocity<qtty::unit::Kilometer, qtty::unit::Second> = distance / elapsed;
assert!((speed.value() - 10.0).abs() < 1e-12);
```

## Feature flags

- `std` (default): enables `std` support in `qtty-core`
- `cross-unit-ops` (default): enables direct cross-unit comparison operators for compatible built-in units
- `alloc`: enables heap-backed helpers such as `qtty::qtty_vec!(vec ...)` in `no_std`
- `serde`: enables serialization helpers for quantities
- `scalar-rational`: enables `num_rational::Rational64`
- `pyo3`: enables PyO3 conversions and Python-facing helpers
- `tiberius`: enables SQL Server integration helpers
- `diesel`: enables Diesel integration helpers

## Modules

- `angular`, `time`, `length`, `mass`, `power`
- `area`, `volume`, `force`, `energy`
- `velocity`, `angular_rate`, `accel`, `unit`
- scalar-specific facades: `f32`, `f64`, `i8`, `i16`, `i32`, `i64`, `i128`

Same-unit division yields a dimensionless quantity, but there is no dedicated
`unitless` module in the facade crate.

## Custom units

`qtty` re-exports the derive and arithmetic macros needed for downstream custom
units, so most consumers do not need to depend on `qtty-core` directly.

```rust
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, qtty::Unit)]
#[unit(crate = qtty, symbol = "smoot", dimension = qtty::Length, ratio = 1.7018)]
pub struct Smoot;

qtty::impl_unit_arithmetic_pairs_between!(qtty::unit::Meter, qtty::unit::Kilometer; Smoot);
```

## Examples

```bash
cargo run -p qtty --example quickstart
cargo run -p qtty --example dimensional_arithmetic
cargo run -p qtty --example all_units --features all-units
```

## Related crates

- `qtty-core`: primitive type system and built-in units
- `qtty-derive`: derive macro for custom units
- `qtty-ffi`: C-compatible ABI and adapter bridge

Repository docs:

- Workspace overview: [`../../doc/users/rust-workspace.md`](../../doc/users/rust-workspace.md)
- Repository layout: [`../../doc/architecture/repository-layout.md`](../../doc/architecture/repository-layout.md)

## License

AGPL-3.0 (see `../LICENSE`).
