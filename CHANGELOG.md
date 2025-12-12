# Changelog
All notable changes to this project are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Procedural derive macro for defining units via `#[derive(Unit)]` and `#[unit(...)]`.
- Workspace split into crates: `units-core` (types), `units-derive` (proc-macro), `unit` (re-exports).
- Feature flags: `std` (default) and optional `serde` for `Quantity<U>`.
- `no_std` support (uses `libm` for math).
- Unit definitions organized under `units-core/src/units/` (angular, length, time, mass, power, velocity, frequency, unitless).

### Changed
- Refactored from a single-crate declarative-macro setup to a multi-crate workspace using procedural macros.
- Migrated ~40 units from `define_unit!` to the `#[derive(Unit)]` workflow.
- Moved unit modules into `units-core/src/units/`.
- Improved proc-macro hygiene (explicit paths for better IDE support and diagnostics).

### Deprecated
- `define_unit!` remains available for now, but is deprecated in favor of `#[derive(Unit)]`.
- Planned removal in a future major version (kept temporarily for migration compatibility).

### Fixed
- Fixed doctests to use correct crate paths.
- Moved unit definitions into `units-core` to satisfy Rust orphan rules.
- Reorganized tests into appropriate crates (189 unit tests, 29 integration tests).

### Security

## [0.0.0] - 2025-09-01

### Added
- Migration from Siderust.
