# `qtty-derive`

[![Crates.io](https://img.shields.io/crates/v/qtty-derive.svg)](https://crates.io/crates/qtty-derive)
[![Docs.rs](https://docs.rs/qtty-derive/badge.svg)](https://docs.rs/qtty-derive)

Derive macro used by `qtty-core` to implement new unit marker types.

Most applications should depend on the `qtty` facade. Reach for `qtty-derive` only if you are extending the unit
catalogue yourself or building a crate that mirrors the `qtty` crate root (`Quantity`, `Unit`, …).

## Install

```toml
[dependencies]
qtty-core = "0.1.0"
qtty-derive = "0.1.0"
```

## Usage

```rust
use qtty_core::{length::{Length, Meter}, Quantity};
use qtty_derive::Unit as UnitDerive;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, UnitDerive)]
#[unit(symbol = "fur", dimension = Length, ratio = 201.168)]
pub struct Furlong;

let dist: Quantity<Furlong> = Quantity::new(1.0);
assert!((dist.to::<Meter>().value() - 201.168).abs() < 1e-12);
```

The derive generates both the `Unit` impl (ratio, dimension tag, and symbol) and a `Display` implementation for
`Quantity<Furlong>` that prints `<value> <symbol>`.

## Attribute reference

The macro reads a required `#[unit(...)]` attribute with the following keys:

- `symbol = "m"`: string literal printed by the generated `Display` impl.
- `dimension = Length`: dimension marker type implementing `Dimension`.
- `ratio = 1.0`: conversion ratio to the canonical unit of the dimension (usually 1.0 for the base unit).

Additional metadata can be added in the future without breaking callers (unknown keys result in a compile error).

## License

AGPL-3.0 (see `../LICENSE`).
