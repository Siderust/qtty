use unit::{Degrees, Kilometers, KilometersPerSecond, Radian, Seconds};

fn main() {
    let a = Degrees::new(180.0);
    let r = a.to::<Radian>();
    assert!((r.value() - core::f64::consts::PI).abs() < 1e-12);

    let d = Kilometers::new(1_000.0);
    let t = Seconds::new(100.0);
    let v: KilometersPerSecond = d / t;
    assert!((v.value() - 10.0).abs() < 1e-12);
}
