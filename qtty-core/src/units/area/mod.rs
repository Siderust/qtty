// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Area units.
//!
//! The canonical scaling unit for this dimension is the **square metre** (`SquareMeter::RATIO == 1.0`).
//! All other area units are expressed as exact ratios to square metres.
//!
//! This module provides:
//!
//! - **Metric squares**: square millimetre, square centimetre, square metre, square kilometre.
//! - **Land measurement**: hectare, are.
//! - **Imperial/US**: square inch, square foot, square yard, square mile, acre.
//!
//! Area units arise *directly* from multiplying two length quantities — no
//! conversion needed:
//!
//! ```rust
//! use qtty_core::length::Meters;
//! use qtty_core::area::SquareMeters;
//!
//! let side = Meters::new(5.0);
//! let area: SquareMeters = side * side;             // Direct product
//! assert!((area.value() - 25.0).abs() < 1e-12);
//! ```
//!
//! ## All area units (default)
//!
//! ```rust
//! use qtty_core::area::*;
//!
//! macro_rules! touch {
//!     ($T:ty, $v:expr) => {{ let q = <$T>::new($v); let _c = q; assert!(q == q); }};
//! }
//!
//! touch!(SquareMeters, 1.0);     touch!(SquareKilometers, 1.0);
//! touch!(SquareCentimeters, 1.0);touch!(SquareMillimeters, 1.0);
//! ```

use crate::units::length::{Centimeter, Kilometer, Meter, Millimeter};
use crate::{Prod, Quantity, Unit};

/// Re-export the area dimension from the dimension module.
pub use crate::dimension::Area;

/// Marker trait for any [`Unit`] whose dimension is [`Area`].
pub trait AreaUnit: Unit<Dim = Area> {}
impl<T: Unit<Dim = Area>> AreaUnit for T {}

/// A composed area quantity from squaring a length unit.
///
/// `SquareOf<L>` is `Quantity<Prod<L, L>>`. Since the metric area types are
/// themselves `Prod` aliases, `SquareOf<Meter>` and [`SquareMeters`] are the
/// **same type**.
///
/// # Examples
///
/// ```rust
/// use qtty_core::area::{SquareOf, SquareMeters};
/// use qtty_core::length::{Meter, Meters};
///
/// let side = Meters::new(5.0);
/// let area: SquareOf<Meter> = side * side;
/// assert!((area.value() - 25.0).abs() < 1e-12);
///
/// // SquareOf<Meter> IS SquareMeters — same type:
/// let named: SquareMeters = area;
/// assert!((named.value() - 25.0).abs() < 1e-12);
/// ```
pub type SquareOf<L> = Quantity<Prod<L, L>>;

#[cfg(feature = "land-area")]
mod land_area;
#[cfg(feature = "land-area")]
pub use land_area::*;
#[cfg(feature = "customary")]
mod customary;
#[cfg(feature = "customary")]
pub use customary::*;

// ─────────────────────────────────────────────────────────────────────────────
// SI / metric area units
// ─────────────────────────────────────────────────────────────────────────────

/// Square metre — product of two [`Meter`] units (1 m²).
pub type SquareMeter = Prod<Meter, Meter>;
/// A quantity measured in square metres (= [`SquareOf<Meter>`]).
pub type SquareMeters = Quantity<SquareMeter>;

/// Square kilometre — product of two [`Kilometer`] units (10⁶ m²).
pub type SquareKilometer = Prod<Kilometer, Kilometer>;
/// A quantity measured in square kilometres.
pub type SquareKilometers = Quantity<SquareKilometer>;

/// Square centimetre — product of two [`Centimeter`] units (10⁻⁴ m²).
pub type SquareCentimeter = Prod<Centimeter, Centimeter>;
/// A quantity measured in square centimetres.
pub type SquareCentimeters = Quantity<SquareCentimeter>;

/// Square millimetre — product of two [`Millimeter`] units (10⁻⁶ m²).
pub type SquareMillimeter = Prod<Millimeter, Millimeter>;
/// A quantity measured in square millimetres.
pub type SquareMillimeters = Quantity<SquareMillimeter>;

// ─────────────────────────────────────────────────────────────────────────────
// Imperial / US customary area units
// ─────────────────────────────────────────────────────────────────────────────

/// Canonical list of all area units.
///
/// Pass a macro identifier as the single argument; it will be invoked with all
/// area unit types as its token list. Drives:
/// - `impl_unit_from_conversions!` — bidirectional `From` impls between all pairs.
/// - `impl_unit_cross_unit_ops!` — cross-unit `PartialEq`/`PartialOrd` (feature-gated).
/// - `assert_units_are_builtin!` — compile-time check that every unit is in
///   `register_builtin_units!` (under `#[cfg(test)]`).
///
/// The macro is exported (`#[doc(hidden)]`) so the `qtty` facade can use it
/// in compile-time consistency checks (`inventory_consistency.rs`).
///
/// ```rust,ignore
/// area_units!(crate::impl_unit_from_conversions);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! area_units {
    ($cb:path) => {
        $cb!(
            SquareMeter,
            SquareKilometer,
            SquareCentimeter,
            SquareMillimeter
        );
    };
}

// Generate all bidirectional From implementations between area units.
area_units!(crate::impl_unit_from_conversions);

// Optional cross-unit operator support (`==`, `<`, etc.).
#[cfg(feature = "cross-unit-ops")]
area_units!(crate::impl_unit_cross_unit_ops);

// ── Cross-feature: customary × land-area ─────────────────────────────────────
#[cfg(all(feature = "customary", feature = "land-area"))]
crate::__impl_from_each_extra_to_bases!(
    {SquareInch, SquareFoot, SquareYard, SquareMile}
    Hectare, Are, Acre
);
#[cfg(all(
    feature = "customary",
    feature = "land-area",
    feature = "cross-unit-ops"
))]
crate::__impl_cross_ops_each_extra_to_bases!(
    {SquareInch, SquareFoot, SquareYard, SquareMile}
    Hectare, Are, Acre
);

// Compile-time check: every unit in the inventory is registered as BuiltinUnit.
#[cfg(test)]
area_units!(crate::assert_units_are_builtin);

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn sqm_to_sqkm() {
        let a = SquareMeters::new(1_000_000.0);
        let b: SquareKilometers = a.to();
        assert_abs_diff_eq!(b.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    #[cfg(feature = "land-area")]
    fn hectare_to_sqm() {
        let a = Hectares::new(1.0);
        let b: SquareMeters = a.to();
        assert_abs_diff_eq!(b.value(), 10_000.0, epsilon = 1e-9);
    }

    #[test]
    #[cfg(feature = "land-area")]
    fn acre_to_hectare() {
        let a = Acres::new(1.0);
        let b: Hectares = a.to();
        assert_abs_diff_eq!(b.value(), 0.404_685_642_24, epsilon = 1e-9);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn sqft_to_sqm() {
        let a = SquareFeet::new(1.0);
        let b: SquareMeters = a.to();
        assert_abs_diff_eq!(b.value(), 0.092_903_04, epsilon = 1e-9);
    }

    #[test]
    fn length_product_is_area() {
        use crate::length::Meters;

        let side = Meters::new(5.0);
        let area: SquareMeters = side * side; // direct — no .to() needed
        assert_abs_diff_eq!(area.value(), 25.0, epsilon = 1e-12);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn sqmile_to_sqkm() {
        let a = SquareMiles::new(1.0);
        let b: SquareKilometers = a.to();
        assert_abs_diff_eq!(b.value(), 2.589_988_110_336, epsilon = 1e-6);
    }

    #[test]
    fn sqcm_to_sqm() {
        let a = SquareCentimeters::new(10_000.0);
        let b: SquareMeters = a.to();
        assert_abs_diff_eq!(b.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn sqmm_to_sqcm() {
        let a = SquareMillimeters::new(100.0);
        let b: SquareCentimeters = a.to();
        assert_abs_diff_eq!(b.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    #[cfg(feature = "land-area")]
    fn are_to_sqm() {
        let a = Ares::new(1.0);
        let b: SquareMeters = a.to();
        assert_abs_diff_eq!(b.value(), 100.0, epsilon = 1e-12);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn sqinch_to_sqcm() {
        let a = SquareInches::new(1.0);
        let b: SquareCentimeters = a.to();
        // 1 in² = 6.4516 cm²
        assert_abs_diff_eq!(b.value(), 6.4516, epsilon = 1e-9);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn sqyard_to_sqm() {
        let a = SquareYards::new(1.0);
        let b: SquareMeters = a.to();
        assert_abs_diff_eq!(b.value(), 0.836_127_36, epsilon = 1e-9);
    }

    #[test]
    fn roundtrip_sqcm_sqm() {
        let original = SquareCentimeters::new(250.0);
        let converted = original.to::<SquareMeter>();
        let back = converted.to::<SquareCentimeter>();
        assert_abs_diff_eq!(back.value(), original.value(), epsilon = 1e-10);
    }

    #[test]
    fn sqrt_recovers_length_unit() {
        use crate::length::{Meter, Meters};

        let area = SquareMeters::new(36.0);
        let side: crate::Quantity<Meter> = area.sqrt();
        assert_abs_diff_eq!(side.value(), 6.0, epsilon = 1e-12);

        // Round-trip: side² == area
        let again: SquareMeters = side * side;
        assert_abs_diff_eq!(again.value(), area.value(), epsilon = 1e-12);

        // Pure type-level: sqrt of (Meters * Meters) is Meters
        let s = Meters::new(7.0);
        let a = s * s;
        let r: Meters = a.sqrt();
        assert_abs_diff_eq!(r.value(), 7.0, epsilon = 1e-12);
    }

    #[test]
    fn squared_unit_conversion_uses_squared_scale() {
        // 1 km² = 1_000_000 m² (scale factor squared)
        let one_sqkm = SquareKilometers::new(1.0);
        let in_sqm: SquareMeters = one_sqkm.to();
        assert_abs_diff_eq!(in_sqm.value(), 1.0e6, epsilon = 1e-6);
    }

    #[test]
    #[cfg(feature = "std")]
    fn squared_unit_formatter_output() {
        // Display: "value sym·sym"
        assert_eq!(format!("{}", SquareMeters::new(2.5)), "2.5 m·m");
        // LowerExp uses scientific notation on the value, same symbol layout.
        assert_eq!(format!("{:e}", SquareMeters::new(1234.0)), "1.234e3 m·m");
    }

    #[test]
    #[cfg(feature = "std")]
    fn symbols_are_correct() {
        // Prod-based aliases inherit the Prod Display ("m·m");
        // SYMBOL is empty but Display writes component symbols.
        assert_eq!(format!("{}", SquareMeters::new(1.0)), "1 m·m");
        #[cfg(feature = "land-area")]
        assert_eq!(Hectare::SYMBOL, "ha");
        #[cfg(feature = "land-area")]
        assert_eq!(Acre::SYMBOL, "ac");
        #[cfg(feature = "customary")]
        assert_eq!(SquareInch::SYMBOL, "in²");
    }
}
