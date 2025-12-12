use unit::{Meters, Seconds, Simplify, Unitless};

fn main() {
    let half = Meters::new(1.0) / Meters::new(2.0);
    let unitless: unit::Quantity<Unitless> = half.simplify();
    assert!((unitless.value() - 0.5).abs() < 1e-12);

    let ratio = Seconds::new(1.0) / Seconds::new(1.0);
    assert_eq!(ratio.asin(), core::f64::consts::FRAC_PI_2);
}
