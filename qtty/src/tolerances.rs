// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

//! Typed integration tolerances for ODE propagators.
//!
//! These newtypes wrap [`Quantity`] values to give clear names to the rel/abs
//! tolerances that integrators like Dormand–Prince 5(4) and DOP853 require.
//! They replace raw `f64` fields and prevent mixing relative and absolute
//! tolerances accidentally.
//!
//! ## Usage
//!
//! ```rust
//! use qtty::tolerances::IntegratorTolerances;
//!
//! let tol = IntegratorTolerances::uniform(1e-9, 1e-6, 1e-9);
//! assert!((tol.rel.value() - 1e-9).abs() < 1e-20);
//! ```

use crate::dynamics::{KmPerSecond, KmPerSeconds};
use qtty_core::units::dimensionless::{Ratio, Ratios};
use qtty_core::units::length::{Kilometer, Kilometers};
use qtty_core::Quantity;

// ─────────────────────────────────────────────────────────────────────────────
// RelativeTolerance
// ─────────────────────────────────────────────────────────────────────────────

/// Typed relative tolerance for ODE integration (dimensionless).
///
/// Wraps a [`Ratios`] value. Typical values are `1e-9` to `1e-12` for
/// high-precision orbit propagation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RelativeTolerance(pub Ratios);

impl RelativeTolerance {
    /// Creates a relative tolerance from a raw `f64` dimensionless value.
    #[inline]
    pub fn new(value: f64) -> Self {
        Self(Quantity::<Ratio>::new(value))
    }

    /// Returns the raw dimensionless value.
    #[inline]
    pub fn value(self) -> f64 {
        self.0.value()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// AbsoluteTolerancePosition
// ─────────────────────────────────────────────────────────────────────────────

/// Typed absolute position tolerance, parameterised by length unit.
///
/// Defaults to [`Kilometer`].  Typical values are `1e-3` km (1 metre) for
/// near-Earth orbit propagation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AbsoluteTolerancePosition<U = Kilometer>(pub Quantity<U>)
where
    U: qtty_core::Unit<Dim = qtty_core::Length>;

impl AbsoluteTolerancePosition<Kilometer> {
    /// Creates a position tolerance in kilometres.
    #[inline]
    pub fn new_km(value: f64) -> Self {
        Self(Kilometers::new(value))
    }
}

impl<U> AbsoluteTolerancePosition<U>
where
    U: qtty_core::Unit<Dim = qtty_core::Length>,
{
    /// Creates a position tolerance from a typed length quantity.
    #[inline]
    pub fn new(quantity: Quantity<U>) -> Self {
        Self(quantity)
    }

    /// Returns the raw numeric value in the stored unit.
    #[inline]
    pub fn value(self) -> f64
    where
        U: qtty_core::Unit<Dim = qtty_core::Length>,
    {
        self.0.value()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// AbsoluteToleranceVelocity
// ─────────────────────────────────────────────────────────────────────────────

/// Typed absolute velocity tolerance, parameterised by velocity unit.
///
/// Defaults to [`KmPerSecond`].  Typical values are `1e-6` km/s for
/// near-Earth orbit propagation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AbsoluteToleranceVelocity<V = KmPerSecond>(pub Quantity<V>)
where
    V: qtty_core::Unit<Dim = qtty_core::Velocity>;

impl AbsoluteToleranceVelocity<KmPerSecond> {
    /// Creates a velocity tolerance in km/s.
    #[inline]
    pub fn new_km_s(value: f64) -> Self {
        Self(KmPerSeconds::new(value))
    }
}

impl<V> AbsoluteToleranceVelocity<V>
where
    V: qtty_core::Unit<Dim = qtty_core::Velocity>,
{
    /// Creates a velocity tolerance from a typed velocity quantity.
    #[inline]
    pub fn new(quantity: Quantity<V>) -> Self {
        Self(quantity)
    }

    /// Returns the raw numeric value in the stored unit.
    #[inline]
    pub fn value(self) -> f64 {
        self.0.value()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// IntegratorTolerances
// ─────────────────────────────────────────────────────────────────────────────

/// Combined typed tolerances for a variable-step ODE integrator.
///
/// Holds:
/// - `rel` — a single relative tolerance applied to all state components.
/// - `abs_pos` — three per-axis absolute position tolerances (x, y, z).
/// - `abs_vel` — three per-axis absolute velocity tolerances (vx, vy, vz).
///
/// Use [`IntegratorTolerances::uniform`] when you want the same threshold
/// for all axes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntegratorTolerances {
    /// Relative tolerance (dimensionless).
    pub rel: RelativeTolerance,
    /// Per-axis absolute position tolerances (km by default).
    pub abs_pos: [AbsoluteTolerancePosition; 3],
    /// Per-axis absolute velocity tolerances (km/s by default).
    pub abs_vel: [AbsoluteToleranceVelocity; 3],
}

impl IntegratorTolerances {
    /// Constructs a set of tolerances with the same threshold for all axes.
    ///
    /// # Arguments
    ///
    /// * `rel` — dimensionless relative tolerance (e.g. `1e-9`).
    /// * `abs_pos_km` — absolute position tolerance in kilometres (e.g. `1e-6` = 1 mm).
    /// * `abs_vel_km_s` — absolute velocity tolerance in km/s (e.g. `1e-9`).
    pub fn uniform(rel: f64, abs_pos_km: f64, abs_vel_km_s: f64) -> Self {
        let abs_p = AbsoluteTolerancePosition::new_km(abs_pos_km);
        let abs_v = AbsoluteToleranceVelocity::new_km_s(abs_vel_km_s);
        Self {
            rel: RelativeTolerance::new(rel),
            abs_pos: [abs_p, abs_p, abs_p],
            abs_vel: [abs_v, abs_v, abs_v],
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    #[test]
    fn relative_tolerance_roundtrip() {
        let r = RelativeTolerance::new(1e-9);
        assert!((r.value() - 1e-9).abs() < 1e-25);
    }

    #[test]
    fn abs_pos_tolerance_roundtrip() {
        let p = AbsoluteTolerancePosition::new_km(1e-6);
        assert!((p.value() - 1e-6).abs() < 1e-22);
    }

    #[test]
    fn abs_vel_tolerance_roundtrip() {
        let v = AbsoluteToleranceVelocity::new_km_s(1e-9);
        assert!((v.value() - 1e-9).abs() < 1e-25);
    }

    #[test]
    fn integrator_tolerances_uniform() {
        let tol = IntegratorTolerances::uniform(1e-9, 1e-6, 1e-9);
        assert!((tol.rel.value() - 1e-9).abs() < 1e-25);
        for p in &tol.abs_pos {
            assert!((p.value() - 1e-6).abs() < 1e-22);
        }
        for v in &tol.abs_vel {
            assert!((v.value() - 1e-9).abs() < 1e-25);
        }
    }
}
