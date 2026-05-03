// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

/// Returns the FFI ABI version (major*10000 + minor*100 + patch).
///
/// Current version: 0.6.1 → 601
///
/// The 0.6.x ABI uses raw `u32` unit identifiers in `QttyQuantity` and
/// `QttyDerivedQuantity` so C callers cannot construct invalid Rust enum
/// discriminants across the FFI boundary.
#[allow(clippy::erasing_op, clippy::identity_op)]
#[no_mangle]
pub extern "C" fn qtty_ffi_version() -> u32 {
    0 * 10000 + 6 * 100 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(qtty_ffi_version(), 601);
    }
}
