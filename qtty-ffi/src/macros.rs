//! Macros for implementing FFI conversions for qtty unit types.
//!
//! This module provides macros that make it easy to implement `From` and `TryFrom`
//! conversions between `qtty` unit types and [`QttyQuantity`].
//!
//! # Example
//!
//! The macro is intended to be used in crates that own the types being converted.
//! Since `qtty-ffi` already implements these for the standard `qtty` types, you
//! typically only need this for custom wrapper types.
//!
//! ```rust,ignore
//! use qtty_ffi::{impl_unit_ffi, QttyQuantity, UnitId};
//! use qtty::length::Meters;
//!
//! // This would work in your own crate that defines MyMeters:
//! impl_unit_ffi!(MyMeters, UnitId::Meter);
//!
//! // Then you can convert between MyMeters and QttyQuantity
//! let meters = MyMeters::new(100.0);
//! let quantity: QttyQuantity = meters.into();
//! ```
/// Implements `From<$qty_type>` for `QttyQuantity` and `TryFrom<QttyQuantity>` for `$qty_type`.
///
/// This macro generates bidirectional conversion implementations between a specific
/// `qtty` quantity type (like `Meters`, `Seconds`, etc.) and the FFI-safe [`QttyQuantity`] type.
///
/// # Arguments
///
/// * `$qty_type` - The `qtty` quantity type (e.g., `qtty::length::Meters`)
/// * `$unit_id` - The corresponding [`UnitId`] variant (e.g., `UnitId::Meter`)
///
/// # Generated Implementations
///
/// * `impl From<$qty_type> for QttyQuantity` - Converts the typed quantity to FFI format
/// * `impl TryFrom<QttyQuantity> for $qty_type` - Converts FFI format back to typed quantity,
///   performing unit conversion if the FFI quantity is in a compatible unit
///
/// # Example
///
/// The macro is used internally by `qtty-ffi` to implement conversions for standard types.
/// For your own types, usage looks like:
///
/// ```rust,ignore
/// use qtty_ffi::{impl_unit_ffi, QttyQuantity, UnitId};
///
/// // Your custom type (defined in your crate)
/// struct MySeconds(f64);
/// impl MySeconds {
///     fn new(v: f64) -> Self { Self(v) }
///     fn value(&self) -> f64 { self.0 }
/// }
///
/// impl_unit_ffi!(MySeconds, UnitId::Second);
/// ```
#[macro_export]
macro_rules! impl_unit_ffi {
    ($qty_type:ty, $unit_id:expr) => {
        impl From<$qty_type> for $crate::QttyQuantity {
            #[inline]
            fn from(qty: $qty_type) -> Self {
                $crate::QttyQuantity::new(qty.value(), $unit_id)
            }
        }

        impl core::convert::TryFrom<$crate::QttyQuantity> for $qty_type {
            type Error = i32;

            #[inline]
            fn try_from(qty: $crate::QttyQuantity) -> Result<Self, Self::Error> {
                // If already the right unit, just wrap
                if qty.unit == $unit_id {
                    return Ok(<$qty_type>::new(qty.value));
                }

                // Otherwise, try to convert
                let converted = $crate::registry::convert_value(qty.value, qty.unit, $unit_id)?;
                Ok(<$qty_type>::new(converted))
            }
        }
    };
}
