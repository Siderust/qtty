# `unit-derive`

Procedural macro implementation used by `unit-core`.

This crate is primarily an internal implementation detail: the `Unit` derive expands in terms of `crate::Unit` and
`crate::Quantity`, which means it is intended to be used inside `unit-core` (or crates with the same crate-root API).

Most users should depend on `unit` instead.

## License

AGPL-3.0 (see `../LICENSE`).
