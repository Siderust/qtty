# qtty-ffi unit registry

## Format

Each row in `units.csv` defines a unit with the format:

```
discriminant,dimension,name,symbol,ratio[,rust_type]
```

- **discriminant**: ABI-stable `u32` identifier (never reuse or change).
- **dimension**: must match a `DimensionId` variant exactly.
- **ratio**: conversion factor to the canonical unit for the dimension (metres for Length, seconds for Time, radians for Angle, grams for Mass, watts for Power, square metres for Area, cubic metres for Volume).
- **rust_type** (optional): fully qualified Rust quantity type for auto-generating `From`/`TryFrom` impls.

## Symbol policy

FFI symbols **must match** the Rust `Unit::SYMBOL` for the corresponding unit marker. If an FFI-specific display symbol is ever needed, it should be added as a separate column rather than overriding the canonical Rust symbol. This ensures that formatting output is consistent whether a caller uses `qtty-core` directly or goes through the FFI adapter.

## Ratio policy

FFI ratios **must match** the Rust `Unit::RATIO` for the corresponding unit marker. When updating a ratio in `qtty-core`, the matching `units.csv` row must be updated in the same change.
