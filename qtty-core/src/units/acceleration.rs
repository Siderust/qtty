// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Acceleration unit aliases and named units (`Length / Time²`).
//!
//! This module provides both a **parametric alias** and a **named SI unit** for
//! acceleration:
//!
//! - [`Accel<L, T>`] — a generic alias over [`Per`] that works with any length
//!   and time units, mirroring the [`Velocity`](super::velocity::Velocity)
//!   pattern.
//! - [`MeterPerSecondSquared`] — the SI coherent unit (m/s²) for contexts
//!   where a concrete named type is more convenient.
//!
//! ## Examples
//!
//! ```rust
//! use qtty_core::acceleration::Accel;
//! use qtty_core::length::Meter;
//! use qtty_core::time::Second;
//!
//! let a: Accel<Meter, Second> = Accel::new(9.806_65);
//! assert!((a.value() - 9.806_65).abs() < 1e-12);
//! ```
//!
//! ```rust
//! use qtty_core::acceleration::{MeterPerSecondSquared, MetersPerSecondSquared};
//! use qtty_core::velocity::Velocity;
//! use qtty_core::length::{Meter, Meters};
//! use qtty_core::time::{Second, Seconds};
//!
//! let v = Meters::new(100.0) / Seconds::new(10.0);   // Velocity<Meter, Second>
//! let a: MetersPerSecondSquared = MetersPerSecondSquared::new(5.0);
//! assert!((a.value() - 5.0).abs() < 1e-12);
//! ```

use crate::{Per, Prod, Quantity, Unit};
use qtty_derive::Unit;

/// Re-export the acceleration dimension from the dimension module.
pub use crate::dimension::Acceleration;

/// Marker trait for any unit whose dimension is [`Acceleration`].
pub trait AccelerationUnit: Unit<Dim = Acceleration> {}
impl<T: Unit<Dim = Acceleration>> AccelerationUnit for T {}

/// An acceleration quantity parameterized by length and time units.
///
/// `Accel<L, T>` represents `L / T²` — the natural result of dividing a
/// velocity by a time quantity, or dividing a length by the square of a time.
///
/// # Examples
///
/// ```rust
/// use qtty_core::acceleration::Accel;
/// use qtty_core::length::{Kilometer, Meter};
/// use qtty_core::time::{Hour, Second};
///
/// let a1: Accel<Meter, Second> = Accel::new(9.8);
/// let a2: Accel<Kilometer, Hour> = a1.to();
/// ```
pub type Accel<L, T> = Quantity<Per<L, Prod<T, T>>>;

// ─────────────────────────────────────────────────────────────────────────────
// SI named unit
// ─────────────────────────────────────────────────────────────────────────────

/// Metre per second squared — SI coherent unit of acceleration.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "m/s²", dimension = Acceleration, ratio = 1.0)]
pub struct MeterPerSecondSquared;
/// A quantity measured in metres per second squared.
pub type MetersPerSecondSquared = Quantity<MeterPerSecondSquared>;
/// One metre per second squared.
pub const METER_PER_SECOND_SQUARED: MetersPerSecondSquared = MetersPerSecondSquared::new(1.0);

/// Standard gravity (g₀ = 9.806 65 m/s², exact by definition).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "g₀", dimension = Acceleration, ratio = 9.80665)]
pub struct StandardGravity;
/// A quantity measured in standard gravities.
pub type StandardGravities = Quantity<StandardGravity>;
/// One standard gravity.
pub const STANDARD_GRAVITY: StandardGravities = StandardGravities::new(1.0);

// ─────────────────────────────────────────────────────────────────────────────
// Unit inventory macro
// ─────────────────────────────────────────────────────────────────────────────

/// Canonical list of always-available acceleration units.
#[macro_export]
#[doc(hidden)]
macro_rules! acceleration_units {
    ($cb:path) => {
        $cb!(MeterPerSecondSquared, StandardGravity);
    };
}

// Generate bidirectional From impls.
acceleration_units!(crate::impl_unit_from_conversions);

// Cross-unit ops.
#[cfg(feature = "cross-unit-ops")]
acceleration_units!(crate::impl_unit_cross_unit_ops);

// Compile-time check.
#[cfg(test)]
acceleration_units!(crate::assert_units_are_builtin);

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn standard_gravity_to_mps2() {
        let g = StandardGravities::new(1.0);
        let mps2: MetersPerSecondSquared = g.to();
        assert_abs_diff_eq!(mps2.value(), 9.806_65, epsilon = 1e-10);
    }

    #[test]
    fn mps2_to_standard_gravity() {
        let a = MetersPerSecondSquared::new(9.806_65);
        let g: StandardGravities = a.to();
        assert_abs_diff_eq!(g.value(), 1.0, epsilon = 1e-10);
    }
}
