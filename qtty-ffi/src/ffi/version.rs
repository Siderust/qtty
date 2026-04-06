/// Returns the FFI ABI version (major*10000 + minor*100 + patch).
///
/// Current version: 0.5.0 → 500
///
/// # Breaking change (0.5.0)
///
/// `QttyQuantity.unit` and `QttyDerivedQuantity.{numerator,denominator}` are
/// now raw `u32` instead of `UnitId` enums.  This eliminates UB from invalid
/// discriminants constructed by C callers.
#[allow(clippy::erasing_op, clippy::identity_op)]
#[no_mangle]
pub extern "C" fn qtty_ffi_version() -> u32 {
    0 * 10000 + 5 * 100 + 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(qtty_ffi_version(), 500);
    }
}
