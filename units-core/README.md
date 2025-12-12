# `unit-core`

Core, zero-cost building blocks for strongly typed physical units.

Most users should depend on `unit` instead of this crate. Use `unit-core` directly if you want the minimal type
system (`Quantity`, `Unit`, `Per`, …) without the facade re-exports.

## Install

```toml
[dependencies]
unit-core = "0.1.0"
```

## `no_std`

`unit-core` supports `no_std` with `libm` used for required floating-point math.

```toml
[dependencies]
unit-core = { version = "0.1.0", default-features = false }
```

## Features

- `std` (default): enables `std` support.
- `serde`: enables `serde` support for `Quantity<U>` (serialize/deserialize as a bare `f64`).

## License

AGPL-3.0 (see `../LICENSE`).
