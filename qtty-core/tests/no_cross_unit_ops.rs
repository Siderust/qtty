#![cfg(not(feature = "cross-unit-ops"))]

use core::cmp::Ordering;
use qtty_core::length::{Kilometer, Kilometers, Meter, Meters};
use qtty_core::Quantity;

#[test]
fn from_and_to_still_work_without_cross_unit_ops() {
    let km = Kilometers::new(1.25);

    let m_via_to: Quantity<Meter> = km.to();
    assert!((m_via_to.value() - 1250.0).abs() < 1e-12);

    let m_via_from: Quantity<Meter> = km.into();
    assert!((m_via_from.value() - 1250.0).abs() < 1e-12);

    let km_roundtrip: Quantity<Kilometer> = m_via_from.into();
    assert!((km_roundtrip.value() - 1.25).abs() < 1e-12);
}

#[test]
fn eq_unit_and_cmp_unit_replace_cross_unit_operators() {
    let km = Kilometers::new(2.0);
    let m_less = Meters::new(1_500.0);
    let m_eq = Meters::new(2_000.0);
    let m_more = Meters::new(2_500.0);

    assert!(!km.eq_unit(&m_less));
    assert!(km.eq_unit(&m_eq));
    assert!(!km.eq_unit(&m_more));

    assert_eq!(km.cmp_unit(&m_less), Some(Ordering::Greater));
    assert_eq!(km.cmp_unit(&m_eq), Some(Ordering::Equal));
    assert_eq!(km.cmp_unit(&m_more), Some(Ordering::Less));
}
