# `qtty-core`

[![Crates.io](https://img.shields.io/crates/v/qtty-core.svg)](https://crates.io/crates/qtty-core)
[![Docs.rs](https://docs.rs/qtty-core/badge.svg)](https://docs.rs/qtty-core)

Zero-cost building blocks for strongly typed physical quantities.

This crate contains the minimal type system used by the `qtty` facade:

- [`Quantity<U>`] — an `f64` tagged with a zero-sized unit marker type
- [`Unit`] and [`Per<N, D>`] — traits/types that encode conversion ratios and derived units
- Predefined unit modules grouped by dimension (length, time, mass, angular, power, frequency, velocity)

Most users should depend on `qtty`. Reach for `qtty-core` when you need the primitives directly (custom units,
embedded/`no_std` builds, serialization without the facade, etc.).

## Install

```toml
[dependencies]
qtty-core = "0.1.0"
```

## Quick start

Convert between built-in units:

```rust
use qtty_core::length::{Kilometers, Meter};

let km = Kilometers::new(1.25);
let m = km.to::<Meter>();
assert!((m.value() - 1250.0).abs() < 1e-12);
```

Compose derived units:

```rust
use qtty_core::length::{Meter, Meters};
use qtty_core::time::{Second, Seconds};
use qtty_core::velocity::Velocity;

let distance = Meters::new(100.0);
let time = Seconds::new(20.0);
let v: Velocity<Meter, Second> = distance / time;
assert!((v.value() - 5.0).abs() < 1e-12);
```

## Defining your own unit

`qtty-core` pairs with the `qtty-derive` proc-macro to make new unit marker types trivial:

```rust
use qtty_core::{length::{Length, Meter}, Quantity};
use qtty_derive::Unit as UnitDerive;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, UnitDerive)]
#[unit(symbol = "fur", dimension = Length, ratio = 201.168)]
pub struct Furlong;

let dist: Quantity<Furlong> = Quantity::new(3.0);
assert!((dist.to::<Meter>().value() - 603.504).abs() < 1e-12);
```

The derive fills in the `Unit` impl and a `Display` implementation automatically.

## `no_std`

Disable default features to build `qtty-core` without `std` (it falls back to `libm` for floating-point math):

```toml
[dependencies]
qtty-core = { version = "0.1.0", default-features = false }
```

## Feature flags

- `std` (default): enables `std` support.
- `serde`: serializes/deserializes `Quantity<U>` as bare `f64` values.

## License

AGPL-3.0 (see `../LICENSE`).
