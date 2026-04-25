# qtty-core

Low-level type system and built-in unit catalogs behind `qtty`.

Most applications should depend on [`qtty`](https://crates.io/crates/qtty)
instead. Reach for `qtty-core` when you need the primitive building blocks
directly:

- `Unit` marker types and `Quantity<U, S>`
- structural derived units such as `Per<N, D>` and `Prod<A, B>`
- built-in unit catalogs under `qtty_core::{angular, length, time, ...}`
- `no_std` support and optional `serde`, `pyo3`, `diesel`, and `tiberius`
  integrations

Install:

```toml
[dependencies]
qtty-core = "0.6.0"
```

Minimal `no_std`:

```toml
[dependencies]
qtty-core = { version = "0.6.0", default-features = false }
```

Repository docs:

- Workspace overview: [`../../doc/users/rust-workspace.md`](../../doc/users/rust-workspace.md)
- Repository layout: [`../../doc/architecture/repository-layout.md`](../../doc/architecture/repository-layout.md)
