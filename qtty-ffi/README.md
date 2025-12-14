# qtty-ffi

C-compatible FFI bindings for [qtty](../qtty/) physical quantities and unit conversions.

## Overview

`qtty-ffi` provides a stable C ABI for the `qtty` library, enabling:

1. **C/C++ interoperability**: Call into Rust to construct and convert quantities using `qtty`'s conversion logic.
2. **Downstream Rust FFI**: Expose your own structs/functions over FFI using shared `qtty-ffi` types.

## Building

```bash
# Build as cdylib (shared library) and staticlib
cargo build -p qtty-ffi

# The header file is generated at:
# qtty-ffi/include/qtty_ffi.h
```

### Unit Definitions

All FFI units are defined in [`units.csv`](units.csv), a simple CSV file with the format:

```csv
discriminant,dimension,name,symbol,ratio
10011,Length,Meter,m,1.0
10014,Length,Kilometer,km,1000.0
```

**Discriminant encoding (DSSCC):**
- **D** (1 digit): Dimension (1=Length, 2=Time, 3=Angle, 4=Mass, 5=Power)
- **SS** (2 digits): System/Category (00=SI, 10=Astronomical/Common, 20=Imperial/Calendar, etc.)
- **CC** (2 digits): Counter within system (00-99)

Example: `10011` = Meter (Length/SI/#11), `21000` = Minute (Time/Common/#0)

**Benefits:**
- ✅ Single source of truth for all units
- ✅ Easy to add/modify units (just edit CSV)
- ✅ Simple build script (~200 lines vs 400+ with AST parsing)
- ✅ Git diffs show exactly what changed
- ✅ No complex dependencies (no syn/quote)
- ✅ Clear ABI contract visible in one file

See [`units.csv.md`](units.csv.md) for full documentation on the CSV format and how to add new units.

## ABI Types

### `QttyQuantity` (C: `qtty_quantity_t`)

A POD quantity carrier type:

```c
typedef struct {
    double value;
    UnitId unit;
} qtty_quantity_t;
```

### `UnitId`

Unit identifier enum with explicit discriminants:

| Unit | Discriminant | Dimension |
|------|--------------|-----------|
| `Meter` | 100 | Length |
| `Kilometer` | 101 | Length |
| `Second` | 200 | Time |
| `Minute` | 201 | Time |
| `Hour` | 202 | Time |
| `Day` | 203 | Time |
| `Radian` | 300 | Angle |
| `Degree` | 301 | Angle |

### `DimensionId`

Dimension identifier enum:

| Dimension | Discriminant |
|-----------|--------------|
| `Length` | 1 |
| `Time` | 2 |
| `Angle` | 3 |

### Status Codes

| Code | Value | Description |
|------|-------|-------------|
| `QTTY_OK` | 0 | Success |
| `QTTY_ERR_UNKNOWN_UNIT` | -1 | Invalid unit ID |
| `QTTY_ERR_INCOMPATIBLE_DIM` | -2 | Dimension mismatch |
| `QTTY_ERR_NULL_OUT` | -3 | Null output pointer |
| `QTTY_ERR_INVALID_VALUE` | -4 | Invalid value (reserved) |

## C API Functions

### Unit Validation

```c
// Check if a unit ID is valid
bool qtty_unit_is_valid(UnitId unit);

// Get the dimension of a unit
int32_t qtty_unit_dimension(UnitId unit, DimensionId* out);

// Check if two units are compatible (same dimension)
int32_t qtty_units_compatible(UnitId a, UnitId b, bool* out);

// Get unit name as NUL-terminated string
const char* qtty_unit_name(UnitId unit);
```

### Quantity Operations

```c
// Create a quantity
int32_t qtty_quantity_make(double value, UnitId unit, qtty_quantity_t* out);

// Convert a quantity to a different unit
int32_t qtty_quantity_convert(qtty_quantity_t src, UnitId dst_unit, qtty_quantity_t* out);

// Convert just the value (without wrapping in struct)
int32_t qtty_quantity_convert_value(double value, UnitId src_unit, UnitId dst_unit, double* out_value);
```

### Version

```c
// Get FFI ABI version (currently 1)
uint32_t qtty_ffi_version(void);
```

## Usage in C/C++

Include the generated header and link against the library:

```c
#include "qtty_ffi.h"
#include <stdio.h>

int main() {
    qtty_quantity_t meters, kilometers;
    
    // Create a quantity: 1000 meters
    int32_t status = qtty_quantity_make(1000.0, UNIT_ID_METER, &meters);
    if (status != QTTY_OK) {
        fprintf(stderr, "Failed to create quantity\n");
        return 1;
    }
    
    // Convert to kilometers
    status = qtty_quantity_convert(meters, UNIT_ID_KILOMETER, &kilometers);
    if (status == QTTY_OK) {
        printf("1000 meters = %.2f kilometers\n", kilometers.value);
    } else if (status == QTTY_ERR_INCOMPATIBLE_DIM) {
        fprintf(stderr, "Cannot convert between different dimensions\n");
    }
    
    return 0;
}
```

## Usage in Downstream Rust Crates

### Using the Helper Traits

The crate provides `From` and `TryFrom` implementations for converting between `qtty` types and `QttyQuantity`:

```rust
use qtty::length::{Meters, Kilometers};
use qtty_ffi::{QttyQuantity, UnitId};

// Convert Rust type to FFI
let meters = Meters::new(1000.0);
let ffi_qty: QttyQuantity = meters.into();

// Convert FFI back to Rust type (with automatic unit conversion)
let km: Kilometers = ffi_qty.try_into().unwrap();
assert!((km.value() - 1.0).abs() < 1e-12);
```

### Using the Macro

For custom types, use the `impl_unit_ffi!` macro:

```rust
use qtty_ffi::{impl_unit_ffi, QttyQuantity, UnitId};

// Your custom quantity type (must have new() and value() methods)
struct MyMeters(f64);
impl MyMeters {
    fn new(v: f64) -> Self { Self(v) }
    fn value(&self) -> f64 { self.0 }
}

// Implement FFI conversions
impl_unit_ffi!(MyMeters, UnitId::Meter);

// Now you can convert
let m = MyMeters::new(100.0);
let ffi: QttyQuantity = m.into();
```

### Using the Helper Functions

Explicit functions are also available:

```rust
use qtty::length::Meters;
use qtty_ffi::{meters_into_ffi, try_into_kilometers};

let m = Meters::new(1000.0);
let ffi = meters_into_ffi(m);
let km = try_into_kilometers(ffi).unwrap();
```

### Exposing Your Own FFI

When building your own FFI layer on top of `qtty-ffi`:

```rust
use qtty_ffi::{QttyQuantity, UnitId, QTTY_OK, QTTY_ERR_NULL_OUT};

#[no_mangle]
pub extern "C" fn my_crate_do_something(
    input: QttyQuantity,
    out: *mut QttyQuantity
) -> i32 {
    if out.is_null() {
        return QTTY_ERR_NULL_OUT;
    }
    
    // Process the quantity...
    let result = QttyQuantity::new(input.value * 2.0, input.unit);
    
    unsafe { *out = result; }
    QTTY_OK
}
```

## ABI Stability Contract

The following are guaranteed stable:

- All discriminant values in `UnitId` and `DimensionId`
- Memory layout of `QttyQuantity` (16 bytes: f64 + u32 + padding)
- All status code values
- All exported function signatures

New items may be added in future versions:
- New unit variants (with new discriminant values)
- New dimension variants (with new discriminant values)
- New functions

## Thread Safety

All functions are thread-safe. The library contains no global mutable state.

## Error Handling

- Functions never panic across FFI boundaries
- All errors are returned as status codes
- Special float values (NaN, ±Inf) propagate through conversions
- Null output pointers are always checked before use

## License

Same license as the parent `qtty` workspace (AGPL-3.0).
