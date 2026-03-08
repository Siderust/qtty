# qtty

[![Crates.io](https://img.shields.io/crates/v/qtty.svg)](https://crates.io/crates/qtty)
[![Docs.rs](https://docs.rs/qtty/badge.svg)](https://docs.rs/qtty)

The user-facing crate for strongly typed physical quantities, conversions, and astronomy-friendly units.

`qtty` re-exports the `qtty-core` type system plus a curated set of built-in units so you can write dimensionally safe
code without giving up ergonomics.

## Highlights

- Zero-cost `Quantity<U, S>` model with compile-time unit safety
- Explicit conversions via `.to::<TargetUnit>()`
- Dimensional arithmetic for area, volume, velocity, frequency, and other derived quantities
- Built-in astronomy-oriented units such as `AstronomicalUnit`, `LightYear`, `Parsec`, `SolarMass`, and `SolarLuminosity`
- `no_std` support, optional `serde`, optional PyO3/SQL integrations, and integer scalar families

## Install

```toml
[dependencies]
qtty = "0.4.1"
```

Disable default features for `no_std`:

```toml
[dependencies]
qtty = { version = "0.4.1", default-features = false }
```

Enable heap-backed helpers in `no_std`:

```toml
[dependencies]
qtty = { version = "0.4.1", default-features = false, features = ["alloc"] }
```

## Quick start

```rust
use qtty::{Degrees, Radian};

let angle = Degrees::new(90.0);
let radians = angle.to::<Radian>();
assert!((radians.value() - core::f64::consts::FRAC_PI_2).abs() < 1e-12);
```

```rust
use qtty::{Kilometer, Kilometers, Second, Seconds};
use qtty::velocity::Velocity;

let distance = Kilometers::new(1_000.0);
let elapsed = Seconds::new(100.0);
let speed: Velocity<Kilometer, Second> = distance / elapsed;
assert!((speed.value() - 10.0).abs() < 1e-12);
```

## Feature flags

- `std` (default): enables `std` support in `qtty-core`
- `cross-unit-ops` (default): enables direct cross-unit comparison operators for compatible built-in units
- `alloc`: enables heap-backed helpers such as `qtty::qtty_vec!(vec ...)` in `no_std`
- `serde`: enables serialization helpers for quantities
- `scalar-decimal`: enables `rust_decimal::Decimal`
- `scalar-rational`: enables `num_rational::Rational64`
- `pyo3`: enables PyO3 conversions and Python-facing helpers
- `tiberius`: enables SQL Server integration helpers
- `diesel`: enables Diesel integration helpers

## Modules

- `angular`, `time`, `length`, `mass`, `power`
- `area`, `volume`
- `velocity`, `frequency`
- `unitless`
- scalar-specific facades: `f32`, `f64`, `i8`, `i16`, `i32`, `i64`, `i128`

## Examples

```bash
cargo run -p qtty --example quickstart
cargo run -p qtty --example dimensional_arithmetic
cargo run -p qtty --example all_units
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
