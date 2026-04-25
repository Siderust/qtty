# qtty-ffi

C-compatible ABI bridge for [`qtty`](https://crates.io/crates/qtty).

`qtty-ffi` exposes stable carrier structs, unit identifiers, status codes, and
conversion/formatting functions so C and C++ consumers can use `qtty`'s unit
registry without reimplementing conversion logic.

## Highlights

- `QttyQuantity` and `QttyDerivedQuantity` POD carriers
- stable `UnitId` and `DimensionId` discriminants
- `extern "C"` entry points for construction, conversion, inspection, and
  formatting
- generated header at `include/qtty_ffi.h`
- optional Rust-side `qtty_serde` and `pyo3` helpers

## Install

```toml
[dependencies]
qtty-ffi = "0.6.1"
```

## C example

```c
#include "qtty_ffi.h"

qtty_quantity_t meters;
qtty_quantity_make(1000.0, UNIT_ID_METER, &meters);

qtty_quantity_t kilometers;
if (qtty_quantity_convert(meters, UNIT_ID_KILOMETER, &kilometers) == QTTY_OK) {
    /* kilometers.value == 1.0 */
}
```

## Rust example

```rust
use qtty::Meter;
use qtty_ffi::{QttyQuantity, UnitId};

let qty: QttyQuantity = Meter::new(12.5).into();
assert_eq!(qty.unit, UnitId::Meter as u32);
```

## Related crates

- `qtty`: user-facing Rust facade
- `qtty-core`: low-level unit type system
- `qtty-derive`: proc-macro for custom unit markers
