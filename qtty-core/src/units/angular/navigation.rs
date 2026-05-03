// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

/// Gradian (also called gon; `1/400` of a full turn = `0.9` degree).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "gon", dimension = Angular, ratio = 0.9)]
pub struct Gradian;
/// Type alias shorthand for [`Gradian`].
pub type Gon = Gradian;
/// Convenience alias for a gradian quantity.
pub type Gradians = Quantity<Gon>;
/// One gradian.
pub const GON: Gradians = Gradians::new(1.0);

crate::impl_unit_from_conversions_between!(
    Degree, Radian, Milliradian, Turn;
    Gradian
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    Degree, Radian, Milliradian, Turn;
    Gradian
);

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────
#[macro_export]
#[doc(hidden)]
macro_rules! angular_navigation_units {
    ($cb:path) => {
        $cb!(Gradian,);
    };
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use proptest::prelude::*;

    #[test]
    fn gradian_to_degrees() {
        let gradian = Gradians::new(100.0);
        let degrees: Degrees = gradian.to();
        assert_abs_diff_eq!(degrees.value(), 90.0, epsilon = 1e-12);
    }

    #[test]
    fn full_turn_to_gradians() {
        let turn = Turns::new(1.0);
        let gradians: Gradians = turn.to();
        assert_abs_diff_eq!(gradians.value(), 400.0, epsilon = 1e-12);
    }

    proptest! {
        #[test]
        fn gradian_degree_roundtrip(v in -1.0e9_f64..1.0e9_f64) {
            let gradians = Gradians::new(v);
            let roundtrip: Gradians = gradians.to::<Degree>().to();
            prop_assert!((roundtrip.value() - v).abs() <= v.abs().max(1.0) * 1e-12);
        }
    }
}
