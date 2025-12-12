# `qtty-core`

Core, zero-cost building blocks for strongly typed physical units.

Most users should depend on `qtty` instead of this crate. Use `qtty-core` directly if you want the minimal type
system (`Quantity`, `Unit`, `Per`, …) without the facade re-exports.

## Install

```toml
[dependencies]
qtty-core = "0.1.0"
```

## `no_std`

`qtty-core` supports `no_std` with `libm` used for required floating-point math.

```toml
[dependencies]
qtty-core = { version = "0.1.0", default-features = false }
```

## Features

- `std` (default): enables `std` support.
- `serde`: enables `serde` support for `Quantity<U>` (serialize/deserialize as a bare `f64`).

## License

AGPL-3.0 (see `../LICENSE`).
