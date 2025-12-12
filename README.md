# `qtty`

Strongly typed physical and astronomy-friendly units built around a small, zero-cost core.

This repository is a Cargo workspace containing three crates:

- `qtty` — the user-facing crate that re-exports the full API and a set of predefined units.
- `qtty-core` — the type system (`Quantity`, `Unit`, `Per`, …) and the predefined qtty modules.
- `qtty-derive` — an internal proc-macro used by `qtty-core` to define units.

## Install

```toml
[dependencies]
qtty = "0.1.0"
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
- `serde`: enables `serde` support for `Quantity<U>` (serialize/deserialize as a bare `f64`).

Disable default features for `no_std`:

```toml
[dependencies]
qtty = { version = "0.1.0", default-features = false }
```

## Documentation

- API docs: `https://docs.rs/qtty` (published versions)
- Workspace repository: `https://github.com/Siderust/qtty`

## License

AGPL-3.0 (see `LICENSE`).
