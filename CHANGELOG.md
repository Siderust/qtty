# Changelog

All notable changes to this project are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2026-04-15

### Added

- **Quantity-level real-number rounding helpers** (`qtty-core`, re-exported by
  `qtty`) — `Quantity<U, S>` for `S: Real` now exposes `floor()`, `ceil()`,
  `round()`, `trunc()`, and `fract()`, matching the scalar `Real` surface while
  preserving unit types through common rounding workflows.

- **`assert_units_are_builtin!`** (`qtty-core`, `#[doc(hidden)]`) — compile-time
  assertion macro driven by each dimension's inventory macro under `#[cfg(test)]`.
  Uses a supertrait bound pattern: adding a unit to a dimension inventory without
  also registering it in `register_builtin_units!` becomes a compile error. Catches
  the most common drift case (new unit added to the dimension file but the
  cross-dimension registry not updated).

- **Facade consistency test** (`qtty/tests/inventory_consistency.rs`) — compile-time
  integration test that uses exported inventory macros to assert every always-available
  unit in `angular`, `length`, `time`, `mass`, `power`, `area`, and `volume`
  is both re-exported in `qtty::unit::*` and has a root quantity alias in `qtty::*`.
  Adding a unit to an inventory but forgetting the corresponding `lib.rs` re-export
  or root alias generation now fails CI instead of silently becoming a missing export.

- New public **acceleration / force / energy** unit families across `qtty-core`
  and `qtty`, including `Accel<L, T>`, `MeterPerSecondSquared`,
  `StandardGravity`, SI newton/joule ladders, and feature-gated
  `PoundForce`, `Dyne`, `Calorie`, and electron-volt units.

- **`SquareOf<L>` / `CubeOf<L>` helper aliases** — new typed composition helpers
  for area and volume quantities derived from multiplied length quantities.

- New **stable unit arithmetic layer** (`unit_arithmetic` module) with `UnitDiv` and `UnitMul` extension traits that control output types for quantity division and multiplication, replacing the previous blanket impls.
- Generic recovery impls: `U / U → Unitless`, `N / Per<N, D> → D`, `Per<N, D> * D → N`, `D * Per<N, D> → N`.
- Macro-generated fallback pair tables for all built-in unit marker types: cross-unit division produces `Per<A, B>`, multiplication produces `Prod<A, B>`.
- Exported macros `impl_unit_division_pairs!`, `impl_unit_multiplication_pairs!`, and `impl_unit_arithmetic_pairs!` for downstream custom units to opt into the same generated arithmetic.
- `asin`, `acos`, and `atan` methods on `Quantity<Unitless, S>` (moved from `Quantity<Per<U, U>>`) so same-unit ratios keep ergonomic trig behavior.
- Comprehensive compile-time and runtime tests for unit arithmetic covering all recovery patterns, cross-unit pairs, and custom-unit registration.
- Added invalid-unit regression coverage for `qtty-ffi` quantity carriers so raw `u32` unit IDs from C callers are rejected cleanly instead of producing undefined behavior.
- Added serde round-trip coverage for the Rust-side `qtty-ffi` carrier structs using their raw numeric unit IDs.
- **`qtty-ffi` Area & Volume FFI coverage** — `DimensionId::Area` (6) and `DimensionId::Volume` (7) are now part of the ABI, exposing 11 area units (`SquareMeter` … `SquareDecimeter`) and 13 volume units (`CubicMeter` … `UsTeaspoon`) with stable discriminant ranges 60000–60010 and 70000–70012 respectively.
- **`qtty-ffi` `discriminants.csv`** — new file that is the sole source of ABI-stable discriminant values. All unit metadata (ratios, symbols) is now derived at compile time from `<Type as qtty::Unit>::RATIO`/`::SYMBOL`, eliminating the historic dual-source-of-truth between `units.csv` and qtty-core.
- **`qtty` crate-root module re-exports** — `qtty::{area, volume, acceleration, force, energy}` are now re-exported at the crate root, and root quantity aliases are generated from the inventory macros for all built-in scalar families.
- **Auto-generated FFI unit constants** — `qtty-ffi` now emits named `QTTY_UNIT_*`
  constants from `discriminants.csv` for the full registry instead of maintaining
  a hand-written subset.
- `AngularRate<N, D>` type alias and `AngularRateUnit` trait as the primary
  names for angular-rate quantities (`Angular / Time`), replacing the
  misleading `Frequency` / `FrequencyUnit` names. The dimension alias is now
  `AngularRate`.
- `Exact::checked_from_f64(value: f64) -> Option<Self>` for converting a
  floating-point value to an exact scalar without silent overflow; returns
  `None` when the value is out of range or non-representable. (QTTY-003)
- `Quantity::checked_to_lossy<T>() -> Option<Quantity<T, S>>` for safe
  cross-unit conversion when the scalar type is `Exact`; returns `None`
  instead of silently saturating.
- 18 regression tests in `qtty-core/tests/audit_regressions.rs` covering
  cross-unit comparison symmetry, integer `abs()` boundary behaviour, and
  lossy/checked conversion semantics.
- **`MulAssign<S>`** for `Quantity<U, S>` — in-place scalar multiplication
  (`q *= 2.0`), symmetric with the pre-existing `DivAssign<S>`.
- **`Rem<Quantity<U, S>>`** for same-unit remainder — `5 m % 3 m == 2 m`,
  complementing the pre-existing scalar `Rem<S>`.
- **`DAYS_PER_GREGORIAN_YEAR`** named constant (`qtty-core::time`) — replaces
  the four repeated `365.242_5` literals in `Year`/`Decade`/`Century`/`Millennium`
  ratio expressions.

### Removed
- **`qtty-ffi` `units.csv`** — hardcoded ratio/symbol data removed; metadata is now derived from qtty-core trait constants so divergence is impossible at compile time.
- **`qtty-ffi` deprecated helper functions** — `meters_into_ffi`, `try_into_meters`, `kilometers_into_ffi`, `try_into_kilometers`, `seconds_into_ffi`, `try_into_seconds`, `minutes_into_ffi`, `try_into_minutes`, `hours_into_ffi`, `try_into_hours`, `days_into_ffi`, `try_into_days`, `radians_into_ffi`, `try_into_radians`, `degrees_into_ffi`, `try_into_degrees` (all deprecated since 0.5.1) have been removed. Use `From`/`TryFrom` directly (`qty.into()`, `qty.try_into()`).
- Removed the `scalar-decimal` feature and `rust_decimal` scalar support from `qtty-core` and the `qtty` facade crate.
- **Breaking:** Removed the public `Simplify` trait and `.simplify()` method from `qtty-core` and the `qtty` facade crate; unit arithmetic now resolves these cases at compile time.
- **Breaking:** Removed the deprecated `frequency` alias modules from `qtty-core` and `qtty`. Use `angular_rate` for `Angular / Time` quantities.
- **Breaking:** Removed deprecated `Quantity<Unitless>::asin()`, `acos()`, and `atan()` scalar-returning methods. Use `asin_angle()`, `acos_angle()`, and `atan_angle()` instead.

### Changed
- Downstream crates can now keep more rounding and timestamp-normalization logic
  in typed quantities instead of erasing units to raw floating-point values for
  `floor`/`ceil`/`round`/fractional extraction.

- **CODATA 2018 → 2022** (`fundamental-physics`) — Updated Bohr radius,
  classical electron radius, and electron reduced Compton wavelength to the
  CODATA 2022 recommended values. Planck length and atomic mass unit are
  unchanged (identical in both adjustments). Affected constants and their
  tests updated accordingly.
- **Internal: inventory macros are now the canonical unit lists** — every
  always-available unit family in `qtty-core` (`angular`, `length`, `time`,
  `mass`, `power`, `area`, `volume`, `acceleration`, `force`, `energy`) and the
  relevant feature-gated subfamilies now expose a `{family}_units!($cb:path)`
  macro that drives conversions, cross-unit ops, facade alias generation,
  FFI registry generation, and builtin-unit checks. No public API change.

- **Internal: nominal length units have full pairwise `From` conversions** —
  `length_nominal_units!` now drives `impl_unit_from_conversions!` for all 8 nominal
  units (`SolarRadius`, `SolarDiameter`, `EarthRadius`, `EarthEquatorialRadius`,
  `EarthPolarRadius`, `JupiterRadius`, `LunarRadius`, `LunarDistance`). Previously
  only `SolarRadius ↔ Kilometer` was generated. The explicit cross-group pair is
  retained.

- **Breaking:** Metric area marker types `SquareMeter`, `SquareKilometer`,
  `SquareCentimeter`, and `SquareMillimeter` are now `Prod<Length, Length>`
  aliases. `side * side` can therefore be assigned directly to `SquareMeters`,
  but symbol behavior now follows `Prod` (`Display` renders `m·m`; there is no
  dedicated `SquareMeter::SYMBOL` value like `m²`).

- **Breaking:** Same-unit division (`Meter / Meter`) now directly returns `Quantity<Unitless>` instead of `Quantity<Per<Meter, Meter>>`. Code that type-annotated the result as `Quantity<Per<U, U>>` must be updated.
- **`qtty-ffi` build pipeline** — `build.rs` now resolves all unit metadata by extracting inventory types from qtty-core source files and emitting `<Type as qtty::Unit>::RATIO` / `::SYMBOL` expressions directly. Hardcoded floats are gone; the generated registry is always in sync with qtty-core by construction.
- **`qtty-ffi` consistency test** — `tests/csv_inventory_consistency.rs` replaced by `tests/consistency.rs`. The new test uses only compile-time `UnitId::$variant` assertions (forward check) and a lightweight runtime smoke-test that verifies trait-derived metadata matches the registry. The `KNOWN_RATIO_DIVERGENCES` / `KNOWN_SYMBOL_DIVERGENCES` workaround lists are gone.
- **Breaking:** `Per<N, D> * D` and `D * Per<N, D>` now directly return the numerator quantity (e.g., `Quantity<Meter>`) instead of `Quantity<Prod<Per<N, D>, D>>`. The `.to()` call to recover the numerator is no longer needed.
- **Breaking:** `N / Per<N, D>` now directly returns the denominator quantity (e.g., `Quantity<Second>`) instead of `Quantity<Per<N, Per<N, D>>>`.
- **Breaking:** `asin`/`acos`/`atan` are now on `Quantity<Unitless>` instead of `Quantity<Per<U, U>>`. Since same-unit division now yields `Unitless` directly, this is transparent for `(a / b).asin()` patterns.
- `qtty-ffi` quantity carrier fields and C-facing unit parameters now use raw `u32`/`uint32_t` unit IDs, and `qtty_ffi_version()` now reports ABI version `500`.
- `Quantity::sqrt()` was renamed to `Quantity::scalar_sqrt()` to make it explicit that the operation returns the underlying scalar rather than a quantity with the original unit type.
- **Breaking:** Dimension aliases `VelocityDim` and `AngularRateDim` are now
  `Velocity` and `AngularRate`. At the `qtty` crate root, `Velocity` and
  `AngularRate` now name dimensions; the quantity aliases remain under
  `qtty::velocity::Velocity<L, T>` and `qtty::frequency::AngularRate<N, D>`.
- **Breaking:** `Frequency<N, D>`, `FrequencyUnit`, and `FrequencyDim` are
  removed. Use `AngularRate<N, D>`, `AngularRateUnit`, and `AngularRate`.
- Canonical symbols were tightened for two built-in units: `Turn` now renders as
  `tr` and `Century` now renders as `c`.
- Cross-unit comparison (`==`, `<`, …) is now **symmetric**: both operands
  are independently scaled to the same reference unit before comparison,
  eliminating the previous asymmetry where `a == b` could differ from
  `b == a` after a floating-point round-trip.

### Fixed
- **Pound-force value corrected** (`qtty-core::force`, feature `customary`) —
  `PoundForce::RATIO` was `4.448_222_615_260_5` (typo in digit 6) instead of the
  exact NIST SP 1247 value `4.448_221_615_260_5 N` (= `g₀ × 1 lb`). The error
  was one part per million and affected all conversions involving `PoundForce`.
  The accompanying test and docstring were updated to match.

- **Year/Decade/Century/Millennium docstrings** (`qtty-core::time`) — doc
  comments incorrectly described the `Year` struct and its multiples as *"mean
  tropical year"*. The ratio used (`365.242 5 d = DAYS_PER_GREGORIAN_YEAR`) is the
  **Gregorian calendar mean year** (`365 + 97/400`), not the astronomical mean
  tropical year (≈ 365.242 19 d, which differs by ~27 s/yr). Docstrings for
  `Year`, `Decade`, `Century`, and `Millennium` now correctly say "mean Gregorian
  year" and explain the distinction. A stale `// (tropical year)` test comment in
  `angular_rate.rs` was corrected to match.

- **Removed `unitless.rs` tombstone file** (`qtty-core`) — a file containing only
  a comment ("The `Unitless` type has been removed") was left dangling after the
  removal of the `Unitless` type in a prior release. It was never included in any
  `mod` declaration and served no purpose; deleted to avoid confusion.

- **`f32` `to_const` precision** — ratio conversion for `f32`-backed quantities
  now performs the `U::RATIO / T::RATIO` division in `f64` before casting to
  `f32`, matching the `f64` code path and avoiding doubled rounding error from
  two independent `as f32` casts.
- Cleaned up stale `scalar-decimal` cfg gates, tests, and documentation left behind by the Decimal removal so current builds no longer emit `unexpected_cfgs` warnings.
- Made `Quantity::mean()` overflow-safe for integer-backed quantities by avoiding addition before division.
- Made `Degrees::from_dms()` and `HourAngles::from_hms()` safe for `i32::MIN` inputs by avoiding signed integer negation before widening.
- Fixed `qtty-core` pure `no_std` test builds by gating std-dependent internal test modules behind the `std` feature.
- Fixed `qtty` pure `no_std` test/example target checks by gating std-only integration tests and the `all_units` example.
- Generalized generated pairwise unit `From`/`Into` conversions across `Real` scalar types so non-default scalar modules such as `qtty::f32` get the same conversion ergonomics as the default `f64` surface.
- Updated crate docs and README dependency snippets to the current `0.5.0` release line.
- Integer `abs()` no longer panics in debug builds on the minimum signed
  value (e.g. `i32::MIN`); it now uses `saturating_abs()`, returning
  `i32::MAX`. (QTTY-002)
- `qtty-ffi` formatting now clamps requested precision to 100 to avoid
  pathological allocations and returns `QTTY_ERR_BUFFER_TOO_SMALL` for
  zero-length output buffers instead of `QTTY_ERR_NULL_OUT`.
- `to_lossy()` documentation now explicitly describes truncation and
  saturation semantics for integer scalars and warns that the result may
  not equal the original value. (QTTY-003)
- Crate-level docs in `qtty-core` and `qtty` corrected: removed the claim
  that all quantities are "backed by an `f64`"; serde section now says "raw
  scalar value" instead of "raw `f64` value".

## [0.5.0] - 2026-03-31

### Removed
- Removed the string-based `qtty-ffi` JSON serialization entry points:
  - `qtty_quantity_to_json_value` / `qtty_quantity_from_json_value`
  - `qtty_quantity_to_json` / `qtty_quantity_from_json`
  - `qtty_derived_to_json` / `qtty_derived_from_json`
  - `qtty_string_free`

### Changed
- `qtty-ffi` now exposes a POD-only FFI surface for quantity carriers; callers should pass `qtty_quantity_t` and `qtty_derived_quantity_t` directly instead of string payloads.

## [0.4.1] - 2026-03-08

### Added
- Added a new `qtty` example, `all_units`, that surveys the built-in dimensions, scalar families, dimensional arithmetic, and `qtty_vec!` usage in one runnable program.

### Changed
- Restored the workspace and facade crate READMEs as release-facing documentation instead of placeholder pointers to the repository `doc/` tree.
- Updated optional `pyo3` dependencies across `qtty`, `qtty-core`, and `qtty-ffi` from `0.27.2` to `0.28.2`.


## [0.4.0] - 2026-02-26

### Added
- Implemented `Display`, `LowerExp`, and `UpperExp` delegation for `Quantity<U, S>` so standard Rust format annotations (precision, scientific notation) are respected by all unit types.
- Exposed a new FFI function `qtty_quantity_format(qtty_quantity_t, precision, flags, buf, buf_len)` plus format flag constants `QTTY_FMT_DEFAULT`, `QTTY_FMT_LOWER_EXP`, and `QTTY_FMT_UPPER_EXP` for C consumers to format quantities from Rust with the same options as Rust's formatters.
- Added C++ convenience: `qtty::Quantity<UnitTag>::format(int precision, uint32_t flags)` that calls the FFI formatter, and a C++20 `std::formatter` specialization to integrate with `std::format` when available.
- Added comprehensive C++ tests covering streaming `operator<<`, `format()`, and scientific/precision formatting modes.

### Changed
- cbindgen/header generation: `qtty-ffi` build now gracefully skips automatic cbindgen expansion on stable toolchains (nightly required for macro expansion); the shipped `qtty_ffi.h` is updated to include the new formatter API and constants.

### Fixed
- Corrected `Display` implementations to delegate formatting to the inner scalar so `{:.N}`, `{:e}`, and related annotations behave as expected for `Quantity` values.
- Adjusted C++ helpers and tests to match C++ stream precision semantics and to ensure `format()` mirrors specified precision/flags.

## [0.3.1] - 2026-02-24

### Added
- New `qtty` crate feature `alloc` for heap-backed helpers in `no_std` builds. (see #10)
- Integration compile checks for `qtty::qtty_vec!` across `std`, `no_std + alloc`, and pure `no_std` modes. (see #10)
- New integer scalar facade modules `qtty::i8`, `qtty::i16`, and `qtty::i128`, mirroring the unit aliases available in `qtty::i32`. (see #11)
- New `cross-unit-ops` feature in `qtty-core` and `qtty` (enabled by default) to gate generation of direct cross-unit comparison operator impls (`==`, `<`, etc.). (see #15)
- New reduced-mode CI profile (`No Cross-Unit Ops`) plus targeted compile checks validating `eq_unit`/`cmp_unit` and ensuring direct cross-unit operators are disabled when the feature is off. (see #15)
- Documented compile-time benchmark commands (`cargo +nightly -Z timings`) for comparing default and reduced-mode builds. (see #15)
- FFI JSON serialization: New FFI functions for serializing and deserializing `QttyQuantity`/`QttyDerivedQuantity` to/from JSON:
  - `qtty_quantity_to_json_value` / `qtty_quantity_from_json_value`
  - `qtty_quantity_to_json` / `qtty_quantity_from_json`
  - `qtty_derived_to_json` / `qtty_derived_from_json`


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
- **Unitless refactor**: `Unitless` changed from a `pub type Unitless = f64` alias to a proper zero-sized marker type (`pub struct Unitless;`). The `Unit` impl for `Unitless` remains (`RATIO = 1.0`, `Dim = Dimensionless`, `SYMBOL = ""`) while removing the implicit `Unit` implementation for `f64`. API ergonomics preserved: `Quantity<Unitless>` display and From conversions unchanged. Updated docs, tests, and examples accordingly.

### Deprecated
- `define_unit!` is retained for internal use and backward compatibility; new units in `qtty-core` use `#[derive(Unit)]`.
- Specific velocity type aliases (e.g., `MetersPerSecond`, `KilometersPerSecond`) in favor of generic `Velocity<N, D>` type.
- Specific frequency type aliases (e.g., `RadiansPerSecond`, `DegreesPerDay`) in favor of generic `Frequency<N, D>` type.

### Fixed
- `qtty` feature flags now correctly control `qtty-core` defaults (including `no_std` builds).
- Improved type safety and consistency across velocity and frequency unit definitions.


## [0.0.0] - 2025-09-01

- Migration from Siderust
