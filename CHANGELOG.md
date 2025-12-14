# Changelog
All notable changes to this project are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
