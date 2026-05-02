// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Thermodynamic temperature units.
//!
//! The canonical scaling unit for this dimension is the **kelvin**
//! (`Kelvin::RATIO == 1.0`). Temperature here is the SI base dimension Θ
//! (thermodynamic temperature), measured on an absolute scale.
//!
//! Only the kelvin is provided. Affine-offset scales such as Celsius and
//! Fahrenheit are intentionally omitted: they require an additive offset that
//! does not compose linearly with other quantities and would silently break
//! the [`Quantity`] arithmetic guarantees. Convert affine-offset readings to
//! kelvin at the boundary before constructing a [`Kelvins`] value.
//!
//! ```rust
//! use qtty_core::temperature::Kelvins;
//!
//! let t = Kelvins::new(284.65); // ≈ 11.5 °C
//! assert!((t.value() - 284.65).abs() < 1e-9);
//! ```

use crate::{Quantity, Unit};
use qtty_derive::Unit;

/// Re-export the temperature dimension from the dimension module.
pub use crate::dimension::Temperature;

/// Marker trait for any [`Unit`] whose dimension is [`Temperature`].
pub trait TemperatureUnit: Unit<Dim = Temperature> {}
impl<T: Unit<Dim = Temperature>> TemperatureUnit for T {}

// ─────────────────────────────────────────────────────────────────────────────
// SI kelvin
// ─────────────────────────────────────────────────────────────────────────────

/// Kelvin — SI base unit of thermodynamic temperature.
///
/// BIPM SI brochure 9th ed., Table 2: the kelvin (K) is defined by fixing the
/// numerical value of the Boltzmann constant *k* to 1.380 649 × 10⁻²³ J K⁻¹.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "K", dimension = Temperature, ratio = 1.0)]
pub struct Kelvin;
/// Type alias shorthand for [`Kelvin`].
pub type K = Kelvin;
/// A quantity measured in kelvin.
pub type Kelvins = Quantity<Kelvin>;
/// One kelvin.
pub const KELVIN: Kelvins = Kelvins::new(1.0);

/// Rankine (°R) — absolute temperature scale; 1 °R = 5/9 K exactly.
///
/// The Rankine degree equals one Fahrenheit degree, starting from absolute
/// zero. Used primarily in US engineering. The conversion to kelvin is
/// exact: `T_K = T_R × 5/9`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "°R", dimension = Temperature, ratio = 5.0 / 9.0)]
pub struct Rankine;
/// Type alias shorthand for [`Rankine`].
pub type R = Rankine;
/// A quantity measured in rankine.
pub type Rankines = Quantity<Rankine>;
/// One rankine.
pub const RANKINE: Rankines = Rankines::new(1.0);

// ─────────────────────────────────────────────────────────────────────────────
// Unit inventory macro
// ─────────────────────────────────────────────────────────────────────────────

/// Canonical list of always-available temperature units.
#[macro_export]
#[doc(hidden)]
macro_rules! temperature_units {
    ($cb:path) => {
        $cb!(Kelvin, Rankine);
    };
}

// Generate bidirectional From impls between all temperature units.
temperature_units!(crate::impl_unit_from_conversions);

#[cfg(feature = "cross-unit-ops")]
temperature_units!(crate::impl_unit_cross_unit_ops);

// Compile-time check: every temperature unit is registered as BuiltinUnit.
#[cfg(test)]
temperature_units!(crate::assert_units_are_builtin);

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn kelvin_roundtrip() {
        let t = Kelvins::new(284.65);
        let t2: Kelvins = t.to();
        assert_abs_diff_eq!(t2.value(), 284.65, epsilon = 1e-12);
    }

    #[test]
    fn kelvin_addition() {
        let a = Kelvins::new(100.0);
        let b = Kelvins::new(184.65);
        let c = a + b;
        assert_abs_diff_eq!(c.value(), 284.65, epsilon = 1e-12);
    }

    #[test]
    fn rankine_to_kelvin() {
        // 1 °R = 5/9 K exactly
        let r = Rankines::new(1.0);
        let k: Kelvins = r.to();
        assert_abs_diff_eq!(k.value(), 5.0 / 9.0, epsilon = 1e-15);
    }

    #[test]
    fn rankine_absolute_zero() {
        // 0 °R = 0 K
        let r = Rankines::new(0.0);
        let k: Kelvins = r.to();
        assert_abs_diff_eq!(k.value(), 0.0, epsilon = 1e-15);
    }

    #[test]
    fn kelvin_to_rankine() {
        // 273.15 K = 491.67 °R
        let k = Kelvins::new(273.15);
        let r: Rankines = k.to();
        assert_abs_diff_eq!(r.value(), 491.67, epsilon = 1e-9);
    }
}
