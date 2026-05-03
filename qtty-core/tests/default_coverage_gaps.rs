// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

#![cfg(feature = "std")]

use core::cmp::Ordering;
use core::fmt;

use approx::assert_abs_diff_eq;
use qtty_core::acceleration::{MetersPerSecondSquared, StandardGravities};
use qtty_core::angular::{Degrees, Milliradians, Radians, Turns};
use qtty_core::energy::{Joules, WattHours};
use qtty_core::force::Newtons;
use qtty_core::length::{
    Attometers, Centimeters, Decameters, Decimeters, Exameters, Femtometers, Gigameters,
    Hectometers, Kilometers, Megameters, Meter, Meters, Micrometers, Millimeters, Nanometers,
    Petameters, Picometers, Terameters, Yoctometers, Yottameters, Zeptometers, Zettameters,
};
use qtty_core::mass::{Grams, Kilograms, Tonnes};
use qtty_core::power::{Kilowatts, Watts};
use qtty_core::pressure::{Bars, Hectopascals, Millipascals, Pascals};
use qtty_core::temperature::{Kelvins, Rankines};
use qtty_core::time::{
    Attoseconds, Centiseconds, Centuries, Days, Decades, Decaseconds, Deciseconds, Femtoseconds,
    Fortnights, Gigaseconds, Hectoseconds, Hours, Kiloseconds, Megaseconds, Microseconds,
    Millennia, Milliseconds, Minutes, Nanoseconds, Picoseconds, Seconds, Teraseconds, Weeks, Years,
};
use qtty_core::volume::{
    Centiliters, CubicCentimeters, CubicKilometers, CubicMeters, CubicMillimeters, Deciliters,
    Liters, Microliters, Milliliters,
};
use qtty_core::{Quantity, Scalar};

fn assert_quantity_format_impls<Q>(q: Q, symbol: &str)
where
    Q: Copy + fmt::Display + fmt::LowerExp + fmt::UpperExp,
{
    let display = format!("{}", &q as &dyn fmt::Display);
    let lower = format!("{:e}", &q as &dyn fmt::LowerExp);
    let upper = format!("{:E}", &q as &dyn fmt::UpperExp);

    assert!(
        display.ends_with(&format!(" {symbol}")),
        "display output should end with the unit symbol: {display}"
    );
    assert!(
        lower.ends_with(&format!(" {symbol}")),
        "lower-exp output should end with the unit symbol: {lower}"
    );
    assert!(
        upper.ends_with(&format!(" {symbol}")),
        "upper-exp output should end with the unit symbol: {upper}"
    );
}

#[test]
fn default_length_unit_formatting_hits_proc_macro_impls() {
    assert_quantity_format_impls(Meters::new(1.25), "m");
    assert_quantity_format_impls(Kilometers::new(1.25), "km");
    assert_quantity_format_impls(Centimeters::new(1.25), "cm");
    assert_quantity_format_impls(Millimeters::new(1.25), "mm");
    assert_quantity_format_impls(Micrometers::new(1.25), "μm");
    assert_quantity_format_impls(Nanometers::new(1.25), "nm");
    assert_quantity_format_impls(Picometers::new(1.25), "pm");
    assert_quantity_format_impls(Femtometers::new(1.25), "fm");
    assert_quantity_format_impls(Attometers::new(1.25), "am");
    assert_quantity_format_impls(Zeptometers::new(1.25), "zm");
    assert_quantity_format_impls(Yoctometers::new(1.25), "ym");
    assert_quantity_format_impls(Megameters::new(1.25), "Mm");
    assert_quantity_format_impls(Decimeters::new(1.25), "dm");
    assert_quantity_format_impls(Decameters::new(1.25), "dam");
    assert_quantity_format_impls(Hectometers::new(1.25), "hm");
    assert_quantity_format_impls(Gigameters::new(1.25), "Gm");
    assert_quantity_format_impls(Terameters::new(1.25), "Tm");
    assert_quantity_format_impls(Petameters::new(1.25), "Pm");
    assert_quantity_format_impls(Exameters::new(1.25), "Em");
    assert_quantity_format_impls(Zettameters::new(1.25), "Zm");
    assert_quantity_format_impls(Yottameters::new(1.25), "Ym");
}

#[test]
fn default_time_unit_formatting_hits_proc_macro_impls() {
    assert_quantity_format_impls(Attoseconds::new(1.25), "as");
    assert_quantity_format_impls(Femtoseconds::new(1.25), "fs");
    assert_quantity_format_impls(Picoseconds::new(1.25), "ps");
    assert_quantity_format_impls(Nanoseconds::new(1.25), "ns");
    assert_quantity_format_impls(Microseconds::new(1.25), "µs");
    assert_quantity_format_impls(Milliseconds::new(1.25), "ms");
    assert_quantity_format_impls(Centiseconds::new(1.25), "cs");
    assert_quantity_format_impls(Deciseconds::new(1.25), "ds");
    assert_quantity_format_impls(Seconds::new(1.25), "s");
    assert_quantity_format_impls(Decaseconds::new(1.25), "das");
    assert_quantity_format_impls(Hectoseconds::new(1.25), "hs");
    assert_quantity_format_impls(Kiloseconds::new(1.25), "ks");
    assert_quantity_format_impls(Megaseconds::new(1.25), "Ms");
    assert_quantity_format_impls(Gigaseconds::new(1.25), "Gs");
    assert_quantity_format_impls(Teraseconds::new(1.25), "Ts");
    assert_quantity_format_impls(Minutes::new(1.25), "min");
    assert_quantity_format_impls(Hours::new(1.25), "h");
    assert_quantity_format_impls(Days::new(1.25), "d");
    assert_quantity_format_impls(Weeks::new(1.25), "wk");
    assert_quantity_format_impls(Fortnights::new(1.25), "fn");
    assert_quantity_format_impls(Years::new(1.25), "yr");
    assert_quantity_format_impls(Decades::new(1.25), "dec");
    assert_quantity_format_impls(Centuries::new(1.25), "c");
    assert_quantity_format_impls(Millennia::new(1.25), "mill");
}

#[test]
fn default_named_unit_formatting_hits_proc_macro_impls() {
    assert_quantity_format_impls(Degrees::new(1.25), "°");
    assert_quantity_format_impls(Radians::new(1.25), "rad");
    assert_quantity_format_impls(Milliradians::new(1.25), "mrad");
    assert_quantity_format_impls(Turns::new(1.25), "tr");

    assert_quantity_format_impls(MetersPerSecondSquared::new(1.25), "m/s²");
    assert_quantity_format_impls(StandardGravities::new(1.25), "g₀");

    assert_quantity_format_impls(Joules::new(1.25), "J");
    assert_quantity_format_impls(WattHours::new(1.25), "Wh");

    assert_quantity_format_impls(Newtons::new(1.25), "N");

    assert_quantity_format_impls(Grams::new(1.25), "g");
    assert_quantity_format_impls(Kilograms::new(1.25), "kg");
    assert_quantity_format_impls(Tonnes::new(1.25), "t");

    assert_quantity_format_impls(Watts::new(1.25), "W");
    assert_quantity_format_impls(Kilowatts::new(1.25), "kW");

    assert_quantity_format_impls(Pascals::new(1.25), "Pa");
    assert_quantity_format_impls(Hectopascals::new(1.25), "hPa");
    assert_quantity_format_impls(Millipascals::new(1.25), "mPa");
    assert_quantity_format_impls(Bars::new(1.25), "bar");

    assert_quantity_format_impls(Kelvins::new(1.25), "K");
    assert_quantity_format_impls(Rankines::new(1.25), "°R");

    assert_quantity_format_impls(CubicMeters::new(1.25), "m³");
    assert_quantity_format_impls(CubicKilometers::new(1.25), "km³");
    assert_quantity_format_impls(CubicCentimeters::new(1.25), "cm³");
    assert_quantity_format_impls(CubicMillimeters::new(1.25), "mm³");
    assert_quantity_format_impls(Liters::new(1.25), "L");
    assert_quantity_format_impls(Milliliters::new(1.25), "mL");
    assert_quantity_format_impls(Microliters::new(1.25), "µL");
    assert_quantity_format_impls(Centiliters::new(1.25), "cL");
    assert_quantity_format_impls(Deciliters::new(1.25), "dL");
}

#[test]
fn quantity_helper_branches_are_exercised() {
    let neg = Meters::new(-10.0);
    let pos = Meters::new(4.0);
    assert_eq!(neg.mean(pos).value(), -3.0);

    let raw = Kilometers::new(1.25).erase_unit_raw();
    assert_eq!(raw, 1.25);

    assert_eq!(
        Meters::new(500.0).cmp_unit(&Kilometers::new(1.0)),
        Some(Ordering::Less)
    );

    let wrapped = Degrees::new(-10.0).rem_euclid(360.0);
    assert_abs_diff_eq!(wrapped.value(), 350.0, epsilon = 1e-12);

    let mut scaled = Meters::new(3.0);
    scaled *= 4.0;
    assert_eq!(scaled.value(), 12.0);

    let remainder = Meters::new(10.0) % Meters::new(4.0);
    assert_eq!(remainder.value(), 2.0);

    let five = Quantity::<Meter, i32>::new(5);
    let two = Quantity::<Meter, i32>::new(2);
    assert_eq!(five.min_const(two).value(), 2);
    assert_eq!(two.max_const(five).value(), 5);
}

#[test]
fn scalar_rem_euclid_integer_impl_is_exercised() {
    assert_eq!(<i32 as Scalar>::rem_euclid(-5, 3), 1);
}
