use unit::{Arcseconds, Degrees};

fn main() {
    let a = Degrees::new(370.0).wrap_signed();
    assert_eq!(a.value(), 10.0);

    let s = Degrees::new(45.0).abs_separation(Degrees::new(350.0));
    assert_eq!(s.value(), 55.0);

    let arcsec: Arcseconds = Degrees::new(1.0).to();
    assert_eq!(arcsec.value(), 3600.0);
}
