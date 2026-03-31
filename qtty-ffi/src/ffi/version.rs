/// Returns the FFI ABI version (major*10000 + minor*100 + patch).
///
/// Current version: 0.4.1 → 401
#[allow(clippy::erasing_op, clippy::identity_op)]
#[no_mangle]
pub extern "C" fn qtty_ffi_version() -> u32 {
    0 * 10000 + 4 * 100 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(qtty_ffi_version(), 401);
    }
}
