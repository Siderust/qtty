# Changelog
All notable changes to this project are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- New `qtty` crate feature `alloc` for heap-backed helpers in `no_std` builds. (see #10)
- Integration compile checks for `qtty::qtty_vec!` across `std`, `no_std + alloc`, and pure `no_std` modes. (see #10)
- New integer scalar facade modules `qtty::i8`, `qtty::i16`, and `qtty::i128`, mirroring the unit aliases available in `qtty::i32`. (see #11)
- New `cross-unit-ops` feature in `qtty-core` and `qtty` (enabled by default) to gate generation of direct cross-unit comparison operator impls (`==`, `<`, etc.). (see #15)
- New reduced-mode CI profile (`No Cross-Unit Ops`) plus targeted compile checks validating `eq_unit`/`cmp_unit` and ensuring direct cross-unit operators are disabled when the feature is off. (see #15)
- Documented compile-time benchmark commands (`cargo +nightly -Z timings`) for comparing default and reduced-mode builds. (see #15)

### Fixed
- `qtty::qtty_vec!(vec ...)` no longer hardcodes `std`; it now works with `alloc` in `no_std` builds. (see #10)
- In pure `no_std` (without `alloc`), `qtty::qtty_vec!(vec ...)` now fails with a clear feature requirement message while array form continues to work. (see #10)
- `qtty` crate docs now match the public integer module surface (`i8`, `i16`, `i32`, `i64`, `i128`) and include coverage for integer `to_lossy()` flows in facade integration tests. (see #11)
- Unit-erasure conversion into `Quantity<Unitless>` is no longer limited to length units; time, mass, angular, and other supported non-dimensionless units now convert while preserving the raw scalar value (no normalization). (see #12)
- Removed `DivAssign<Self>` for `Quantity` because `quantity /= quantity` is dimensionally unsound; `/=` is now scalar-only (`DivAssign<S>`). Migration: replace `q /= other_q` with `q = (q / other_q).simplify()` when you need a unitless ratio, or use explicit scalar division where appropriate. (see #14)
- Reduced quadratic impl bloat in built-in unit catalogs by splitting conversion generation from cross-unit comparison generation; reduced mode now keeps `From` conversions while omitting cross-unit operator impls. (see #15)

## [0.3.0] - 2026-02-09

### Added
- Added support for operations with Rust built-in numeric types, improving ergonomics when combining `Quantity` values with primitive scalars.
- Added `Iterator::sum` support for `Quantity`, including ergonomic accumulation into `f64` from iterators of `Quantity<_, f64>` (owned or borrowed items).
- Full dimensional arithmetic support using compile-time exponent math (`Dim`, `DimMul`, `DimDiv`) powered by `typenum`.
- New product unit type `Prod<A, B>` to represent unit multiplication (`Length * Length`, `Area * Length`, etc.).
- New `area` unit module with metric, land, and imperial/US units (for example `SquareMeter`, `Hectare`, `Acre`).
- New `volume` unit module with metric cubic units, liter-family units, and imperial/US units (for example `CubicMeter`, `Liter`, `UsGallon`).
- New `qtty` example `dimensional_arithmetic` demonstrating compile-time dimensional composition and conversions.
- `Quantity::eq_unit` and `Quantity::cmp_unit` helpers for comparing values across different units in the same dimension.
- Cross-unit comparison operator support (`==`, `!=`, `<`, `>`, `<=`, `>=`) via `impl_unit_conversions!`, with unit conversion applied before comparison.
- Expanded `qtty-core` comparison tests covering same-unit ordering, scalar ordering, cross-unit comparisons, NaN behavior, and integer `Eq`/`Ord` use cases.
- `Quantity::mean` helper to compute the arithmetic midpoint between two values of the same quantity type (including integer scalar support).
- New exported `qtty::qtty_vec!` macro for building typed quantity arrays and `Vec`s directly from scalar literals.

### Changed
- Division and multiplication now compose dimensions generically at the type level, so multiplied quantities produce `Quantity<Prod<...>>` and can be converted to named units with `.to()`.
- Core/base dimensions are now unified under the new generic `Dim<...>` model, with backward-compatible aliases for `DivDim` and new `MulDim`.
- Public exports now include area/volume modules and additional dimension aliases from `qtty-core` and the `qtty` facade.
- `Quantity` ordering/equality trait implementations were refined: `PartialOrd` is now implemented explicitly (same-unit and scalar comparisons), and `Eq`/`Ord` are enabled when the scalar type supports total equality/ordering.
- Unit display symbols were standardized to canonical scientific notation across affected units (for example `Degree` now renders as `°`, `Radian` as `rad`, and SI symbols like `km`/`μm` are used consistently).

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
