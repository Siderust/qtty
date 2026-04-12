// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Force units.
//!
//! The canonical scaling unit for this dimension is the **newton** (`Newton::RATIO == 1.0`).
//! All other force units are expressed as exact ratios to newtons.
//!
//! This module provides:
//!
//! - **SI newton** and the full SI prefix ladder.
//! - **CGS dyne** (feature: `fundamental-physics`).
//! - **Pound-force** (feature: `customary`).
//!
//! ```rust
//! use qtty_core::force::{Kilonewtons, Newton};
//!
//! let kn = Kilonewtons::new(1.0);
//! let n = kn.to::<Newton>();
//! assert_eq!(n.value(), 1000.0);
//! ```

use crate::{Quantity, Unit};
use qtty_derive::Unit;

/// Re-export the force dimension from the dimension module.
pub use crate::dimension::Force;

/// Marker trait for any [`Unit`] whose dimension is [`Force`].
pub trait ForceUnit: Unit<Dim = Force> {}
impl<T: Unit<Dim = Force>> ForceUnit for T {}

// ─────────────────────────────────────────────────────────────────────────────
// SI newton
// ─────────────────────────────────────────────────────────────────────────────

/// Newton — SI coherent derived unit of force (kg·m/s²).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "N", dimension = Force, ratio = 1.0)]
pub struct Newton;
/// Type alias shorthand for [`Newton`].
pub type N = Newton;
/// A quantity measured in newtons.
pub type Newtons = Quantity<N>;
/// One newton.
pub const NEWTON: Newtons = Newtons::new(1.0);

macro_rules! si_newton {
    ($name:ident, $sym:literal, $ratio:expr, $alias:ident, $qty:ident, $one:ident) => {
        #[doc = concat!("SI-prefixed newton unit (", stringify!($ratio), " N).")]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
        #[unit(symbol = $sym, dimension = Force, ratio = $ratio)]
        pub struct $name;
        #[doc = concat!("Type alias shorthand for [`", stringify!($name), "`].")]
        pub type $alias = $name;
        #[doc = concat!("A quantity measured in ", stringify!($name), "s.")]
        pub type $qty = Quantity<$alias>;
        #[doc = concat!("One ", stringify!($name), ".")]
        pub const $one: $qty = $qty::new(1.0);
    };
}

si_newton!(Micronewton, "µN", 1e-6, Un, Micronewtons, MICRONEWTON);
si_newton!(Millinewton, "mN", 1e-3, Mn, Millinewtons, MILLINEWTON);
si_newton!(Kilonewton, "kN", 1e3, Kn, Kilonewtons, KILONEWTON);
si_newton!(Meganewton, "MN", 1e6, MnAlias, Meganewtons, MEGANEWTON);
si_newton!(Giganewton, "GN", 1e9, GnAlias, Giganewtons, GIGANEWTON);

// ─────────────────────────────────────────────────────────────────────────────
// Feature-gated units
// ─────────────────────────────────────────────────────────────────────────────

/// Dyne — CGS unit of force (1 dyn = 10⁻⁵ N).
#[cfg(feature = "fundamental-physics")]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "dyn", dimension = Force, ratio = 1e-5)]
pub struct Dyne;
/// A quantity measured in dynes.
#[cfg(feature = "fundamental-physics")]
pub type Dynes = Quantity<Dyne>;

/// Pound-force (1 lbf = g₀ × 1 lb ≈ 4.448 222 N).
#[cfg(feature = "customary")]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "lbf", dimension = Force, ratio = 4.448_222_615_260_5)]
pub struct PoundForce;
/// A quantity measured in pounds-force.
#[cfg(feature = "customary")]
pub type PoundsForce = Quantity<PoundForce>;

// ─────────────────────────────────────────────────────────────────────────────
// Unit inventory macro
// ─────────────────────────────────────────────────────────────────────────────

/// Canonical list of always-available (metric SI) force units.
#[macro_export]
#[doc(hidden)]
macro_rules! force_units {
    ($cb:path) => {
        $cb!(
            Newton,
            Micronewton,
            Millinewton,
            Kilonewton,
            Meganewton,
            Giganewton
        );
    };
}

// Generate bidirectional From impls between base metric SI force units.
force_units!(crate::impl_unit_from_conversions);

#[cfg(feature = "cross-unit-ops")]
force_units!(crate::impl_unit_cross_unit_ops);

// Compile-time check: every base force unit is registered as BuiltinUnit.
#[cfg(test)]
force_units!(crate::assert_units_are_builtin);

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn kilonewton_to_newton() {
        let kn = Kilonewtons::new(1.0);
        let n: Newtons = kn.to();
        assert_abs_diff_eq!(n.value(), 1_000.0, epsilon = 1e-12);
    }

    #[test]
    fn newton_to_millinewton() {
        let n = Newtons::new(1.0);
        let mn: Millinewtons = n.to();
        assert_abs_diff_eq!(mn.value(), 1_000.0, epsilon = 1e-12);
    }

    #[test]
    #[cfg(feature = "fundamental-physics")]
    fn newton_to_dyne() {
        let n = Newtons::new(1.0);
        let d: Dynes = n.to();
        assert_abs_diff_eq!(d.value(), 1e5, epsilon = 1e-7);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn newton_to_lbf() {
        let n = Newtons::new(4.448_222_615_260_5);
        let lbf: PoundsForce = n.to();
        assert_abs_diff_eq!(lbf.value(), 1.0, epsilon = 1e-9);
    }
}
