# qtty-ffi Architecture

`qtty-ffi` turns the `qtty` quantity system into a stable C ABI that can be safely consumed from C/C++ (or any language with a C FFI) while still exposing Rust helpers for downstream crates. The crate guarantees ABI stability, prohibits panics across the FFI boundary, and keeps conversion logic centralized and data-driven.

---

## Goals and Constraints

- **Stable ABI** – status codes, enum discriminants, and struct layouts are fixed once released (`qtty-ffi/src/lib.rs:50`).
- **Thread safe / panic safe** – functions perform pointer validation, avoid mutable globals, and catch unwinds at the boundary (`qtty-ffi/src/ffi.rs:32`).
- **Single source of truth for units** – unit metadata is maintained in a CSV file that drives both Rust code generation and the public C header.
- **Composable Rust helpers** – downstream crates can round-trip between strongly typed `qtty` units and the flat FFI representation via macros and explicit helper functions (`qtty-ffi/src/macros.rs:62` and `qtty-ffi/src/helpers.rs:54`).

---

## Build-Time Pipeline

### 1. Unit Catalog (`units.csv`)

All supported unit IDs live in `units.csv`, which documents the discriminant scheme, canonical-unit ratios, and coverage per dimension (`qtty-ffi/units.csv:1`). The discriminant format (`DSSCC`) encodes dimension, system, and counter, making it easy to spot gaps and ensure ABI stability.

### 2. Code Generation (`build.rs`)

During compilation the build script parses the CSV, emits Rust source snippets into `$OUT_DIR`, and re-runs whenever the CSV changes (`qtty-ffi/build.rs:5`). Generated artifacts:

- `unit_id_enum.rs` – the ABI-stable `UnitId` enum with explicit discriminants consumed by the types module via `include!` (`qtty-ffi/src/types.rs:75`).
- `unit_names.rs` / `unit_names_cstr.rs` – lookup tables for Rust strings and C strings (`qtty-ffi/src/types.rs:82`).
- `unit_from_u32.rs` – lossless discriminant parsing (`qtty-ffi/src/types.rs:92`).
- `unit_registry.rs` – `UnitMeta` match arms used by the registry (`qtty-ffi/src/registry.rs:55`).

Because the generated files are `include!`-d inside the modules, they become part of the crate without bespoke `build.rs` dependencies or runtime allocation. The generator also logs the number of units processed so the build output makes it obvious when the data set changes (`qtty-ffi/build.rs:22`).

### 3. C Header Emission (cbindgen)

After generating the Rust-side artifacts, the build script drives `cbindgen` to render `include/qtty_ffi.h` from the Rust sources (`qtty-ffi/build.rs:169`). The `cbindgen.toml` configuration describes how to rename enums/structs, adds documentation into the header, and enforces include guards plus C++ compatibility (`qtty-ffi/cbindgen.toml:4`).

---

## Runtime Module Layout

### Crate Root (`src/lib.rs`)

The crate root wires all submodules, re-exports the public FFI surface, helper functions, and ABI constants (`qtty-ffi/src/lib.rs:96`). It also places `#![deny(missing_docs)]` and `#![deny(unsafe_op_in_unsafe_fn)]` on the crate to keep the surface thoroughly documented and to enforce disciplined unsafe usage.

### Types Layer (`src/types.rs`)

This module defines every ABI-stable type:

- Status codes and their semantic meanings (`qtty-ffi/src/types.rs:19`).
- `DimensionId`, with explicit discriminants per physical dimension (`qtty-ffi/src/types.rs:48`).
- `UnitId`, populated from generated code (`qtty-ffi/src/types.rs:75`).
- `QttyQuantity`, the `#[repr(C)]` payload used across the ABI (`qtty-ffi/src/types.rs:129`).

`UnitId` also exposes compile-time lookup helpers (`name`, `name_cstr`, `from_u32`) that are generated off of the CSV data, so lookup tables never fall out of sync with the enum values.

### Registry (`src/registry.rs`)

The registry is a Rust-only service that maps `UnitId` values to `UnitMeta` instances and performs conversions. Each `UnitMeta` stores the `DimensionId`, a scale factor relative to the dimension’s canonical unit, and a friendly name (`qtty-ffi/src/registry.rs:32`). The generated `unit_registry.rs` provides an exhaustive match, so invalid unit IDs return `None` without allocations (`qtty-ffi/src/registry.rs:55`). Conversion uses a canonicalization formula (`qtty-ffi/src/registry.rs:102`) so any pair of compatible units can be converted through the canonical unit, avoiding a combinatorial explosion of conversion ratios.

### FFI Boundary (`src/ffi.rs`)

This module contains the `#[no_mangle] pub extern "C"` functions that form the ABI. Every function follows the same pattern:

1. Wrap the body in `catch_panic!` to prevent unwinding across FFI (`qtty-ffi/src/ffi.rs:32`).
2. Validate pointers before dereferencing (`qtty-ffi/src/ffi.rs:82`).
3. Delegate to the registry or types module for the heavy lifting.
4. Return a status code instead of propagating errors or panics (`qtty-ffi/src/ffi.rs:118` and `qtty-ffi/src/ffi.rs:197`).

The result is a small, predictable ABI surface: unit validation/inspection APIs, quantity creation, value/quantity conversion, unit name lookup, and an ABI version query.

### Helper Conversions (`src/macros.rs`, `src/helpers.rs`)

Rust consumers often need to convert their strongly typed `qtty` units to the flat `QttyQuantity` representation (and back). The `impl_unit_ffi!` macro generates the glue by implementing `From<T>` and `TryFrom<QttyQuantity>` for each quantity type, invoking the registry for cross-unit conversions when necessary (`qtty-ffi/src/macros.rs:62`). The `helpers` module invokes the macro for every built-in `qtty` unit and exposes function-style helpers (e.g., `meters_into_ffi`, `try_into_hours`) for API consumers who prefer functions over trait conversions (`qtty-ffi/src/helpers.rs:77`).

---

## Data Flow: Value Conversion

```text
Callers (C or Rust) ──► FFI Layer ──► Registry ──► Output
        │                  │             │
        │                  │             ├─ Validates unit metadata via generated map
        │                  ├─ Validates pointers & catches panics
        ├─ Supplies UnitId/QttyQuantity derived from CSV data
```

1. A caller constructs or receives a `QttyQuantity`. In Rust this likely came from `impl_unit_ffi!` machinery; in C it came from `qtty_quantity_make`.
2. Conversion requests flow through the `qtty_quantity_convert` or `qtty_quantity_convert_value` FFI call (`qtty-ffi/src/ffi.rs:197`).
3. The FFI layer checks that the output pointer is not null, then calls `registry::convert_value`, which fetches metadata for both units and enforces same-dimension conversion (`qtty-ffi/src/registry.rs:102`).
4. Successful conversions write the new value/unit into caller-owned memory; failures return status codes so the caller can branch without accessing uninitialized memory.

Because all metadata is generated at build time and embedded in the binary, the runtime path contains no heap allocations and only simple math on f64s.

---

## Error Handling, Safety, and ABI Guarantees

- **Pointer discipline** – every FFI function returns `QTTY_ERR_NULL_OUT` when given a null output pointer, preventing UB (`qtty-ffi/src/ffi.rs:82` and `qtty-ffi/src/ffi.rs:158`).
- **Unit validation** – conversions consult the registry and return `QTTY_ERR_UNKNOWN_UNIT` for unknown IDs before touching the destination pointer (`qtty-ffi/src/ffi.rs:197`).
- **Dimension checks** – incompatible units short-circuit with `QTTY_ERR_INCOMPATIBLE_DIM`, mirroring the registry error codes (`qtty-ffi/src/registry.rs:106`).
- **Layout guarantees** – layout tests assert that `QttyQuantity`, `UnitId`, and `DimensionId` sizes and alignment match the ABI contract (`qtty-ffi/src/lib.rs:131` and `qtty-ffi/tests/integration_tests.rs:201`).
- **Panic containment** – the `catch_panic!` macro ensures panics become deterministic error codes (`qtty-ffi/src/ffi.rs:32`).

---

## Testing and Verification

- **Crate-level tests** focus on layout guarantees and known conversions, ensuring the most critical invariants remain intact (`qtty-ffi/src/lib.rs:131`).
- **Module unit tests** cover registry conversions, helper round-trips, and macro-generated behavior (`qtty-ffi/src/registry.rs:151` and `qtty-ffi/src/helpers.rs:189`).
- **Integration tests** exercise the public ABI end-to-end: validation, conversions, error paths, unit names, helper APIs, and propagation of special float values (`qtty-ffi/tests/integration_tests.rs:18`).

This layered strategy catches regressions both at the structural level (layout, discriminants) and at the behavior level (conversion math, error codes).

---

## Extensibility

1. **Add a unit** by appending a row to `units.csv`, choosing an unused discriminant in the correct range, and rebuilding. The build script regenerates every dependent artifact automatically.
2. **Expose new dimensions** by extending `DimensionId`, adding new canonical unit definitions, and feeding the build script rows that use the new dimension name (`qtty-ffi/src/types.rs:48`).
3. **Add helper conversions** by calling `impl_unit_ffi!` for the new `qtty` type or using the macro inside your own crate (`qtty-ffi/src/macros.rs:62`).
4. **Extend the C ABI** by adding new `extern "C"` functions to `src/ffi.rs` and letting `cbindgen` include them in the generated header. Keep ABI stability by preserving existing signatures and status codes.

---

## Files of Interest

- `src/lib.rs` – crate root and exports (`qtty-ffi/src/lib.rs:96`).
- `src/types.rs` – ABI types and generated `UnitId` (`qtty-ffi/src/types.rs:19`).
- `src/registry.rs` – conversion engine (`qtty-ffi/src/registry.rs:32`).
- `src/ffi.rs` – public C ABI (`qtty-ffi/src/ffi.rs:46`).
- `src/helpers.rs` & `src/macros.rs` – Rust helper APIs (`qtty-ffi/src/helpers.rs:54`, `qtty-ffi/src/macros.rs:62`).
- `units.csv` – authoritative unit table (`qtty-ffi/units.csv:1`).
- `build.rs` – CSV-driven code generation and header emission (`qtty-ffi/build.rs:5`).
- `cbindgen.toml` – header customization (`qtty-ffi/cbindgen.toml:4`).
- `tests/integration_tests.rs` – ABI-level regression tests (`qtty-ffi/tests/integration_tests.rs:18`).

These files collectively describe the software architecture and are the primary touch points when extending or auditing the crate.
