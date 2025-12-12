//! # Length Units Module
//!
//! This module provides types and utilities for handling length-related calculations
//! in astronomical and scientific contexts. It includes representations for various
//! length units and conversions between them.
//!
//! ## Features
//! - **Astronomical Unit (AstronomicalUnits)**: The mean distance between the Earth and the Sun.
//! - **Light Year (LightYears)**: The distance light travels in one Julian year in vacuum.
//! - Conversion between AstronomicalUnits and LightYears.
//!
//! ## Example Usage
//! ```rust
//! use siderust_units::{AU, LightYears};
//!
//! let au = 1.0*AU;
//! let ly = LightYears::from(au);
//! assert!((ly.value() - 1.582e-5).abs() < 1e-8);
//! ```

use super::*;

pub enum Length {}
impl Dimension for Length {}
pub trait LengthUnit: Unit<Dim = Length> {}
impl<T: Unit<Dim = Length>> LengthUnit for T {}

define_unit!("m", Meter, Length, 1.0);
pub type Meters = Quantity<Meter>;

define_unit!("Km", Kilometer, Length, 1_000.0);
pub type Km = Kilometer;
pub type Kilometers = Quantity<Km>;
pub const KM: Kilometers = Kilometers::new(1.0);

define_unit!("Au", AstronomicalUnit, Length, 149_597_870_000.7);
pub type Au = AstronomicalUnit;
pub type AstronomicalUnits = Quantity<Au>;
pub const AU: AstronomicalUnits = AstronomicalUnits::new(1.0);

define_unit!("Ly", LightYear, Length, 9_460_730_472_580_000.8);
pub type Ly = LightYear;
pub type LightYears = Quantity<Ly>;
pub const LY: LightYears = LightYears::new(1.0);

define_unit!("SR", SolarRadius, Length, 695_700_000.0);
pub type SolarRadiuses = Quantity<SolarRadius>;
pub const SR: SolarRadiuses = SolarRadiuses::new(1.0);

define_unit!("ps", Parsec, Length, 3.26 * LightYear::RATIO);
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
        assert_abs_diff_eq!(km.value(), 695_700.0, epsilon = 1e-6);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Roundtrip conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn roundtrip_km_m() {
        let original = Kilometers::new(42.5);
        let m = original.to::<Meter>();
        let back = m.to::<Kilometer>();
        assert_abs_diff_eq!(back.value(), original.value(), epsilon = 1e-12);
    }

    #[test]
    fn roundtrip_au_ly() {
        let original = AstronomicalUnits::new(1000.0);
        let ly = original.to::<LightYear>();
        let back = ly.to::<AstronomicalUnit>();
        assert_relative_eq!(back.value(), original.value(), max_relative = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Unit constants
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn unit_constants() {
        assert_eq!(KM.value(), 1.0);
        assert_eq!(AU.value(), 1.0);
        assert_eq!(LY.value(), 1.0);
        assert_eq!(SR.value(), 1.0);
        assert_eq!(PS.value(), 1.0);
    }

    #[test]
    fn constant_arithmetic() {
        let two_au = 2.0 * AU;
        assert_eq!(two_au.value(), 2.0);

        let half_ly = LY / 2.0;
        assert_eq!(half_ly.value(), 0.5);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Display
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn display_meters() {
        let m = Meters::new(1000.0);
        assert_eq!(format!("{}", m), "1000 \"m\"");
    }

    #[test]
    fn display_kilometers() {
        let km = Kilometers::new(42.5);
        assert_eq!(format!("{}", km), "42.5 \"Km\"");
    }

    #[test]
    fn display_au() {
        let au = AstronomicalUnits::new(1.5);
        assert_eq!(format!("{}", au), "1.5 \"Au\"");
    }

    #[test]
    fn display_light_year() {
        let ly = LightYears::new(4.2);
        assert_eq!(format!("{}", ly), "4.2 \"Ly\"");
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Edge cases
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn zero_length() {
        let zero = Meters::new(0.0);
        assert_eq!(zero.to::<Kilometer>().value(), 0.0);
        assert_eq!(zero.to::<LightYear>().value(), 0.0);
    }

    #[test]
    fn negative_length() {
        // Negative lengths are allowed (could represent direction)
        let neg = Kilometers::new(-100.0);
        let m = neg.to::<Meter>();
        assert_abs_diff_eq!(m.value(), -100_000.0, epsilon = 1e-9);
    }

    #[test]
    fn very_small_length() {
        let tiny = Meters::new(1e-10);
        let km = tiny.to::<Kilometer>();
        assert_abs_diff_eq!(km.value(), 1e-13, epsilon = 1e-25);
    }

    #[test]
    fn very_large_length() {
        // Distance to Andromeda galaxy ~ 2.5 million LY
        let andromeda = LightYears::new(2.5e6);
        let au = andromeda.to::<AstronomicalUnit>();
        // Should be about 1.58e11 AU
        assert!(au.value() > 1e11);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_km_m_roundtrip(km_val in -1e9..1e9f64) {
            let km = Kilometers::new(km_val);
            let m = km.to::<Meter>();
            let back = m.to::<Kilometer>();
            assert_relative_eq!(back.value(), km.value(), max_relative = 1e-12);
        }

        #[test]
        fn prop_km_to_m_ratio(km_val in -1e9..1e9f64) {
            let km = Kilometers::new(km_val);
            let m = km.to::<Meter>();
            assert_relative_eq!(m.value(), km_val * 1000.0, max_relative = 1e-12);
        }

        #[test]
        fn prop_au_ly_roundtrip(au_val in 1e-6..1e6f64) {
            let au = AstronomicalUnits::new(au_val);
            let ly = au.to::<LightYear>();
            let back = ly.to::<AstronomicalUnit>();
            assert_relative_eq!(back.value(), au.value(), max_relative = 1e-12);
        }
    }
}
