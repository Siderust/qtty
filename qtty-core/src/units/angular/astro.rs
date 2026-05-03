// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

/// Arcminute (`1/60` degree).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "′", dimension = Angular, ratio = 1.0 / 60.0)]
pub struct Arcminute;
/// Alias for [`Arcminute`] (minute of angle, MOA).
pub type MOA = Arcminute;
/// Type alias shorthand for [`Arcminute`].
pub type Arcm = Arcminute;
/// Convenience alias for an arcminute quantity.
pub type Arcminutes = Quantity<Arcm>;
/// One arcminute.
pub const ARCM: Arcminutes = Arcminutes::new(1.0);

/// Arcsecond (`1/3600` degree).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "″", dimension = Angular, ratio = 1.0 / 3600.0)]
pub struct Arcsecond;
/// Type alias shorthand for [`Arcsecond`].
pub type Arcs = Arcsecond;
/// Convenience alias for an arcsecond quantity.
pub type Arcseconds = Quantity<Arcs>;
/// One arcsecond.
pub const ARCS: Arcseconds = Arcseconds::new(1.0);

/// Milliarcsecond (`1/3_600_000` degree).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mas", dimension = Angular, ratio = 1.0 / 3_600_000.0)]
pub struct MilliArcsecond;
/// Type alias shorthand for [`MilliArcsecond`].
pub type Mas = MilliArcsecond;
/// Convenience alias for a milliarcsecond quantity.
pub type MilliArcseconds = Quantity<Mas>;
/// One milliarcsecond.
pub const MAS: MilliArcseconds = MilliArcseconds::new(1.0);

/// Microarcsecond (`1/3_600_000_000` degree).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "μas", dimension = Angular, ratio = 1.0 / 3_600_000_000.0)]
pub struct MicroArcsecond;
/// Type alias shorthand for [`MicroArcsecond`].
pub type Uas = MicroArcsecond;
/// Convenience alias for a microarcsecond quantity.
pub type MicroArcseconds = Quantity<Uas>;
/// One microarcsecond.
pub const UAS: MicroArcseconds = MicroArcseconds::new(1.0);

/// Hour angle hour (`15` degrees).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "h", dimension = Angular, ratio = 15.0)]
pub struct HourAngle;
/// Type alias shorthand for [`HourAngle`].
pub type Hms = HourAngle;
/// Convenience alias for an hour-angle quantity.
pub type HourAngles = Quantity<Hms>;
/// One hour angle hour (==15°).
pub const HOUR_ANGLE: HourAngles = HourAngles::new(1.0);

impl HourAngles {
    /// Construct from **HMS** components (`hours`, `minutes`, `seconds`).
    ///
    /// Sign is taken from `hours`; the `minutes` and `seconds` parameters are treated as magnitudes.
    ///
    /// ```rust
    /// use qtty_core::angular::HourAngles;
    /// let ra = HourAngles::from_hms(5, 30, 0.0); // 5h30m == 5.5h
    /// assert_eq!(ra.value(), 5.5);
    /// ```
    pub const fn from_hms(hours: i32, minutes: u32, seconds: f64) -> Self {
        let sign = if hours < 0 { -1.0 } else { 1.0 };
        let h_abs = if hours < 0 {
            -(hours as f64)
        } else {
            hours as f64
        };
        let m = minutes as f64 / 60.0;
        let s = seconds / 3600.0;
        let total_hours = sign * (h_abs + m + s);
        Self::new(total_hours)
    }
}

crate::impl_unit_from_conversions_between!(
    Degree, Radian, Milliradian, Turn;
    Arcminute, Arcsecond, MilliArcsecond, MicroArcsecond, HourAngle
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    Degree, Radian, Milliradian, Turn;
    Arcminute, Arcsecond, MilliArcsecond, MicroArcsecond, HourAngle
);

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────
#[macro_export]
#[doc(hidden)]
macro_rules! angular_astro_units {
    ($cb:path) => {
        $cb!(
            Arcminute,
            Arcsecond,
            MilliArcsecond,
            MicroArcsecond,
            HourAngle,
        );
    };
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use proptest::prelude::*;

    #[test]
    fn arcminute_to_degrees() {
        let arcm = Arcminutes::new(60.0);
        let degrees: Degrees = arcm.to();
        assert_abs_diff_eq!(degrees.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn hour_angle_to_degrees() {
        let hour = HourAngles::new(1.0);
        let degrees: Degrees = hour.to();
        assert_abs_diff_eq!(degrees.value(), 15.0, epsilon = 1e-12);
    }

    proptest! {
        #[test]
        fn arcsecond_degree_roundtrip(v in -1.0e9_f64..1.0e9_f64) {
            let arcseconds = Arcseconds::new(v);
            let roundtrip: Arcseconds = arcseconds.to::<Degree>().to();
            prop_assert!((roundtrip.value() - v).abs() <= v.abs().max(1.0) * 1e-12);
        }
    }
}
