use super::*;

pub enum Dimensionless {}
impl Dimension for Dimensionless {}
pub type Unitless = f64;

impl Unit for Unitless {
    const RATIO: f64 = 1.0;
    type Dim = Dimensionless;
    const SYMBOL: &'static str = "";
}
impl std::fmt::Display for Quantity<Unitless> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl<U: LengthUnit> From<Quantity<U>> for Quantity<Unitless> {
    fn from(length: Quantity<U>) -> Self {
        Self::new(length.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use proptest::prelude::*;

    // ─────────────────────────────────────────────────────────────────────────────
    // Basic Unitless behavior
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn unitless_new_and_value() {
        let u: Quantity<Unitless> = Quantity::new(42.0);
        assert_eq!(u.value(), 42.0);
    }

    #[test]
    fn unitless_from_f64() {
        let u: Quantity<Unitless> = 1.23456.into();
        assert_abs_diff_eq!(u.value(), 1.23456, epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Display formatting
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn display_unitless() {
        let u: Quantity<Unitless> = Quantity::new(123.456);
        let s = format!("{}", u);
        assert_eq!(s, "123.456");
    }

    #[test]
    fn display_unitless_integer() {
        let u: Quantity<Unitless> = Quantity::new(42.0);
        let s = format!("{}", u);
        assert_eq!(s, "42");
    }

    #[test]
    fn display_unitless_negative() {
        let u: Quantity<Unitless> = Quantity::new(-99.5);
        let s = format!("{}", u);
        assert_eq!(s, "-99.5");
    }

    #[test]
    fn display_unitless_zero() {
        let u: Quantity<Unitless> = Quantity::new(0.0);
        let s = format!("{}", u);
        assert_eq!(s, "0");
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Conversion from Length to Unitless
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn from_meters_to_unitless() {
        let m = Meters::new(100.0);
        let u: Quantity<Unitless> = m.into();
        assert_eq!(u.value(), 100.0);
    }

    #[test]
    fn from_kilometers_to_unitless() {
        let km = Kilometers::new(5.5);
        let u: Quantity<Unitless> = km.into();
        assert_eq!(u.value(), 5.5);
    }

    #[test]
    fn from_au_to_unitless() {
        let au = AstronomicalUnits::new(1.0);
        let u: Quantity<Unitless> = au.into();
        assert_eq!(u.value(), 1.0);
    }

    #[test]
    fn from_light_year_to_unitless() {
        let ly = LightYears::new(4.2);
        let u: Quantity<Unitless> = ly.into();
        assert_eq!(u.value(), 4.2);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Unit trait implementation
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn unitless_ratio() {
        assert_eq!(Unitless::RATIO, 1.0);
    }

    #[test]
    fn unitless_symbol() {
        assert_eq!(Unitless::SYMBOL, "");
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Arithmetic operations
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn unitless_add() {
        let a: Quantity<Unitless> = Quantity::new(3.0);
        let b: Quantity<Unitless> = Quantity::new(4.0);
        assert_eq!((a + b).value(), 7.0);
    }

    #[test]
    fn unitless_sub() {
        let a: Quantity<Unitless> = Quantity::new(10.0);
        let b: Quantity<Unitless> = Quantity::new(3.0);
        assert_eq!((a - b).value(), 7.0);
    }

    #[test]
    fn unitless_mul_f64() {
        let u: Quantity<Unitless> = Quantity::new(5.0);
        assert_eq!((u * 2.0).value(), 10.0);
    }

    #[test]
    fn unitless_div_f64() {
        let u: Quantity<Unitless> = Quantity::new(10.0);
        assert_eq!((u / 2.0).value(), 5.0);
    }

    #[test]
    fn unitless_neg() {
        let u: Quantity<Unitless> = Quantity::new(5.0);
        assert_eq!((-u).value(), -5.0);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Edge cases
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn unitless_zero() {
        let u: Quantity<Unitless> = Quantity::new(0.0);
        assert_eq!(u.value(), 0.0);
    }

    #[test]
    fn unitless_nan() {
        let u: Quantity<Unitless> = Quantity::new(f64::NAN);
        assert!(u.value().is_nan());
    }

    #[test]
    fn unitless_infinity() {
        let inf: Quantity<Unitless> = Quantity::new(f64::INFINITY);
        let neg_inf: Quantity<Unitless> = Quantity::new(f64::NEG_INFINITY);
        assert!(inf.value().is_infinite());
        assert!(neg_inf.value().is_infinite());
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_from_length_preserves_value(v in -1e10..1e10f64) {
            let m = Meters::new(v);
            let u: Quantity<Unitless> = m.into();
            assert_abs_diff_eq!(u.value(), v, epsilon = 1e-12);
        }

        #[test]
        fn prop_arithmetic_consistency(a in -1e10..1e10f64, b in -1e10..1e10f64) {
            let qa: Quantity<Unitless> = Quantity::new(a);
            let qb: Quantity<Unitless> = Quantity::new(b);
            assert_abs_diff_eq!((qa + qb).value(), a + b, epsilon = 1e-9);
            assert_abs_diff_eq!((qa - qb).value(), a - b, epsilon = 1e-9);
        }

        #[test]
        fn prop_display_contains_value(v in -1e6..1e6f64) {
            let u: Quantity<Unitless> = Quantity::new(v);
            let s = format!("{}", u);
            // The display should parse back to approximately the same value
            let parsed: f64 = s.parse().unwrap();
            assert_abs_diff_eq!(parsed, v, epsilon = 1e-6);
        }
    }
}
