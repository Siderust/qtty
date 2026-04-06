// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Stable, macro-generated unit arithmetic layer.
//!
//! This module provides the [`UnitDiv`] and [`UnitMul`] traits that control what
//! unit type results from dividing or multiplying two quantities. The blanket
//! `Quantity<N> / Quantity<D>` and `Quantity<A> * Quantity<B>` operator impls
//! delegate to these traits to determine the output unit.
//!
//! Generic "recovery" impls are provided for common structural patterns:
//!
//! - `U / U → Unitless`
//! - `N / Per<N, D> → D`
//! - `Per<N, D> * D → N`
//! - `D * Per<N, D> → N`
//!
//! For all built-in unit pairs (plain marker types), macro-generated fallback
//! impls produce the default composite types:
//!
//! - `A / B → Per<A, B>` (when `A ≠ B`)
//! - `A * B → Prod<A, B>`
//!
//! Downstream crates can use the exported macros
//! [`impl_unit_division_pairs!`], [`impl_unit_multiplication_pairs!`], and
//! [`impl_unit_arithmetic_pairs!`] to register their own custom units into the
//! same fallback tables.

use crate::dimension::{DimDiv, DimMul, Dimension};
use crate::unit::{Per, Unit, Unitless};

// ─────────────────────────────────────────────────────────────────────────────
// Core traits
// ─────────────────────────────────────────────────────────────────────────────

/// Determines the unit type produced by dividing a quantity of unit `Self` by
/// a quantity of unit `Rhs`.
pub trait UnitDiv<Rhs: Unit>: Unit {
    /// The resulting unit type.
    type Output: Unit;
}

/// Determines the unit type produced by multiplying a quantity of unit `Self`
/// by a quantity of unit `Rhs`.
pub trait UnitMul<Rhs: Unit>: Unit {
    /// The resulting unit type.
    type Output: Unit;
}

// ─────────────────────────────────────────────────────────────────────────────
// Generic structural recovery impls
// ─────────────────────────────────────────────────────────────────────────────

// U / U → Unitless
//
// Any unit divided by itself produces a dimensionless result.  This is
// implemented with a blanket impl so it works for *all* units — built-in,
// composite (`Per`, `Prod`), and downstream custom types — without needing
// per-type macro entries.
impl<U: Unit> UnitDiv<U> for U
where
    U::Dim: DimDiv<U::Dim>,
    <U::Dim as DimDiv<U::Dim>>::Output: Dimension,
{
    type Output = Unitless;
}

// N / Per<N, D> → D
impl<N: Unit, D: Unit> UnitDiv<Per<N, D>> for N
where
    N::Dim: DimDiv<D::Dim>,
    <N::Dim as DimDiv<D::Dim>>::Output: Dimension,
    N::Dim: DimDiv<<N::Dim as DimDiv<D::Dim>>::Output>,
    <N::Dim as DimDiv<<N::Dim as DimDiv<D::Dim>>::Output>>::Output: Dimension,
    // Disambiguate from the blanket U/U impl: this impl is only valid when
    // `Per<N, D>` is NOT the same type as `N`. In practice it never is,
    // because `Per` is a distinct wrapper struct.
{
    type Output = D;
}

// Per<N, D> * D → N
impl<N: Unit, D: Unit> UnitMul<D> for Per<N, D>
where
    N::Dim: DimDiv<D::Dim>,
    <N::Dim as DimDiv<D::Dim>>::Output: Dimension,
    <N::Dim as DimDiv<D::Dim>>::Output: DimMul<D::Dim>,
    <<N::Dim as DimDiv<D::Dim>>::Output as DimMul<D::Dim>>::Output: Dimension,
{
    type Output = N;
}

// D * Per<N, D> → N
impl<N: Unit, D: Unit> UnitMul<Per<N, D>> for D
where
    N::Dim: DimDiv<D::Dim>,
    <N::Dim as DimDiv<D::Dim>>::Output: Dimension,
    D::Dim: DimMul<<N::Dim as DimDiv<D::Dim>>::Output>,
    <D::Dim as DimMul<<N::Dim as DimDiv<D::Dim>>::Output>>::Output: Dimension,
{
    type Output = N;
}

// ─────────────────────────────────────────────────────────────────────────────
// Macro-generated fallback tables for built-in units
// ─────────────────────────────────────────────────────────────────────────────

// The macros below generate `UnitDiv` and `UnitMul` impls for every ordered
// pair of distinct built-in unit marker types. This replaces the previous
// blanket impls that produced `Per<N, D>` or `Prod<A, B>` for all pairs
// unconditionally.

/// Generates `UnitDiv` impls for all ordered pairs of distinct units.
///
/// For each pair `(A, B)` where `A ≠ B`, implements `UnitDiv<B> for A`
/// with `Output = Per<A, B>`.
///
/// Self-pairs are skipped because they are covered by the blanket
/// `impl<U> UnitDiv<U> for U { type Output = Unitless; }`.
///
/// # Example
///
/// ```ignore
/// impl_unit_division_pairs!(Meter, Second, Kilogram);
/// // Generates:
/// //   impl UnitDiv<Second>   for Meter    { type Output = Per<Meter, Second>; }
/// //   impl UnitDiv<Kilogram> for Meter    { type Output = Per<Meter, Kilogram>; }
/// //   impl UnitDiv<Meter>    for Second   { type Output = Per<Second, Meter>; }
/// //   impl UnitDiv<Kilogram> for Second   { type Output = Per<Second, Kilogram>; }
/// //   impl UnitDiv<Meter>    for Kilogram { type Output = Per<Kilogram, Meter>; }
/// //   impl UnitDiv<Second>   for Kilogram { type Output = Per<Kilogram, Second>; }
/// ```
#[macro_export]
macro_rules! impl_unit_division_pairs {
    // Base case: single unit, nothing to pair (self-pair covered by blanket).
    ($unit:ty) => {};

    // Recursive case: pair the first unit with every other, then recurse.
    ($first:ty, $($rest:ty),+ $(,)?) => {
        $(
            impl $crate::unit_arithmetic::UnitDiv<$rest> for $first
            where
                <$first as $crate::Unit>::Dim: $crate::DimDiv<<$rest as $crate::Unit>::Dim>,
                <<$first as $crate::Unit>::Dim as $crate::DimDiv<<$rest as $crate::Unit>::Dim>>::Output: $crate::Dimension,
            {
                type Output = $crate::Per<$first, $rest>;
            }

            impl $crate::unit_arithmetic::UnitDiv<$first> for $rest
            where
                <$rest as $crate::Unit>::Dim: $crate::DimDiv<<$first as $crate::Unit>::Dim>,
                <<$rest as $crate::Unit>::Dim as $crate::DimDiv<<$first as $crate::Unit>::Dim>>::Output: $crate::Dimension,
            {
                type Output = $crate::Per<$rest, $first>;
            }
        )+

        $crate::impl_unit_division_pairs!($($rest),+);
    };
}

/// Generates `UnitMul` impls for all ordered pairs of units, **including**
/// self-pairs (`A * A`).
///
/// For each pair `(A, B)`, implements `UnitMul<B> for A` with
/// `Output = Prod<A, B>`, and symmetrically `UnitMul<A> for B` with
/// `Output = Prod<B, A>`.
///
/// For self-pairs (`A * A`), a single `UnitMul<A> for A` is emitted.
///
/// # Example
///
/// ```ignore
/// impl_unit_multiplication_pairs!(Meter, Second);
/// // Generates:
/// //   impl UnitMul<Meter>  for Meter  { type Output = Prod<Meter, Meter>; }
/// //   impl UnitMul<Second> for Second { type Output = Prod<Second, Second>; }
/// //   impl UnitMul<Second> for Meter  { type Output = Prod<Meter, Second>; }
/// //   impl UnitMul<Meter>  for Second { type Output = Prod<Second, Meter>; }
/// ```
#[macro_export]
macro_rules! impl_unit_multiplication_pairs {
    // Base case: single unit – emit the self-pair.
    ($unit:ty) => {
        impl $crate::unit_arithmetic::UnitMul<$unit> for $unit
        where
            <$unit as $crate::Unit>::Dim: $crate::DimMul<<$unit as $crate::Unit>::Dim>,
            <<$unit as $crate::Unit>::Dim as $crate::DimMul<<$unit as $crate::Unit>::Dim>>::Output: $crate::Dimension,
        {
            type Output = $crate::Prod<$unit, $unit>;
        }
    };

    // Recursive case: self-pair for first, cross-pairs with all others, then recurse.
    ($first:ty, $($rest:ty),+ $(,)?) => {
        // Self-pair: first * first
        impl $crate::unit_arithmetic::UnitMul<$first> for $first
        where
            <$first as $crate::Unit>::Dim: $crate::DimMul<<$first as $crate::Unit>::Dim>,
            <<$first as $crate::Unit>::Dim as $crate::DimMul<<$first as $crate::Unit>::Dim>>::Output: $crate::Dimension,
        {
            type Output = $crate::Prod<$first, $first>;
        }

        $(
            impl $crate::unit_arithmetic::UnitMul<$rest> for $first
            where
                <$first as $crate::Unit>::Dim: $crate::DimMul<<$rest as $crate::Unit>::Dim>,
                <<$first as $crate::Unit>::Dim as $crate::DimMul<<$rest as $crate::Unit>::Dim>>::Output: $crate::Dimension,
            {
                type Output = $crate::Prod<$first, $rest>;
            }

            impl $crate::unit_arithmetic::UnitMul<$first> for $rest
            where
                <$rest as $crate::Unit>::Dim: $crate::DimMul<<$first as $crate::Unit>::Dim>,
                <<$rest as $crate::Unit>::Dim as $crate::DimMul<<$first as $crate::Unit>::Dim>>::Output: $crate::Dimension,
            {
                type Output = $crate::Prod<$rest, $first>;
            }
        )+

        $crate::impl_unit_multiplication_pairs!($($rest),+);
    };
}

/// Convenience macro that generates both division and multiplication pair
/// tables for a set of units.
///
/// Equivalent to calling [`impl_unit_division_pairs!`] and
/// [`impl_unit_multiplication_pairs!`] with the same unit list.
#[macro_export]
macro_rules! impl_unit_arithmetic_pairs {
    ($($unit:ty),+ $(,)?) => {
        $crate::impl_unit_division_pairs!($($unit),+);
        $crate::impl_unit_multiplication_pairs!($($unit),+);
    };
}

// ─────────────────────────────────────────────────────────────────────────────
// Built-in unit registration
// ─────────────────────────────────────────────────────────────────────────────

// Register all built-in unit marker types so that cross-unit division and
// multiplication "just work" for the standard catalog.

impl_unit_arithmetic_pairs!(
    // ── Unitless ──────────────────────────────────────────────────────────
    crate::unit::Unitless,
    // ── Length ────────────────────────────────────────────────────────────
    crate::units::length::Meter,
    crate::units::length::Kilometer,
    crate::units::length::Centimeter,
    crate::units::length::Millimeter,
    crate::units::length::Micrometer,
    crate::units::length::Nanometer,
    crate::units::length::Picometer,
    crate::units::length::Femtometer,
    crate::units::length::Attometer,
    crate::units::length::Zeptometer,
    crate::units::length::Yoctometer,
    crate::units::length::Megameter,
    crate::units::length::Decimeter,
    crate::units::length::Decameter,
    crate::units::length::Hectometer,
    crate::units::length::Gigameter,
    crate::units::length::Terameter,
    crate::units::length::Petameter,
    crate::units::length::Exameter,
    crate::units::length::Zettameter,
    crate::units::length::Yottameter,
    crate::units::length::AstronomicalUnit,
    crate::units::length::LightYear,
    crate::units::length::Parsec,
    crate::units::length::Kiloparsec,
    crate::units::length::Megaparsec,
    crate::units::length::Gigaparsec,
    crate::units::length::Inch,
    crate::units::length::Foot,
    crate::units::length::Yard,
    crate::units::length::Mile,
    crate::units::length::NauticalMile,
    crate::units::length::Chain,
    crate::units::length::Rod,
    crate::units::length::Link,
    crate::units::length::Fathom,
    crate::units::length::EarthMeridionalCircumference,
    crate::units::length::EarthEquatorialCircumference,
    crate::units::length::BohrRadius,
    crate::units::length::ClassicalElectronRadius,
    crate::units::length::PlanckLength,
    crate::units::length::ElectronReducedComptonWavelength,
    // ── Length (nominal) ──────────────────────────────────────────────────
    crate::units::length::nominal::SolarRadius,
    crate::units::length::nominal::SolarDiameter,
    crate::units::length::nominal::EarthRadius,
    crate::units::length::nominal::EarthEquatorialRadius,
    crate::units::length::nominal::EarthPolarRadius,
    crate::units::length::nominal::JupiterRadius,
    crate::units::length::nominal::LunarRadius,
    crate::units::length::nominal::LunarDistance,
    // ── Time ──────────────────────────────────────────────────────────────
    crate::units::time::Attosecond,
    crate::units::time::Femtosecond,
    crate::units::time::Picosecond,
    crate::units::time::Nanosecond,
    crate::units::time::Microsecond,
    crate::units::time::Millisecond,
    crate::units::time::Centisecond,
    crate::units::time::Decisecond,
    crate::units::time::Second,
    crate::units::time::Decasecond,
    crate::units::time::Hectosecond,
    crate::units::time::Kilosecond,
    crate::units::time::Megasecond,
    crate::units::time::Gigasecond,
    crate::units::time::Terasecond,
    crate::units::time::Minute,
    crate::units::time::Hour,
    crate::units::time::Day,
    crate::units::time::Week,
    crate::units::time::Fortnight,
    crate::units::time::Year,
    crate::units::time::Decade,
    crate::units::time::Century,
    crate::units::time::Millennium,
    crate::units::time::JulianYear,
    crate::units::time::JulianCentury,
    crate::units::time::SiderealDay,
    crate::units::time::SynodicMonth,
    crate::units::time::SiderealYear,
    // ── Mass ──────────────────────────────────────────────────────────────
    crate::units::mass::Gram,
    crate::units::mass::Yoctogram,
    crate::units::mass::Zeptogram,
    crate::units::mass::Attogram,
    crate::units::mass::Femtogram,
    crate::units::mass::Picogram,
    crate::units::mass::Nanogram,
    crate::units::mass::Microgram,
    crate::units::mass::Milligram,
    crate::units::mass::Centigram,
    crate::units::mass::Decigram,
    crate::units::mass::Decagram,
    crate::units::mass::Hectogram,
    crate::units::mass::Kilogram,
    crate::units::mass::Megagram,
    crate::units::mass::Gigagram,
    crate::units::mass::Teragram,
    crate::units::mass::Petagram,
    crate::units::mass::Exagram,
    crate::units::mass::Zettagram,
    crate::units::mass::Yottagram,
    crate::units::mass::Tonne,
    crate::units::mass::Carat,
    crate::units::mass::Grain,
    crate::units::mass::Pound,
    crate::units::mass::Ounce,
    crate::units::mass::Stone,
    crate::units::mass::ShortTon,
    crate::units::mass::LongTon,
    crate::units::mass::AtomicMassUnit,
    crate::units::mass::SolarMass,
    // ── Angular ───────────────────────────────────────────────────────────
    crate::units::angular::Degree,
    crate::units::angular::Radian,
    crate::units::angular::Milliradian,
    crate::units::angular::Arcminute,
    crate::units::angular::Arcsecond,
    crate::units::angular::MilliArcsecond,
    crate::units::angular::MicroArcsecond,
    crate::units::angular::Gradian,
    crate::units::angular::Turn,
    crate::units::angular::HourAngle,
    // ── Power ─────────────────────────────────────────────────────────────
    crate::units::power::Watt,
    crate::units::power::Yoctowatt,
    crate::units::power::Zeptowatt,
    crate::units::power::Attowatt,
    crate::units::power::Femtowatt,
    crate::units::power::Picowatt,
    crate::units::power::Nanowatt,
    crate::units::power::Microwatt,
    crate::units::power::Milliwatt,
    crate::units::power::Deciwatt,
    crate::units::power::Decawatt,
    crate::units::power::Hectowatt,
    crate::units::power::Kilowatt,
    crate::units::power::Megawatt,
    crate::units::power::Gigawatt,
    crate::units::power::Terawatt,
    crate::units::power::Petawatt,
    crate::units::power::Exawatt,
    crate::units::power::Zettawatt,
    crate::units::power::Yottawatt,
    crate::units::power::ErgPerSecond,
    crate::units::power::HorsepowerMetric,
    crate::units::power::HorsepowerElectric,
    crate::units::power::SolarLuminosity,
    // ── Area ──────────────────────────────────────────────────────────────
    crate::units::area::SquareMeter,
    crate::units::area::SquareKilometer,
    crate::units::area::SquareCentimeter,
    crate::units::area::SquareMillimeter,
    crate::units::area::Hectare,
    crate::units::area::Are,
    crate::units::area::SquareInch,
    crate::units::area::SquareFoot,
    crate::units::area::SquareYard,
    crate::units::area::SquareMile,
    crate::units::area::Acre,
    // ── Volume ────────────────────────────────────────────────────────────
    crate::units::volume::CubicMeter,
    crate::units::volume::CubicKilometer,
    crate::units::volume::CubicCentimeter,
    crate::units::volume::CubicMillimeter,
    crate::units::volume::Liter,
    crate::units::volume::Milliliter,
    crate::units::volume::Microliter,
    crate::units::volume::Centiliter,
    crate::units::volume::Deciliter,
    crate::units::volume::CubicInch,
    crate::units::volume::CubicFoot,
    crate::units::volume::UsGallon,
    crate::units::volume::UsFluidOunce
);
