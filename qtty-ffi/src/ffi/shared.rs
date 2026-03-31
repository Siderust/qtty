use crate::types::{QttyStatus, UnitId};

/// Catches any panic and returns `InternalPanic` instead of unwinding across FFI.
/// Use this for functions that return `QttyStatus`.
macro_rules! catch_panic {
    ($body:expr) => {{
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $body)) {
            Ok(result) => result,
            Err(_) => QttyStatus::InternalPanic,
        }
    }};
}

/// Catches any panic and returns the given fallback for non-QttyStatus return types.
macro_rules! catch_panic_or {
    ($fallback:expr, $body:expr) => {{
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $body)) {
            Ok(result) => result,
            Err(_) => $fallback,
        }
    }};
}

pub(crate) use catch_panic;
pub(crate) use catch_panic_or;

/// Decode a raw `u32` to `UnitId`, returning `UnknownUnit` if invalid.
#[inline]
pub(crate) fn decode_unit(raw: u32) -> Result<UnitId, QttyStatus> {
    UnitId::from_u32(raw).ok_or(QttyStatus::UnknownUnit)
}

/// Validate an output pointer and convert it to `NonNull`.
#[inline]
pub(crate) fn out_ptr<T>(ptr: *mut T) -> Result<std::ptr::NonNull<T>, QttyStatus> {
    std::ptr::NonNull::new(ptr).ok_or(QttyStatus::NullOut)
}
