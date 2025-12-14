# `qtty-ffi`

[![Crates.io](https://img.shields.io/crates/v/qtty-ffi.svg)](https://crates.io/crates/qtty-ffi)
[![Docs.rs](https://docs.rs/qtty-ffi/badge.svg)](https://docs.rs/qtty-ffi)

C-compatible FFI bindings for [`qtty`](../qtty/) physical quantities and unit conversions.

## What this crate provides

- **Stable ABI**: `#[repr(C)]`/`#[repr(u32)]` types with explicit discriminants for every unit.
- **Generated header**: `build.rs` runs `cbindgen`, producing `include/qtty_ffi.h` for C/C++ consumers.
- **CSV-driven registry**: every unit lives in [`units.csv`](units.csv); edit one file and the Rust code plus header stay in sync (details in [`units.csv.md`](units.csv.md)).
- **Rust helpers**: conversion traits, helper fns, and the `impl_unit_ffi!` macro to adapt your own quantity wrappers.

## Building & header generation

```bash
# Build cdylib/staticlib/rlib targets
cargo build -p qtty-ffi

# Generated header checked into git:
# qtty-ffi/include/qtty_ffi.h
```

`build.rs` re-runs whenever `units.csv`, `src/`, or `cbindgen.toml` change. It emits:

- `UnitId` enum variants/discriminants and the lookup tables behind `qtty_unit_*` APIs.
- The runtime registry that powers conversions.
- The C header described above.

### Unit definitions

`units.csv` rows look like:

```csv
discriminant,dimension,name,symbol,ratio
10011,Length,Meter,m,1.0
21000,Time,Minute,min,60.0
30001,Angle,Degree,deg,0.017453292519943295
```

- **Discriminants** follow a DSSCC scheme (`D = dimension`, `SS = system/category`, `CC = counter`).
- **Dimension** must be one of the `DimensionId` variants (`Length`, `Time`, `Angle`, `Mass`, `Power`).
- **Ratio** is the scale factor relative to the canonical unit (meters, seconds, radians, grams, watts).

The CSV becomes the ABI contract: review diffs to see exactly which units were added or changed.

## ABI surface

### Types

```c
typedef struct {
    double value;
    UnitId unit;
} qtty_quantity_t;

typedef enum {
    DimensionId_Length = 1,
    DimensionId_Time = 2,
    DimensionId_Angle = 3,
    DimensionId_Mass = 4,
    DimensionId_Power = 5,
} DimensionId;
```

`UnitId` is `#[repr(u32)]` and contains every row from `units.csv`. Layouts (16-byte `qtty_quantity_t`, 4-byte enums) are part of the ABI contract.

### Status codes

| Constant | Value | Meaning |
| --- | --- | --- |
| `QTTY_OK` | 0 | Success |
| `QTTY_ERR_UNKNOWN_UNIT` | -1 | Invalid unit id |
| `QTTY_ERR_INCOMPATIBLE_DIM` | -2 | Dimension mismatch |
| `QTTY_ERR_NULL_OUT` | -3 | Null output pointer |
| `QTTY_ERR_INVALID_VALUE` | -4 | Reserved for future validation |

### Functions

```c
bool qtty_unit_is_valid(UnitId unit);
int32_t qtty_unit_dimension(UnitId unit, DimensionId* out);
int32_t qtty_units_compatible(UnitId a, UnitId b, bool* out);
const char* qtty_unit_name(UnitId unit);

int32_t qtty_quantity_make(double value, UnitId unit, qtty_quantity_t* out);
int32_t qtty_quantity_convert(qtty_quantity_t src, UnitId dst, qtty_quantity_t* out);
int32_t qtty_quantity_convert_value(double value, UnitId src, UnitId dst, double* out_value);

uint32_t qtty_ffi_version(void); // currently 1
```

## Usage from C/C++

```c
#include "qtty_ffi.h"
#include <stdio.h>

int main(void) {
    qtty_quantity_t meters;
    if (qtty_quantity_make(1000.0, UnitId_Meter, &meters) != QTTY_OK) {
        return 1;
    }

    qtty_quantity_t kilometers;
    if (qtty_quantity_convert(meters, UnitId_Kilometer, &kilometers) == QTTY_OK) {
        printf("1000 meters = %.2f kilometers\n", kilometers.value);
    }

    return 0;
}
```

## Usage from Rust

```rust
use qtty::length::{Kilometers, Meters};
use qtty_ffi::{impl_unit_ffi, QttyQuantity, UnitId};

// Built-in conversions via From/TryFrom
let ffi: QttyQuantity = Meters::new(1_000.0).into();
let km: Kilometers = ffi.try_into().unwrap();
assert!((km.value() - 1.0).abs() < 1e-12);

// Custom wrapper (must expose new()/value())
struct MyMeters(f64);
impl MyMeters {
    fn new(v: f64) -> Self { Self(v) }
    fn value(&self) -> f64 { self.0 }
}

impl_unit_ffi!(MyMeters, UnitId::Meter);
let ffi_again: QttyQuantity = MyMeters::new(42.0).into();
```

Helper functions like `meters_into_ffi`, `try_into_hours`, etc. are available for ergonomic wrappers.

## ABI stability & thread safety

- Existing `UnitId`/`DimensionId` discriminants, status codes, type layouts, and exported signatures will not change.
- New variants/functions may be added in minor versions (callers should handle unknown ids defensively).
- All functions are thread-safe and never panic; errors flow through the status codes above.
- Special `f64` values (`NaN`, `±INF`) are passed through untouched.

## License

Same license as the parent workspace (AGPL-3.0). See `../LICENSE`.
