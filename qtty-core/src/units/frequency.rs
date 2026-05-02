// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Frequency units (feature: `frequency`).
//!
//! The canonical scaling unit for this dimension is the **hertz**
//! (`Hertz::RATIO == 1.0`). All other frequency units are expressed as
//! exact ratios to hertz (= s⁻¹).
//!
//! ```rust
//! use qtty_core::frequency::{Kilohertzs, Hertz};
//!
//! let khz = Kilohertzs::new(1.0);
//! let hz = khz.to::<Hertz>();
//! assert_eq!(hz.value(), 1_000.0);
//! ```

use crate::{Quantity, Unit};
use qtty_derive::Unit;

/// Re-export the frequency dimension from the dimension module.
pub use crate::dimension::Frequency;

/// Marker trait for any [`Unit`] whose dimension is [`Frequency`].
pub trait FrequencyUnit: Unit<Dim = Frequency> {}
impl<T: Unit<Dim = Frequency>> FrequencyUnit for T {}

// ─────────────────────────────────────────────────────────────────────────────
// SI hertz
// ─────────────────────────────────────────────────────────────────────────────

/// Hertz — SI coherent derived unit of frequency (s⁻¹).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Hz", dimension = Frequency, ratio = 1.0)]
pub struct Hertz;
/// Type alias shorthand for [`Hertz`].
pub type Hz = Hertz;
/// A quantity measured in hertz.
pub type Hertzs = Quantity<Hz>;
/// One hertz.
pub const HERTZ: Hertzs = Hertzs::new(1.0);

macro_rules! si_hertz {
    ($name:ident, $sym:literal, $ratio:expr, $alias:ident, $qty:ident, $one:ident) => {
        #[doc = concat!("SI-prefixed hertz unit (", stringify!($ratio), " Hz).")]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
        #[unit(symbol = $sym, dimension = Frequency, ratio = $ratio)]
        pub struct $name;
        #[doc = concat!("Type alias shorthand for [`", stringify!($name), "`].")]
        pub type $alias = $name;
        #[doc = concat!("A quantity measured in ", stringify!($name), ".")]
        pub type $qty = Quantity<$alias>;
        #[doc = concat!("One ", stringify!($name), ".")]
        pub const $one: $qty = $qty::new(1.0);
    };
}

si_hertz!(Millihertz, "mHz", 1e-3, MHzm, Millihertzs, MILLIHERTZ);
si_hertz!(Kilohertz, "kHz", 1e3, KHz, Kilohertzs, KILOHERTZ);
si_hertz!(Megahertz, "MHz", 1e6, MHz, Megahertzs, MEGAHERTZ);
si_hertz!(Gigahertz, "GHz", 1e9, GHz, Gigahertzs, GIGAHERTZ);
si_hertz!(Terahertz, "THz", 1e12, THz, Terahertzs, TERAHERTZ);

// ─────────────────────────────────────────────────────────────────────────────
// Unit inventory macro
// ─────────────────────────────────────────────────────────────────────────────

/// Canonical list of frequency units.
#[macro_export]
#[doc(hidden)]
macro_rules! frequency_units {
    ($cb:path) => {
        $cb!(Hertz, Millihertz, Kilohertz, Megahertz, Gigahertz, Terahertz);
    };
}

frequency_units!(crate::impl_unit_from_conversions);

#[cfg(feature = "cross-unit-ops")]
frequency_units!(crate::impl_unit_cross_unit_ops);

#[cfg(test)]
frequency_units!(crate::assert_units_are_builtin);

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn kilohertz_to_hertz() {
        let khz = Kilohertzs::new(1.0);
        let hz: Hertzs = khz.to();
        assert_abs_diff_eq!(hz.value(), 1_000.0, epsilon = 1e-12);
    }

    #[test]
    fn megahertz_to_kilohertz() {
        let mhz = Megahertzs::new(1.0);
        let khz: Kilohertzs = mhz.to();
        assert_abs_diff_eq!(khz.value(), 1_000.0, epsilon = 1e-9);
    }

    #[test]
    fn gigahertz_to_hertz() {
        let ghz = Gigahertzs::new(2.4);
        let hz: Hertzs = ghz.to();
        assert_abs_diff_eq!(hz.value(), 2_400_000_000.0, epsilon = 1.0);
    }

    #[test]
    fn millihertz_to_hertz() {
        let mhz = Millihertzs::new(1_000.0);
        let hz: Hertzs = mhz.to();
        assert_abs_diff_eq!(hz.value(), 1.0, epsilon = 1e-12);
    }
}
