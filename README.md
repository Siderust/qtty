# `unit`

Strongly typed physical and astronomy-friendly units built around a small, zero-cost core.

This repository is a Cargo workspace containing three crates:

- `unit` — the user-facing crate that re-exports the full API and a set of predefined units.
- `unit-core` — the type system (`Quantity`, `Unit`, `Per`, …) and the predefined unit modules.
- `unit-derive` — an internal proc-macro used by `unit-core` to define units.

## Install

```toml
[dependencies]
unit = "0.1.0"
```

## Quick start

```rust
use unit::{Degrees, Radian};

let a = Degrees::new(180.0);
let r = a.to::<Radian>();
assert!((r.value() - core::f64::consts::PI).abs() < 1e-12);
```

## Features

- `std` (default): enables `std` support in `unit-core`.
- `serde`: enables `serde` support for `Quantity<U>` (serialize/deserialize as a bare `f64`).

Disable default features for `no_std`:

```toml
[dependencies]
unit = { version = "0.1.0", default-features = false }
```

## Documentation

- API docs: `https://docs.rs/unit` (published versions)
- Workspace repository: `https://github.com/Siderust/units`

## License

AGPL-3.0 (see `LICENSE`).
