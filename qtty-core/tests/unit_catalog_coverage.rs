// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

#![cfg(feature = "std")]

use qtty_core::{Quantity, Unit};

macro_rules! touch_units {
    ($($unit:ident),+ $(,)?) => {{
        $(
            assert!(<$unit as Unit>::RATIO.is_finite());

            let rendered = format!("{}", Quantity::<$unit>::new(1.25));
            assert!(!rendered.is_empty());
        )+
    }};
}

#[test]
fn angular_inventory_units_have_runtime_coverage() {
    use qtty_core::angular::*;

    qtty_core::angular_units!(touch_units);
}

#[cfg(feature = "astro")]
#[test]
fn angular_astro_inventory_units_have_runtime_coverage() {
    use qtty_core::angular::*;

    qtty_core::angular_astro_units!(touch_units);
}

#[cfg(feature = "navigation")]
#[test]
fn angular_navigation_inventory_units_have_runtime_coverage() {
    use qtty_core::angular::*;

    qtty_core::angular_navigation_units!(touch_units);
}

#[test]
fn length_inventory_units_have_runtime_coverage() {
    use qtty_core::length::*;

    qtty_core::length_units!(touch_units);
}

#[cfg(feature = "astro")]
#[test]
fn length_astro_inventory_units_have_runtime_coverage() {
    use qtty_core::length::nominal::*;
    use qtty_core::length::*;

    qtty_core::length_astro_units!(touch_units);
    qtty_core::length_nominal_units!(touch_units);
}

#[cfg(feature = "navigation")]
#[test]
fn length_navigation_inventory_units_have_runtime_coverage() {
    use qtty_core::length::*;

    qtty_core::length_navigation_units!(touch_units);
}

#[cfg(feature = "customary")]
#[test]
fn length_customary_inventory_units_have_runtime_coverage() {
    use qtty_core::length::*;

    qtty_core::length_customary_units!(touch_units);
}

#[cfg(feature = "fundamental-physics")]
#[test]
fn length_fundamental_physics_inventory_units_have_runtime_coverage() {
    use qtty_core::length::*;

    qtty_core::length_fundamental_physics_units!(touch_units);
}

#[test]
fn time_inventory_units_have_runtime_coverage() {
    use qtty_core::time::*;

    qtty_core::time_units!(touch_units);
}

#[cfg(feature = "astro")]
#[test]
fn time_astro_inventory_units_have_runtime_coverage() {
    use qtty_core::time::*;

    qtty_core::time_astro_units!(touch_units);
}

#[cfg(feature = "julian-time")]
#[test]
fn time_julian_inventory_units_have_runtime_coverage() {
    use qtty_core::time::*;

    qtty_core::time_julian_time_units!(touch_units);
}

#[test]
fn mass_inventory_units_have_runtime_coverage() {
    use qtty_core::mass::*;

    qtty_core::mass_units!(touch_units);
}

#[cfg(feature = "astro")]
#[test]
fn mass_astro_inventory_units_have_runtime_coverage() {
    use qtty_core::mass::*;

    qtty_core::mass_astro_units!(touch_units);
}

#[cfg(feature = "customary")]
#[test]
fn mass_customary_inventory_units_have_runtime_coverage() {
    use qtty_core::mass::*;

    qtty_core::mass_customary_units!(touch_units);
}

#[cfg(feature = "fundamental-physics")]
#[test]
fn mass_fundamental_physics_inventory_units_have_runtime_coverage() {
    use qtty_core::mass::*;

    qtty_core::mass_fundamental_physics_units!(touch_units);
}

#[test]
fn area_inventory_units_have_runtime_coverage() {
    use qtty_core::area::*;

    qtty_core::area_units!(touch_units);
}

#[cfg(feature = "customary")]
#[test]
fn area_customary_inventory_units_have_runtime_coverage() {
    use qtty_core::area::*;

    qtty_core::area_customary_units!(touch_units);
}

#[cfg(feature = "land-area")]
#[test]
fn area_land_inventory_units_have_runtime_coverage() {
    use qtty_core::area::*;

    qtty_core::area_land_area_units!(touch_units);
}

#[test]
fn volume_inventory_units_have_runtime_coverage() {
    use qtty_core::volume::*;

    qtty_core::volume_units!(touch_units);
}

#[cfg(feature = "customary")]
#[test]
fn volume_customary_inventory_units_have_runtime_coverage() {
    use qtty_core::volume::*;

    qtty_core::volume_customary_units!(touch_units);
}

#[test]
fn acceleration_inventory_units_have_runtime_coverage() {
    use qtty_core::acceleration::*;

    qtty_core::acceleration_units!(touch_units);
}

#[cfg(feature = "chemistry")]
#[test]
fn amount_inventory_units_have_runtime_coverage() {
    use qtty_core::amount::*;

    qtty_core::amount_units!(touch_units);
}

#[test]
fn energy_inventory_units_have_runtime_coverage() {
    use qtty_core::energy::*;

    qtty_core::energy_units!(touch_units);
}

#[cfg(feature = "fundamental-physics")]
#[test]
fn energy_fundamental_physics_inventory_units_have_runtime_coverage() {
    use qtty_core::energy::*;

    qtty_core::energy_fundamental_physics_units!(touch_units);
}

#[cfg(feature = "customary")]
#[test]
fn energy_customary_inventory_units_have_runtime_coverage() {
    use qtty_core::energy::*;

    qtty_core::energy_customary_units!(touch_units);
}

#[test]
fn force_inventory_units_have_runtime_coverage() {
    use qtty_core::force::*;

    qtty_core::force_units!(touch_units);
}

#[cfg(feature = "fundamental-physics")]
#[test]
fn force_fundamental_physics_inventory_units_have_runtime_coverage() {
    use qtty_core::force::*;

    qtty_core::force_fundamental_physics_units!(touch_units);
}

#[cfg(feature = "customary")]
#[test]
fn force_customary_inventory_units_have_runtime_coverage() {
    use qtty_core::force::*;

    qtty_core::force_customary_units!(touch_units);
}

#[cfg(feature = "frequency")]
#[test]
fn frequency_inventory_units_have_runtime_coverage() {
    use qtty_core::frequency::*;

    qtty_core::frequency_units!(touch_units);
}

#[cfg(feature = "density")]
#[test]
fn density_inventory_units_have_runtime_coverage() {
    use qtty_core::density::*;

    qtty_core::density_units!(touch_units);
}

#[cfg(all(feature = "density", feature = "customary"))]
#[test]
fn density_customary_inventory_units_have_runtime_coverage() {
    use qtty_core::density::*;

    qtty_core::density_customary_units!(touch_units);
}

#[cfg(feature = "electrical")]
#[test]
fn electrical_inventory_units_have_runtime_coverage() {
    use qtty_core::electrical::*;

    qtty_core::ampere_units!(touch_units);
    qtty_core::coulomb_units!(touch_units);
    qtty_core::volt_units!(touch_units);
    qtty_core::ohm_units!(touch_units);
    qtty_core::farad_units!(touch_units);
    qtty_core::henry_units!(touch_units);
    qtty_core::weber_units!(touch_units);
    qtty_core::tesla_units!(touch_units);
}

#[cfg(feature = "photometry")]
#[test]
fn photometry_inventory_units_have_runtime_coverage() {
    use qtty_core::photometry::*;

    qtty_core::candela_units!(touch_units);
    qtty_core::lumen_units!(touch_units);
    qtty_core::lux_units!(touch_units);
}

#[test]
fn power_inventory_units_have_runtime_coverage() {
    use qtty_core::power::*;

    qtty_core::power_units!(touch_units);
}

#[cfg(feature = "customary")]
#[test]
fn power_customary_inventory_units_have_runtime_coverage() {
    use qtty_core::power::*;

    qtty_core::power_customary_units!(touch_units);
}

#[cfg(feature = "fundamental-physics")]
#[test]
fn power_fundamental_physics_inventory_units_have_runtime_coverage() {
    use qtty_core::power::*;

    qtty_core::power_fundamental_physics_units!(touch_units);
}

#[cfg(feature = "astro")]
#[test]
fn power_astro_inventory_units_have_runtime_coverage() {
    use qtty_core::power::*;

    qtty_core::power_astro_units!(touch_units);
}

#[test]
fn pressure_inventory_units_have_runtime_coverage() {
    use qtty_core::pressure::*;

    qtty_core::pressure_units!(touch_units);
}

#[cfg(feature = "customary")]
#[test]
fn pressure_customary_inventory_units_have_runtime_coverage() {
    use qtty_core::pressure::*;

    qtty_core::pressure_customary_units!(touch_units);
}

#[cfg(feature = "radiometry")]
#[test]
fn radiometry_inventory_units_have_runtime_coverage() {
    use qtty_core::radiometry::*;

    qtty_core::radiance_units!(touch_units);
    qtty_core::spectral_radiance_units!(touch_units);
    qtty_core::photon_radiance_units!(touch_units);
    qtty_core::spectral_photon_radiance_units!(touch_units);
    qtty_core::inverse_solid_angle_units!(touch_units);
}

#[test]
fn temperature_inventory_units_have_runtime_coverage() {
    use qtty_core::temperature::*;

    qtty_core::temperature_units!(touch_units);
}
