# Changelog
All notable changes to this project are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Workspace split into crates: `unit` (facade), `unit-core` (types + units), `unit-derive` (proc-macro).
- Feature flags: `std` (default) and optional `serde` for `Quantity<U>`.
- `no_std` support in `unit-core` (uses `libm` for floating-point math not in `core`).
- Predefined unit modules under `unit-core::units` (angular, time, length, mass, power, velocity, frequency, unitless).

### Changed
- Documentation rewrite for docs.rs (crate docs, READMEs, examples).

### Deprecated
- `define_unit!` is retained for internal use and backward compatibility; new units in `unit-core` use `#[derive(Unit)]`.

### Fixed
- `unit` feature flags now correctly control `unit-core` defaults (including `no_std` builds).
