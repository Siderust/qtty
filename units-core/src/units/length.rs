//! Length units and conversions.
//!
//! This module defines common SI and astronomy-friendly length units.
//!
//! ## Example
//! ```rust
//! use unit_core::length::{AU, LightYears};
//!
//! let au = 1.0 * AU;
//! let ly = LightYears::from(au);
//! assert!((ly.value() - 1.581_250_74e-5).abs() < 1e-8);
//! ```

use crate::{Dimension, Quantity, Unit};
use unit_derive::Unit;

pub enum Length {}
impl Dimension for Length {}
pub trait LengthUnit: Unit<Dim = Length> {}
impl<T: Unit<Dim = Length>> LengthUnit for T {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "m", dimension = Length, ratio = 1.0)]
pub struct Meter;
pub type Meters = Quantity<Meter>;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Km", dimension = Length, ratio = 1_000.0)]
pub struct Kilometer;
pub type Km = Kilometer;
pub type Kilometers = Quantity<Km>;
pub const KM: Kilometers = Kilometers::new(1.0);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Au", dimension = Length, ratio = 149_597_870_000.7)]
pub struct AstronomicalUnit;
pub type Au = AstronomicalUnit;
pub type AstronomicalUnits = Quantity<Au>;
pub const AU: AstronomicalUnits = AstronomicalUnits::new(1.0);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Ly", dimension = Length, ratio = 9_460_730_472_580_000.8)]
pub struct LightYear;
pub type Ly = LightYear;
pub type LightYears = Quantity<Ly>;
pub const LY: LightYears = LightYears::new(1.0);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "SR", dimension = Length, ratio = 695_700_000.0)]
pub struct SolarRadius;
pub type SolarRadiuses = Quantity<SolarRadius>;
pub const SR: SolarRadiuses = SolarRadiuses::new(1.0);

// Parsec: defined as 3.26 light years
// Note: We use a const expression that will be evaluated at compile time
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ps", dimension = Length, ratio = 3.26 * 9_460_730_472_580_000.8)]
pub struct Parsec;
pub type Parsecs = Quantity<Parsec>;
pub const PS: Parsecs = Parsecs::new(1.0);

/// AstronomicalUnit -> LightYear.
impl From<AstronomicalUnits> for LightYears {
    fn from(au: AstronomicalUnits) -> Self {
        au.to::<LightYear>()
    }
}

/// LightYear -> AstronomicalUnits.
impl From<LightYears> for AstronomicalUnits {
    fn from(ly: LightYears) -> Self {
        ly.to::<AstronomicalUnit>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_abs_diff_eq, assert_relative_eq};
    use proptest::prelude::*;

    // ─────────────────────────────────────────────────────────────────────────────
    // Basic conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn kilometer_to_meter() {
        let km = Kilometers::new(1.0);
        let m = km.to::<Meter>();
        assert_abs_diff_eq!(m.value(), 1000.0, epsilon = 1e-9);
    }

    #[test]
    fn meter_to_kilometer() {
        let m = Meters::new(1000.0);
        let km = m.to::<Kilometer>();
        assert_abs_diff_eq!(km.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn au_to_meters() {
        let au = AstronomicalUnits::new(1.0);
        let m = au.to::<Meter>();
        // 1 AU ≈ 149,597,870,700 meters
        assert_relative_eq!(m.value(), 149_597_870_000.7, max_relative = 1e-9);
    }

    #[test]
    fn au_to_kilometers() {
        let au = AstronomicalUnits::new(1.0);
        let km = au.to::<Kilometer>();
        // 1 AU ≈ 149,597,870.7 km
        assert_relative_eq!(km.value(), 149_597_870.000_7, max_relative = 1e-9);
    }

    #[test]
    fn light_year_to_meters() {
        let ly = LightYears::new(1.0);
        let m = ly.to::<Meter>();
        // 1 LY ≈ 9.461e15 meters
        assert_relative_eq!(m.value(), 9_460_730_472_580_000.8, max_relative = 1e-9);
    }

    #[test]
    fn light_year_to_kilometers() {
        let ly = LightYears::new(1.0);
        let km = ly.to::<Kilometer>();
        // 1 LY ≈ 9.461e12 km
        assert_relative_eq!(km.value(), 9_460_730_472_580.000_8, max_relative = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // AU <-> LY conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn au_to_light_year() {
        let au = AstronomicalUnits::new(1.0);
        let ly = au.to::<LightYear>();
        // 1 AU ≈ 1.582e-5 LY
        assert_relative_eq!(ly.value(), 1.582e-5, max_relative = 1e-3);
    }

    #[test]
    fn light_year_to_au() {
        let ly = LightYears::new(1.0);
        let au = ly.to::<AstronomicalUnit>();
        // 1 LY ≈ 63,241 AU
        assert_relative_eq!(au.value(), 63241.0, max_relative = 1e-3);
    }

    #[test]
    fn from_impl_au_to_ly() {
        let au = 1.0 * AU;
        let ly: LightYears = au.into();
        assert_relative_eq!(ly.value(), 1.582e-5, max_relative = 1e-3);
    }

    #[test]
    fn from_impl_ly_to_au() {
        let ly = 1.0 * LY;
        let au: AstronomicalUnits = ly.into();
        assert_relative_eq!(au.value(), 63241.0, max_relative = 1e-3);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Parsec conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn parsec_to_light_year() {
        let pc = Parsecs::new(1.0);
        let ly = pc.to::<LightYear>();
        // 1 pc ≈ 3.26 LY
        assert_relative_eq!(ly.value(), 3.26, max_relative = 1e-6);
    }

    #[test]
    fn parsec_to_au() {
        let pc = Parsecs::new(1.0);
        let au = pc.to::<AstronomicalUnit>();
        // 1 pc ≈ 206,265 AU (using exact definition: 1 pc = 3.26 LY, 1 LY ≈ 63241 AU)
        // So 1 pc ≈ 3.26 * 63241 ≈ 206,165 AU
        assert_relative_eq!(au.value(), 3.26 * 63241.0, max_relative = 1e-2);
    }

    #[test]
    fn parsec_ratio_sanity() {
        // Parsec is defined as 3.26 * LightYear::RATIO
        assert_relative_eq!(Parsec::RATIO, 3.26 * LightYear::RATIO, max_relative = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Solar radius
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn solar_radius_to_meters() {
        let sr = SolarRadiuses::new(1.0);
        let m = sr.to::<Meter>();
        // 1 R☉ = 695,700 km = 695,700,000 m
        assert_abs_diff_eq!(m.value(), 695_700_000.0, epsilon = 1e-3);
    }

    #[test]
    fn solar_radius_to_km() {
        let sr = SolarRadiuses::new(1.0);
        let km = sr.to::<Kilometer>();
        // 1 R☉ = 695,700 km
        assert_abs_diff_eq!(km.value(), 695_700.0, epsilon = 1e-6);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Roundtrip conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn roundtrip_km_m() {
        let original = Kilometers::new(42.5);
        let converted = original.to::<Meter>();
        let back = converted.to::<Kilometer>();
        assert_abs_diff_eq!(back.value(), original.value(), epsilon = 1e-12);
    }

    #[test]
    fn roundtrip_au_ly() {
        let original = AstronomicalUnits::new(10000.0);
        let converted = original.to::<LightYear>();
        let back = converted.to::<AstronomicalUnit>();
        assert_relative_eq!(back.value(), original.value(), max_relative = 1e-12);
    }

    #[test]
    fn roundtrip_parsec_ly() {
        let original = Parsecs::new(5.0);
        let converted = original.to::<LightYear>();
        let back = converted.to::<Parsec>();
        assert_relative_eq!(back.value(), original.value(), max_relative = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_roundtrip_km_m(k in -1e6..1e6f64) {
            let original = Kilometers::new(k);
            let converted = original.to::<Meter>();
            let back = converted.to::<Kilometer>();
            prop_assert!((back.value() - original.value()).abs() < 1e-9 * k.abs().max(1.0));
        }

        #[test]
        fn prop_km_m_ratio(k in 1e-6..1e6f64) {
            let km = Kilometers::new(k);
            let m = km.to::<Meter>();
            // 1 km = 1000 m
            prop_assert!((m.value() / km.value() - 1000.0).abs() < 1e-9);
        }

        #[test]
        fn prop_roundtrip_au_km(a in 1e-6..1e6f64) {
            let original = AstronomicalUnits::new(a);
            let converted = original.to::<Kilometer>();
            let back = converted.to::<AstronomicalUnit>();
            prop_assert!((back.value() - original.value()).abs() / original.value() < 1e-12);
        }
    }
}
