//! Angle helpers example: wrapping and separation in a single unit type.

use qtty::{Arcsecond, Degree};

fn main() {
    let a = Degree::new(370.0).wrap_signed();
    assert_eq!(a.value(), 10.0);

    let s = Degree::new(45.0).abs_separation(Degree::new(350.0));
    assert_eq!(s.value(), 55.0);

    let arcsec: Arcsecond = Degree::new(1.0).to();
    assert_eq!(arcsec.value(), 3600.0);
}
