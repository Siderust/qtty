# `qtty`

[![Crates.io](https://img.shields.io/crates/v/qtty.svg)](https://crates.io/crates/qtty)
[![Docs.rs](https://docs.rs/qtty/badge.svg)](https://docs.rs/qtty)

Strongly typed physical and astronomy-friendly units built around a tiny, zero-cost type system.

This repository is a Cargo workspace containing four crates:

- `qtty` — the user-facing crate that re-exports the core API plus curated unit modules.
- `qtty-core` — the minimal type system (`Quantity`, `Unit`, `Per`, …) and predefined units.
- `qtty-derive` — a proc-macro used to implement new `Unit` marker types.
- `qtty-ffi` — a C-compatible ABI for constructing and converting quantities outside of Rust.

Most users should depend on the `qtty` crate; the other crates exist for advanced/custom use cases.

## Install

```toml
[dependencies]
qtty = "0.1.0"
```

Disable default features for `no_std` (uses `libm` behind the scenes):

```toml
[dependencies]
qtty = { version = "0.1.0", default-features = false }
```

## Quick start

```rust
use qtty::{Degrees, Radian};

let a = Degrees::new(180.0);
let r = a.to::<Radian>();
assert!((r.value() - core::f64::consts::PI).abs() < 1e-12);
```

## Features

- `std` (default): enables `std` support in `qtty-core`.
- `serde`: serializes/deserializes `Quantity<U>` as bare `f64` values.

Need a C ABI? Use the companion `qtty-ffi` crate (see `qtty-ffi/README.md`).

## Stability & documentation

This workspace is currently `0.x`. Expect breaking changes between minor versions until `1.0`.

- Docs: https://docs.rs/qtty (published versions of the facade crate)
- Repo: https://github.com/Siderust/qtty
- FFI guide: see `qtty-ffi/README.md`

## License

AGPL-3.0 (see `LICENSE`).
