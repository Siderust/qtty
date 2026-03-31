//! Extern "C" API for FFI consumers.
//!
//! This module tree exposes `#[no_mangle] pub extern "C"` functions that form the stable C ABI
//! for `qtty-ffi`. These functions can be called from C/C++ code or any language with C FFI
//! support.
//!
//! # Safety
//!
//! All functions in this module tree:
//! - Never panic across FFI boundaries (all panics are caught and converted to `InternalPanic`)
//! - Validate all input pointers before use
//! - Accept unit and dimension IDs as raw `uint32_t` values and validate them before dispatch
//! - Return [`crate::QttyStatus`] to indicate success or failure
//!
//! # ABI conventions (vNext)
//!
//! - Every fallible exported function returns `QttyStatus` and writes results via out-parameters.
//! - Unit and dimension inputs are raw `u32` IDs validated before dispatch; named constants
//!   in the generated header provide the canonical values for C callers.
//! - `InternalPanic` is reserved exclusively for caught Rust panics and is never used
//!   for domain errors like `UnknownUnit`.

mod derived;
mod quantity;
mod shared;
mod unit;
mod version;

pub use derived::{qtty_derived_convert, qtty_derived_make};
pub use quantity::{
    qtty_quantity_convert, qtty_quantity_convert_value, qtty_quantity_format, qtty_quantity_make,
};
pub use unit::{qtty_unit_dimension, qtty_unit_is_valid, qtty_unit_name, qtty_units_compatible};
pub use version::qtty_ffi_version;
