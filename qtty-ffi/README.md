# qtty-ffi crate

This crate's maintained documentation currently lives with the crate sources:

- Public ABI overview and examples: [`src/lib.rs`](src/lib.rs)
- Exported C header: [`include/qtty_ffi.h`](include/qtty_ffi.h)
- Registry source data: [`units.csv`](units.csv)
- Registry format notes: [`units.csv.md`](units.csv.md)

The exported FFI surface is POD-oriented: callers exchange raw `qtty_quantity_t`
and `qtty_derived_quantity_t` values directly instead of JSON/string payloads.

This README remains the crate metadata entrypoint.
