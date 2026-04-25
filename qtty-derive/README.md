# qtty-derive

Procedural macro crate that implements `#[derive(Unit)]` for the `qtty`
workspace.

Most downstream users should depend on [`qtty`](https://crates.io/crates/qtty)
and write `#[derive(qtty::Unit)]`, not depend on `qtty-derive` directly.

Example:

```rust
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, qtty::Unit)]
#[unit(crate = qtty, symbol = "smoot", dimension = qtty::Length, ratio = 1.7018)]
pub struct Smoot;
```

Repository docs:

- Workspace overview: [`../../doc/users/rust-workspace.md`](../../doc/users/rust-workspace.md)
- Repository layout: [`../../doc/architecture/repository-layout.md`](../../doc/architecture/repository-layout.md)
