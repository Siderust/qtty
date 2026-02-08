# Changelog
All notable changes to this project are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Added support for operations with Rust built-in numeric types, improving ergonomics when combining `Quantity` values with primitive scalars.
- Full dimensional arithmetic support using compile-time exponent math (`Dim`, `DimMul`, `DimDiv`) powered by `typenum`.
- New product unit type `Prod<A, B>` to represent unit multiplication (`Length * Length`, `Area * Length`, etc.).
- New `area` unit module with metric, land, and imperial/US units (for example `SquareMeter`, `Hectare`, `Acre`).
- New `volume` unit module with metric cubic units, liter-family units, and imperial/US units (for example `CubicMeter`, `Liter`, `UsGallon`).
- New `qtty` example `dimensional_arithmetic` demonstrating compile-time dimensional composition and conversions.
- `Quantity::eq_unit` and `Quantity::cmp_unit` helpers for comparing values across different units in the same dimension.
- Cross-unit comparison operator support (`==`, `!=`, `<`, `>`, `<=`, `>=`) via `impl_unit_conversions!`, with unit conversion applied before comparison.
- Expanded `qtty-core` comparison tests covering same-unit ordering, scalar ordering, cross-unit comparisons, NaN behavior, and integer `Eq`/`Ord` use cases.

### Changed
- Division and multiplication now compose dimensions generically at the type level, so multiplied quantities produce `Quantity<Prod<...>>` and can be converted to named units with `.to()`.
- Core/base dimensions are now unified under the new generic `Dim<...>` model, with backward-compatible aliases for `DivDim` and new `MulDim`.
- Public exports now include area/volume modules and additional dimension aliases from `qtty-core` and the `qtty` facade.
- `Quantity` ordering/equality trait implementations were refined: `PartialOrd` is now implemented explicitly (same-unit and scalar comparisons), and `Eq`/`Ord` are enabled when the scalar type supports total equality/ordering.

## [0.2.2] - 2026-01-13

### Added
- `qtty-core` optional `diesel` feature with `Quantity` SQL mapping, query integration, and examples.
- `qtty-core` optional `tiberius` feature with SQL Server `ToSql`/`FromSql` support for `Quantity`.
- `qtty-core` optional `pyo3` feature with `Quantity` conversions to/from Python floats.
- `qtty_core::serde_with_unit` helper for serializing quantities with unit symbols.
- New `qtty-core` tests covering core, Diesel, PyO3, Serde, and Tiberius integrations.

### Changed
- `qtty-core` internals split into feature-gated modules with updated docs and examples.

## [0.2.1] - 2025-12-22

### Added
- Optional `python` feature for `qtty-ffi` that exposes `UnitId` as a PyO3 `pyclass` with pickle support, enabling Python consumers alongside C.
- Generated unit symbol lookups and new `UnitId::symbol()` accessor for retrieving canonical unit symbols from FFI.
- Convenience APIs on `QttyQuantity` for dimension/compatibility queries, conversions, and basic arithmetic on FFI quantities.
- `QttyDerivedQuantity` FFI type for compound quantities (numerator/denominator) with conversion and scalar helpers (e.g., velocities).

### Changed
- `qtty-ffi` build tooling now emits symbol tables and uses the updated cbindgen (0.29.2) plus parser deps to support Python-aware builds.

## [0.2.0] - 2025-12-14

### Added
- Workspace split into crates: `qtty` (facade), `qtty-core` (types + units), `qtty-derive` (proc-macro).
- Feature flags: `std` (default) and optional `serde` for `Quantity<U>`.
- `no_std` support in `qtty-core` (uses `libm` for floating-point math not in `core`).
- Predefined unit modules under `qtty-core::units` (angular, time, length, mass, power, velocity, frequency, unitless).
- **Serde with unit information**: New `qtty_core::serde_with_unit` helper module for serializing quantities with unit symbols. Use `#[serde(with = "qtty_core::serde_with_unit")]` on fields to preserve unit information in JSON/serialized data (e.g., `{"value": 100.0, "unit": "m"}`). Includes unit validation on deserialization. Default serialization remains compact (bare `f64` value).
- **Length**: Extensive new SI-prefixed meter units (yoctometer through yottameter) and additional units (fathom, nautical mile, light year, parsec, etc.).
- **Mass**: Full SI prefix ladder for gram (yoctogram through yottagram), additional units (ton, metric ton, tonne), and nominal astronomical masses (Earth, Jupiter, Sun).
- **Power**: Complete SI prefix ladder for watt (yoctowatt through yottawatt), erg per second, metric horsepower, electric horsepower, and solar luminosity.
- **Time**: Full SI submultiples (attosecond through decisecond) and multiples (decasecond through terasecond), additional civil units (fortnight, decade, millennium), Julian conventions, and astronomical mean units (sidereal day/year, synodic month).
- **Velocity**: Generic `Velocity<Length, Time>` type for composing any length/time unit pair.
- **Frequency**: Generic `Frequency<Angle, Time>` type for composing any angle/time unit pair.

### Changed
- Documentation rewrite for docs.rs (crate docs, READMEs, examples).
- **Time module**: Canonical scaling unit changed from `Day` to `Second` (SI base unit). All time units now express ratios in seconds.
- **Unit symbols**: Updated for consistency (e.g., `Second::SYMBOL` changed from `"sec"` to `"s"`).
- **Velocity and Frequency**: Refactored to use generic parameterized types instead of specific aliases (e.g., `Velocity<Kilometer, Second>` instead of `KilometersPerSecond`).
- Import organization in examples for improved clarity and consistency.
- Conversion constants and ratios updated across all unit modules for accuracy and consistency.
 - **Unitless refactor**: `Unitless` changed from a `pub type Unitless = f64` alias to a proper zero-sized marker type (`pub struct Unitless;`). The `Unit` impl for `Unitless` remains (`RATIO = 1.0`, `Dim = Dimensionless`, `SYMBOL = ""`) while removing the implicit `Unit` implementation for `f64`. API ergonomics preserved: `Quantity<Unitless>` display/From conversions/Simplify behavior unchanged. Updated docs, tests, and examples accordingly.

### Deprecated
- `define_unit!` is retained for internal use and backward compatibility; new units in `qtty-core` use `#[derive(Unit)]`.
- Specific velocity type aliases (e.g., `MetersPerSecond`, `KilometersPerSecond`) in favor of generic `Velocity<N, D>` type.
- Specific frequency type aliases (e.g., `RadiansPerSecond`, `DegreesPerDay`) in favor of generic `Frequency<N, D>` type.

### Fixed
- `qtty` feature flags now correctly control `qtty-core` defaults (including `no_std` builds).
- Improved type safety and consistency across velocity and frequency unit definitions.


## [0.0.0] - 2025-09-01

- Migration from Siderust
