# qtty

[![Crates.io](https://img.shields.io/crates/v/qtty.svg)](https://crates.io/crates/qtty)
[![Docs.rs](https://docs.rs/qtty/badge.svg)](https://docs.rs/qtty)

> **Strongly typed physical quantities for Rust, with astronomy-friendly units and zero-cost dimensional safety.**

`qtty` keeps units in the type system instead of in comments or conventions. Length, time, angle, mass, power, area,
volume, velocity, angular rate, and unitless ratios are modeled as typed quantities, so invalid operations fail at
compile time while valid conversions stay explicit and cheap.

---

## Table of Contents

1. [Supported Feature Flags](#supported-feature-flags)
2. [Features](#features)
3. [Installation](#installation)
4. [Quick Start](#quick-start)
5. [Scalar Types](#scalar-types)
6. [Included Units](#included-units)
7. [Examples](#examples)
8. [Workspace Layout](#workspace-layout)
9. [FFI](#ffi)
10. [Contributing](#contributing)
11. [License](#license)

---

## Supported Feature Flags

| Feature | Default | What it enables |
|---------|---------|-----------------|
| `std` | ✔ | Standard-library integration in `qtty-core` |
| `cross-unit-ops` | ✔ | Direct cross-unit comparisons for compatible built-in units (`==`, `<`, `>=`, …) |
| `alloc` | via `std` | Heap-backed helpers in `no_std`, including `qtty::qtty_vec!(vec ...)` |
| `serde` |  | `Serialize` / `Deserialize` support for quantities |
| `scalar-rational` |  | `num_rational::Rational64` scalar support |
| `pyo3` |  | PyO3 conversions for Python-facing integrations |
| `tiberius` |  | SQL Server integration helpers in `qtty-core` |
| `diesel` |  | Diesel SQL integration helpers in `qtty-core` |

> **Note:** `qtty` supports `no_std`. Disable default features to build without `std`.

---

## Features

| Category | What you get |
|----------|--------------|
| **Typed Quantities** | `Quantity<U, S>` keeps the unit `U` and scalar `S` at the type level, preventing invalid arithmetic across dimensions. |
| **Explicit Conversion** | Convert with `.to::<TargetUnit>()`, or use `.to_lossy()` for integer-backed quantities. |
| **Dimensional Arithmetic** | Multiplication and division compose dimensions at compile time: `Length * Length -> Area`, `Length / Time -> Velocity`, and more. |
| **Broad Unit Catalog** | Built-in modules cover angular, time, length, mass, power, area, volume, velocity, angular-rate, and unitless quantities. |
| **Astronomy-Friendly Units** | Includes `AstronomicalUnit`, `LightYear`, `Parsec`, `SolarMass`, `SolarLuminosity`, sidereal time units, and related helpers. |
| **Multiple Scalar Families** | Use `f64`, `f32`, signed integers, and optional decimal/rational scalars depending on your precision model. |
| **Interop Options** | Optional `serde`, `pyo3`, `diesel`, and `tiberius` support in Rust, plus a separate `qtty-ffi` crate for C-compatible consumers. |

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
qtty = "0.6.1"
```

Minimal `no_std` build:

```toml
[dependencies]
qtty = { version = "0.6.1", default-features = false }
```

`no_std` with heap-backed vectors/macros:

```toml
[dependencies]
qtty = { version = "0.6.1", default-features = false, features = ["alloc"] }
```

Serde support:

```toml
[dependencies]
qtty = { version = "0.6.1", features = ["serde"] }
```

---

## Quick Start

```rust
use qtty::{Degree, Radian};

let angle = Degree::new(180.0);
let radians = angle.to::<qtty::unit::Radian>();

assert!((radians.value() - core::f64::consts::PI).abs() < 1e-12);
```

```rust
use qtty::{Kilometer, Second};
use qtty::velocity::Velocity;

let distance = Kilometer::new(1_000.0);
let elapsed = Second::new(100.0);
let speed: Velocity<qtty::unit::Kilometer, qtty::unit::Second> = distance / elapsed;

assert!((speed.value() - 10.0).abs() < 1e-12);
```

Type errors happen at compile time:

```rust,compile_fail
use qtty::{Kilometer, Second};

let distance = Kilometer::new(1.0);
let time = Second::new(1.0);
let _ = distance + time;
```

---

## Scalar Types

The default scalar type is `f64`, but the same unit system is available across multiple scalar families:

- `qtty::*` / `qtty::f64::*` → `f64`
- `qtty::f32::*` → `f32`
- `qtty::i8::*`, `qtty::i16::*`, `qtty::i32::*`, `qtty::i64::*`, `qtty::i128::*` → signed integers
- `scalar-rational` feature → `num_rational::Rational64`

Integer quantities preserve unit safety for discrete data while exposing lossy conversions where appropriate.

---

## Included Units

The facade crate re-exports the built-in unit modules from `qtty-core` at the crate root:

- **Angular**: degrees, radians, arcminutes, arcseconds, milliarcseconds
- **Time**: seconds, minutes, hours, days, weeks, years, Julian/sidereal variants
- **Length**: meters, kilometers, astronomical units, light-years, parsecs, nautical miles, and more
- **Mass**: gram-family units, kilograms, tonnes, solar mass
- **Power**: watts, metric/electric horsepower, solar luminosity
- **Area / Volume**: square and cubic derived units plus liter-family and land units
- **Velocity / Angular Rate**: generic `Per`-based derived quantities
- **Dimensionless**: ratios and simplified same-unit division results

---

## Examples

Run the shipped examples from the workspace root:

```bash
cargo run -p qtty --example quickstart
cargo run -p qtty --example dimensional_arithmetic
cargo run -p qtty --example all_units --features all-units
```

Additional examples cover angles, astronomy, ratios, serialization, and Python interop.

---

## Workspace Layout

This repository is a Cargo workspace with four crates:

- `qtty` — user-facing facade crate
- `qtty-core` — core type system and built-in units
- `qtty-derive` — proc-macro for defining new unit marker types
- `qtty-ffi` — C-compatible ABI for non-Rust consumers

Further repository docs:

- User overview: [`../doc/users/rust-workspace.md`](../doc/users/rust-workspace.md)
- Architecture: [`../doc/architecture/repository-layout.md`](../doc/architecture/repository-layout.md)
- FFI documentation: [`qtty-ffi/README.md`](qtty-ffi/README.md)

---

## FFI

Need a stable C ABI or adapter boundary? Use `qtty-ffi`, which exposes:

- ABI-stable `UnitId`, `DimensionId`, and quantity structs
- Conversion and formatting helpers for C/C++
- Generated `include/qtty_ffi.h`
- Optional Rust-side `qtty_serde` carrier serialization and PyO3 helpers

See [`qtty-ffi/README.md`](qtty-ffi/README.md) for details.

---

## Contributing

- Add behavioral changes in the Rust crates first; adapters and vendored copies should follow.
- Run focused tests in the narrowest affected crate first.
- Keep public docs and examples aligned with the actual API.

---

## License

AGPL-3.0. See [`LICENSE`](LICENSE).
