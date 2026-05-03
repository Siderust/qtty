// SPDX-License-Identifier: BSD-3-Clause
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
//! For all built-in unit pairs (plain marker types), fallback impls produce the
//! default composite types:
//!
//! - `A / B → Per<A, B>` (when `A ≠ B`)
//! - `A * B → Prod<A, B>`
//!
//! Downstream crates can use the exported macros
//! [`impl_unit_division_pairs!`], [`impl_unit_multiplication_pairs!`],
//! [`impl_unit_arithmetic_pairs!`], and their `*_between!` variants to
//! register their own custom units into the same fallback tables without
//! regenerating built-in/built-in impls.

use crate::dimension::{DimDiv, DimMul, Dimension};
use crate::quantity::Quantity;
use crate::scalar::Scalar;
use crate::unit::{Per, Prod, Unit};

// ─────────────────────────────────────────────────────────────────────────────
// Core traits
// ─────────────────────────────────────────────────────────────────────────────

/// Determines the unit type produced by dividing a quantity of unit `Self` by
/// a quantity of unit `Rhs`.
///
/// When `Self == Rhs` (same-unit division), `Output` is [`SameDivOutput`], which
/// signals that the `Div` operator should return the raw scalar `S` rather than
/// a wrapped `Quantity`.  For all other unit pairs, `Output` is a `Unit` type and
/// the result is a `Quantity<Output, S>`.
pub trait UnitDiv<Rhs: Unit>: Unit {
    /// The resulting type token.  Either a [`Unit`] (wrapped in `Quantity` by
    /// the `Div` impl) or [`SameDivOutput`] (unwrapped to the raw scalar).
    type Output;
}

/// Determines the unit type produced by multiplying a quantity of unit `Self`
/// by a quantity of unit `Rhs`.
pub trait UnitMul<Rhs: Unit>: Unit {
    /// The resulting unit type.
    type Output: Unit;
}

/// Inverse of squaring at the type level: maps a "squared" unit back to its
/// scalar root unit.
///
/// Implemented blanketly for [`Prod<U, U>`](Prod), so any unit that arises as
/// the dimensional square of another unit (e.g. `Prod<Meter, Meter>` ≡
/// `SquareMeter`) carries a `Root = U` and can be square-rooted via
/// [`Quantity::sqrt`](crate::Quantity::sqrt).
///
/// This is the dimensionally correct inverse of the `Quantity<U> *
/// Quantity<U> -> Quantity<Prod<U, U>>` `Mul` impl provided through
/// [`UnitMul`].
///
/// # Note
///
/// The blanket impl deliberately only covers the **symmetric** product
/// `Prod<U, U>`; mixed products like `Prod<Meter, Second>` have no
/// well-defined square root in this system.
pub trait UnitSqrt: Unit {
    /// The unit obtained by taking the dimensional square root of `Self`.
    type Root: Unit;
}

impl<U: Unit> UnitSqrt for Prod<U, U>
where
    U::Dim: DimMul<U::Dim>,
    <U::Dim as DimMul<U::Dim>>::Output: Dimension,
{
    type Root = U;
}

// Marker for plain built-in units. This lets built-in multiplication use a
// single generic impl instead of an O(n²) generated impl table.
pub(crate) trait BuiltinUnit: Unit {}

// ─────────────────────────────────────────────────────────────────────────────
// Same-unit division output marker
// ─────────────────────────────────────────────────────────────────────────────

/// Marker returned by `UnitDiv<U>` when the numerator and denominator units
/// are identical.  It is **not** a `Unit`; the `Div` impl uses it to produce
/// the raw scalar `S` instead of a `Quantity`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SameDivOutput;

// ─────────────────────────────────────────────────────────────────────────────
// Dispatch trait: maps a UnitDiv::Output to the final Div::Output type
// ─────────────────────────────────────────────────────────────────────────────

/// Converts a `UnitDiv::Output` token into the concrete type returned by `Div`.
///
/// - `SameDivOutput` → `S`  (raw scalar; cancellation)
/// - Any `U: Unit`     → `Quantity<U, S>`
///
/// Both implementations are coherent: `SameDivOutput` does not implement
/// `Unit`, so the two impls are provably disjoint under Rust's orphan rules.
pub trait QuantityDivOutput<S: Scalar> {
    /// The type that `Div` will return.
    type Output;
    /// Wraps (or passes through) a raw scalar into the output type.
    fn wrap(v: S) -> Self::Output;
}

impl<S: Scalar> QuantityDivOutput<S> for SameDivOutput {
    type Output = S;
    #[inline]
    fn wrap(v: S) -> S {
        v
    }
}

impl<U: Unit, S: Scalar> QuantityDivOutput<S> for U {
    type Output = Quantity<U, S>;
    #[inline]
    fn wrap(v: S) -> Quantity<U, S> {
        Quantity::new(v)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Generic structural recovery impls
// ─────────────────────────────────────────────────────────────────────────────

// U / U → SameDivOutput  (cancellation — the Div impl unwraps this to S)
//
// Any unit divided by itself produces the raw scalar.  This blanket impl covers
// all units — built-in, composite (`Per`, `Prod`), and downstream custom types.
impl<U: Unit> UnitDiv<U> for U
where
    U::Dim: DimDiv<U::Dim>,
    <U::Dim as DimDiv<U::Dim>>::Output: Dimension,
{
    type Output = SameDivOutput;
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

// Built-in plain units multiply to a `Prod<A, B>`. Division still uses an
// explicit pair table because `U / U -> Unitless` overlaps with a blanket
// `A / B -> Per<A, B>` impl on stable Rust.
impl<A: BuiltinUnit, B: BuiltinUnit> UnitMul<B> for A
where
    A::Dim: DimMul<B::Dim>,
    <A::Dim as DimMul<B::Dim>>::Output: Dimension,
{
    type Output = Prod<A, B>;
}

// ─────────────────────────────────────────────────────────────────────────────
// Fallback registration for plain units
// ─────────────────────────────────────────────────────────────────────────────

// The macros below generate `UnitDiv` and `UnitMul` impls for every ordered
// pair of distinct marker types. Built-in units use the division table only;
// multiplication uses the blanket `BuiltinUnit` impl above.

/// Generates `UnitDiv` impls for all ordered pairs of distinct units.
///
/// For each pair `(A, B)` where `A ≠ B`, implements `UnitDiv<B> for A`
/// with `Output = Per<A, B>`.
///
/// Self-pairs are skipped because they are covered by the blanket
/// `impl<U> UnitDiv<U> for U { type Output = SameDivOutput; }` which returns the raw scalar.
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

/// Generates `UnitDiv` impls between every unit in the **extra** group and
/// every unit in the **base** group, plus all intra-extra pairs.
///
/// This does **not** regenerate intra-base pairs, which avoids conflicting with
/// existing registrations for built-in units.
///
/// # Example
///
/// ```ignore
/// impl_unit_division_pairs_between!(Meter, Kilometer; Smoot, Furlong);
/// // Generates:
/// //   Meter / Smoot, Smoot / Meter
/// //   Kilometer / Smoot, Smoot / Kilometer
/// //   Meter / Furlong, Furlong / Meter
/// //   Kilometer / Furlong, Furlong / Kilometer
/// //   Smoot / Furlong, Furlong / Smoot
/// ```
#[macro_export]
macro_rules! impl_unit_division_pairs_between {
    ($($base:ty),+; $($extra:ty),+ $(,)?) => {
        $crate::__impl_div_pairs_each_extra_to_bases!({$($base),+} $($extra),+);
        $crate::impl_unit_division_pairs!($($extra),+);
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

/// Generates `UnitMul` impls between every unit in the **extra** group and
/// every unit in the **base** group, plus all intra-extra pairs.
///
/// This does **not** regenerate intra-base pairs, which avoids conflicting with
/// existing registrations for built-in units.
#[macro_export]
macro_rules! impl_unit_multiplication_pairs_between {
    ($($base:ty),+; $($extra:ty),+ $(,)?) => {
        $crate::__impl_mul_pairs_each_extra_to_bases!({$($base),+} $($extra),+);
        $crate::impl_unit_multiplication_pairs!($($extra),+);
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

/// Convenience macro that generates both division and multiplication impls
/// between an existing **base** group and a new **extra** group.
///
/// Equivalent to calling [`impl_unit_division_pairs_between!`] and
/// [`impl_unit_multiplication_pairs_between!`] with the same arguments.
#[macro_export]
macro_rules! impl_unit_arithmetic_pairs_between {
    ($($base:ty),+; $($extra:ty),+ $(,)?) => {
        $crate::impl_unit_division_pairs_between!($($base),+; $($extra),+);
        $crate::impl_unit_multiplication_pairs_between!($($base),+; $($extra),+);
    };
}

/// Hidden helper for [`impl_unit_division_pairs_between!`].
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_div_pairs_each_extra_to_bases {
    // Base case: single extra remaining.
    ({$($base:ty),+} $extra:ty) => {
        $(
            impl $crate::unit_arithmetic::UnitDiv<$extra> for $base
            where
                <$base as $crate::Unit>::Dim: $crate::DimDiv<<$extra as $crate::Unit>::Dim>,
                <<$base as $crate::Unit>::Dim as $crate::DimDiv<<$extra as $crate::Unit>::Dim>>::Output: $crate::Dimension,
            {
                type Output = $crate::Per<$base, $extra>;
            }

            impl $crate::unit_arithmetic::UnitDiv<$base> for $extra
            where
                <$extra as $crate::Unit>::Dim: $crate::DimDiv<<$base as $crate::Unit>::Dim>,
                <<$extra as $crate::Unit>::Dim as $crate::DimDiv<<$base as $crate::Unit>::Dim>>::Output: $crate::Dimension,
            {
                type Output = $crate::Per<$extra, $base>;
            }
        )+
    };
    // Recursive case: peel the first extra, recurse on the rest.
    ({$($base:ty),+} $first:ty, $($rest:ty),+) => {
        $(
            impl $crate::unit_arithmetic::UnitDiv<$first> for $base
            where
                <$base as $crate::Unit>::Dim: $crate::DimDiv<<$first as $crate::Unit>::Dim>,
                <<$base as $crate::Unit>::Dim as $crate::DimDiv<<$first as $crate::Unit>::Dim>>::Output: $crate::Dimension,
            {
                type Output = $crate::Per<$base, $first>;
            }

            impl $crate::unit_arithmetic::UnitDiv<$base> for $first
            where
                <$first as $crate::Unit>::Dim: $crate::DimDiv<<$base as $crate::Unit>::Dim>,
                <<$first as $crate::Unit>::Dim as $crate::DimDiv<<$base as $crate::Unit>::Dim>>::Output: $crate::Dimension,
            {
                type Output = $crate::Per<$first, $base>;
            }
        )+

        $crate::__impl_div_pairs_each_extra_to_bases!({$($base),+} $($rest),+);
    };
}

/// Hidden helper for [`impl_unit_multiplication_pairs_between!`].
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_mul_pairs_each_extra_to_bases {
    // Base case: single extra remaining.
    ({$($base:ty),+} $extra:ty) => {
        $(
            impl $crate::unit_arithmetic::UnitMul<$extra> for $base
            where
                <$base as $crate::Unit>::Dim: $crate::DimMul<<$extra as $crate::Unit>::Dim>,
                <<$base as $crate::Unit>::Dim as $crate::DimMul<<$extra as $crate::Unit>::Dim>>::Output: $crate::Dimension,
            {
                type Output = $crate::Prod<$base, $extra>;
            }

            impl $crate::unit_arithmetic::UnitMul<$base> for $extra
            where
                <$extra as $crate::Unit>::Dim: $crate::DimMul<<$base as $crate::Unit>::Dim>,
                <<$extra as $crate::Unit>::Dim as $crate::DimMul<<$base as $crate::Unit>::Dim>>::Output: $crate::Dimension,
            {
                type Output = $crate::Prod<$extra, $base>;
            }
        )+
    };
    // Recursive case: peel the first extra, recurse on the rest.
    ({$($base:ty),+} $first:ty, $($rest:ty),+) => {
        $(
            impl $crate::unit_arithmetic::UnitMul<$first> for $base
            where
                <$base as $crate::Unit>::Dim: $crate::DimMul<<$first as $crate::Unit>::Dim>,
                <<$base as $crate::Unit>::Dim as $crate::DimMul<<$first as $crate::Unit>::Dim>>::Output: $crate::Dimension,
            {
                type Output = $crate::Prod<$base, $first>;
            }

            impl $crate::unit_arithmetic::UnitMul<$base> for $first
            where
                <$first as $crate::Unit>::Dim: $crate::DimMul<<$base as $crate::Unit>::Dim>,
                <<$first as $crate::Unit>::Dim as $crate::DimMul<<$base as $crate::Unit>::Dim>>::Output: $crate::Dimension,
            {
                type Output = $crate::Prod<$first, $base>;
            }
        )+

        $crate::__impl_mul_pairs_each_extra_to_bases!({$($base),+} $($rest),+);
    };
}

// ─────────────────────────────────────────────────────────────────────────────
// Built-in unit registration
// ─────────────────────────────────────────────────────────────────────────────

// Register all built-in unit marker types so that cross-unit division and
// multiplication "just work" for the standard catalog.

macro_rules! register_builtin_units {
    ($($unit:ty),+ $(,)?) => {
        $(
            impl BuiltinUnit for $unit {}
        )+

        impl_unit_division_pairs!($($unit),+);
    };
}

/// Generate cross-group `UnitDiv` impls between two disjoint sets of units,
/// plus intra-group pairs within the `extra` set.
///
/// `register_builtin_units_extend!(base1, base2, ...; extra1, extra2, ...)`
/// produces:
/// - `impl BuiltinUnit` for each extra unit
/// - all-pairs `UnitDiv` within the extras
/// - cross-pairs between every base and every extra
#[cfg(any(
    feature = "astro",
    feature = "julian-time",
    feature = "customary",
    feature = "navigation",
    feature = "fundamental-physics",
    feature = "land-area",
))]
macro_rules! register_builtin_units_extend {
    ($($base:ty),+; $($extra:ty),+) => {
        // Mark each extension type as built-in.
        $(impl BuiltinUnit for $extra {})+

        $crate::impl_unit_division_pairs_between!($($base),+; $($extra),+);
    };
}

/// Helper: invokes `$callback!` with the full list of always-available (base)
/// unit types so that the list is written in exactly one place.
macro_rules! with_base_units {
    ($callback:ident) => {
        $callback!(
            // ── Length (metric SI) ────────────────────────────────────────────────
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
            // ── Time (SI + calendar) ──────────────────────────────────────────────
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
            // ── Mass (SI + tonne) ─────────────────────────────────────────────────
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
            // ── Angular (base) ────────────────────────────────────────────────────
            crate::units::angular::Degree,
            crate::units::angular::Radian,
            crate::units::angular::Milliradian,
            crate::units::angular::Turn,
            // ── Power (SI watts) ──────────────────────────────────────────────────
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
            // ── Area (metric) ─────────────────────────────────────────────────────
            crate::units::area::SquareMeter,
            crate::units::area::SquareKilometer,
            crate::units::area::SquareCentimeter,
            crate::units::area::SquareMillimeter,
            // ── Volume (metric + litre) ───────────────────────────────────────────
            crate::units::volume::CubicMeter,
            crate::units::volume::CubicKilometer,
            crate::units::volume::CubicCentimeter,
            crate::units::volume::CubicMillimeter,
            crate::units::volume::Liter,
            crate::units::volume::Milliliter,
            crate::units::volume::Microliter,
            crate::units::volume::Centiliter,
            crate::units::volume::Deciliter,
            // ── Acceleration ──────────────────────────────────────────────────────
            crate::units::acceleration::MeterPerSecondSquared,
            crate::units::acceleration::StandardGravity,
            // ── Force (SI newtons) ────────────────────────────────────────────────
            crate::units::force::Newton,
            crate::units::force::Micronewton,
            crate::units::force::Millinewton,
            crate::units::force::Kilonewton,
            crate::units::force::Meganewton,
            crate::units::force::Giganewton,
            // ── Energy (SI joules + watt-hours) ──────────────────────────────────
            crate::units::energy::Joule,
            crate::units::energy::Picojoule,
            crate::units::energy::Nanojoule,
            crate::units::energy::Microjoule,
            crate::units::energy::Millijoule,
            crate::units::energy::Kilojoule,
            crate::units::energy::Megajoule,
            crate::units::energy::Gigajoule,
            crate::units::energy::Terajoule,
            crate::units::energy::WattHour,
            crate::units::energy::KilowattHour,
            // ── Pressure (SI pascals + bar) ───────────────────────────────────────
            crate::units::pressure::Pascal,
            crate::units::pressure::Millipascal,
            crate::units::pressure::Hectopascal,
            crate::units::pressure::Kilopascal,
            crate::units::pressure::Megapascal,
            crate::units::pressure::Gigapascal,
            crate::units::pressure::Bar,
            // ── Temperature (SI kelvin + Rankine) ────────────────────────────────
            crate::units::temperature::Kelvin,
            crate::units::temperature::Rankine,
            // ── Solid angle (composed from base angular units) ────────────────────
            crate::units::solid_angle::SquareDegree,
            crate::units::solid_angle::Steradian,
            crate::units::solid_angle::SquareMilliradian
        );
    };
}

// Register always-available (base) units.
with_base_units!(register_builtin_units);

// ── Feature-gated unit extensions ────────────────────────────────────────────
// Each block extends the base registration with BuiltinUnit + cross-division
// pairs against all base units.

#[cfg(feature = "astro")]
macro_rules! extend_with_astro {
    ($($base:ty),+) => {
        register_builtin_units_extend!(
            $($base),+;
            crate::units::length::AstronomicalUnit,
            crate::units::length::LightYear,
            crate::units::length::Parsec,
            crate::units::length::Kiloparsec,
            crate::units::length::Megaparsec,
            crate::units::length::Gigaparsec,
            crate::units::length::nominal::SolarRadius,
            crate::units::length::nominal::SolarDiameter,
            crate::units::length::nominal::EarthRadius,
            crate::units::length::nominal::EarthEquatorialRadius,
            crate::units::length::nominal::EarthPolarRadius,
            crate::units::length::nominal::JupiterRadius,
            crate::units::length::nominal::LunarRadius,
            crate::units::length::nominal::LunarDistance,
            crate::units::time::SiderealDay,
            crate::units::time::SynodicMonth,
            crate::units::time::SiderealYear,
            crate::units::mass::SolarMass,
            crate::units::angular::Arcminute,
            crate::units::angular::Arcsecond,
            crate::units::angular::MilliArcsecond,
            crate::units::angular::MicroArcsecond,
            crate::units::angular::HourAngle,
            crate::units::power::SolarLuminosity,
            crate::units::solid_angle::SquareArcminute,
            crate::units::solid_angle::SquareArcsecond
        );
    };
}
#[cfg(feature = "astro")]
with_base_units!(extend_with_astro);

#[cfg(feature = "julian-time")]
macro_rules! extend_with_julian_time {
    ($($base:ty),+) => {
        register_builtin_units_extend!(
            $($base),+;
            crate::units::time::JulianYear,
            crate::units::time::JulianCentury
        );
    };
}
#[cfg(feature = "julian-time")]
with_base_units!(extend_with_julian_time);

#[cfg(feature = "customary")]
macro_rules! extend_with_customary {
    ($($base:ty),+) => {
        register_builtin_units_extend!(
            $($base),+;
            crate::units::length::Inch,
            crate::units::length::Foot,
            crate::units::length::Yard,
            crate::units::length::Mile,
            crate::units::mass::Carat,
            crate::units::mass::Grain,
            crate::units::mass::Pound,
            crate::units::mass::Ounce,
            crate::units::mass::Stone,
            crate::units::mass::ShortTon,
            crate::units::mass::LongTon,
            crate::units::power::HorsepowerMetric,
            crate::units::power::HorsepowerElectric,
            crate::units::area::SquareInch,
            crate::units::area::SquareFoot,
            crate::units::area::SquareYard,
            crate::units::area::SquareMile,
            crate::units::volume::CubicInch,
            crate::units::volume::CubicFoot,
            crate::units::volume::UsGallon,
            crate::units::volume::UsFluidOunce,
            crate::units::force::PoundForce,
            crate::units::energy::Calorie,
            crate::units::energy::Kilocalorie,
            crate::units::energy::BritishThermalUnit,
            crate::units::energy::Therm,
            crate::units::pressure::Atmosphere,
            crate::units::pressure::Torr,
            crate::units::pressure::MillimeterOfMercury,
            crate::units::pressure::PoundPerSquareInch,
            crate::units::pressure::InchOfMercury
        );
    };
}
#[cfg(feature = "customary")]
with_base_units!(extend_with_customary);

#[cfg(feature = "navigation")]
macro_rules! extend_with_navigation {
    ($($base:ty),+) => {
        register_builtin_units_extend!(
            $($base),+;
            crate::units::length::NauticalMile,
            crate::units::length::Chain,
            crate::units::length::Rod,
            crate::units::length::Link,
            crate::units::length::Fathom,
            crate::units::length::EarthMeridionalCircumference,
            crate::units::length::EarthEquatorialCircumference,
            crate::units::angular::Gradian
        );
    };
}
#[cfg(feature = "navigation")]
with_base_units!(extend_with_navigation);

#[cfg(feature = "fundamental-physics")]
macro_rules! extend_with_fundamental_physics {
    ($($base:ty),+) => {
        register_builtin_units_extend!(
            $($base),+;
            crate::units::length::BohrRadius,
            crate::units::length::ClassicalElectronRadius,
            crate::units::length::PlanckLength,
            crate::units::length::ElectronReducedComptonWavelength,
            crate::units::mass::AtomicMassUnit,
            crate::units::power::ErgPerSecond,
            crate::units::force::Dyne,
            crate::units::energy::Erg,
            crate::units::energy::Electronvolt,
            crate::units::energy::Kiloelectronvolt,
            crate::units::energy::Megaelectronvolt
        );
    };
}
#[cfg(feature = "fundamental-physics")]
with_base_units!(extend_with_fundamental_physics);

#[cfg(feature = "land-area")]
macro_rules! extend_with_land_area {
    ($($base:ty),+) => {
        register_builtin_units_extend!(
            $($base),+;
            crate::units::area::Hectare,
            crate::units::area::Are,
            crate::units::area::Acre
        );
    };
}
#[cfg(feature = "land-area")]
with_base_units!(extend_with_land_area);

#[cfg(feature = "radiometry")]
macro_rules! extend_with_radiometry {
    ($($base:ty),+) => {
        register_builtin_units_extend!(
            $($base),+;
            crate::units::radiometry::WattPerSquareMeterSteradian,
            crate::units::radiometry::ErgPerSecondSquareCentimeterSteradian,
            crate::units::radiometry::WattPerSquareMeterSteradianMeter,
            crate::units::radiometry::WattPerSquareMeterSteradianNanometer,
            crate::units::radiometry::ErgPerSecondSquareCentimeterSteradianAngstrom,
            crate::units::radiometry::PhotonPerSquareMeterSecondSteradian,
            crate::units::radiometry::PhotonPerSquareCentimeterSecondSteradian,
            crate::units::radiometry::PhotonPerSquareCentimeterNanosecondSteradian,
            crate::units::radiometry::PhotonPerSquareMeterSecondSteradianMeter,
            crate::units::radiometry::PhotonPerSquareCentimeterSecondSteradianAngstrom,
            crate::units::radiometry::PhotonPerSquareCentimeterSecondSteradianNanometer,
            crate::units::radiometry::PhotonPerSquareCentimeterNanosecondSteradianNanometer,
            crate::units::radiometry::S10
        );
    };
}
#[cfg(feature = "radiometry")]
with_base_units!(extend_with_radiometry);

#[cfg(feature = "frequency")]
macro_rules! extend_with_frequency {
    ($($base:ty),+) => {
        register_builtin_units_extend!(
            $($base),+;
            crate::units::frequency::Hertz,
            crate::units::frequency::Millihertz,
            crate::units::frequency::Kilohertz,
            crate::units::frequency::Megahertz,
            crate::units::frequency::Gigahertz,
            crate::units::frequency::Terahertz
        );
    };
}
#[cfg(feature = "frequency")]
with_base_units!(extend_with_frequency);

#[cfg(feature = "chemistry")]
macro_rules! extend_with_chemistry {
    ($($base:ty),+) => {
        register_builtin_units_extend!(
            $($base),+;
            crate::units::amount::Nanomole,
            crate::units::amount::Micromole,
            crate::units::amount::Millimole,
            crate::units::amount::Mole,
            crate::units::amount::Kilomole
        );
    };
}
#[cfg(feature = "chemistry")]
with_base_units!(extend_with_chemistry);

#[cfg(feature = "electrical")]
macro_rules! extend_with_electrical {
    ($($base:ty),+) => {
        register_builtin_units_extend!(
            $($base),+;
            crate::units::electrical::Ampere,
            crate::units::electrical::Microampere,
            crate::units::electrical::Milliampere,
            crate::units::electrical::Kiloampere,
            crate::units::electrical::Coulomb,
            crate::units::electrical::Millicoulomb,
            crate::units::electrical::Microcoulomb,
            crate::units::electrical::Kilocoulomb,
            crate::units::electrical::Volt,
            crate::units::electrical::Microvolt,
            crate::units::electrical::Millivolt,
            crate::units::electrical::Kilovolt,
            crate::units::electrical::Megavolt,
            crate::units::electrical::Ohm,
            crate::units::electrical::Milliohm,
            crate::units::electrical::Kilohm,
            crate::units::electrical::Megaohm,
            crate::units::electrical::Farad,
            crate::units::electrical::Picofarad,
            crate::units::electrical::Nanofarad,
            crate::units::electrical::Microfarad,
            crate::units::electrical::Millifarad,
            crate::units::electrical::Henry,
            crate::units::electrical::Microhenry,
            crate::units::electrical::Millihenry,
            crate::units::electrical::Weber,
            crate::units::electrical::Milliweber,
            crate::units::electrical::Tesla,
            crate::units::electrical::Microtesla,
            crate::units::electrical::Millitesla
        );
    };
}
#[cfg(feature = "electrical")]
with_base_units!(extend_with_electrical);

#[cfg(feature = "density")]
macro_rules! extend_with_density {
    ($($base:ty),+) => {
        register_builtin_units_extend!(
            $($base),+;
            crate::units::density::KilogramPerCubicMeter,
            crate::units::density::GramPerCubicCentimeter,
            crate::units::density::GramPerMilliliter
        );
    };
}
#[cfg(feature = "density")]
with_base_units!(extend_with_density);

#[cfg(all(feature = "density", feature = "customary"))]
macro_rules! extend_density_with_customary {
    ($($base:ty),+) => {
        register_builtin_units_extend!(
            $($base),+;
            crate::units::density::PoundPerCubicFoot
        );
    };
}
#[cfg(all(feature = "density", feature = "customary"))]
with_base_units!(extend_density_with_customary);

#[cfg(feature = "photometry")]
macro_rules! extend_with_photometry {
    ($($base:ty),+) => {
        register_builtin_units_extend!(
            $($base),+;
            crate::units::photometry::Candela,
            crate::units::photometry::Lumen,
            crate::units::photometry::Millilumen,
            crate::units::photometry::Kilolumen,
            crate::units::photometry::Lux,
            crate::units::photometry::Millilux,
            crate::units::photometry::Kilolux
        );
    };
}
#[cfg(feature = "photometry")]
with_base_units!(extend_with_photometry);

// ── Per-feature "as-base" UnitDiv helpers ─────────────────────────────────────
//
// Each macro below holds ONE optional feature's complete unit list.  When two
// optional features are both enabled the macro for the *larger* family is
// invoked with the *smaller* family's units as arguments, so each unit list
// lives in exactly one place rather than being copy-pasted into every pairing.
//
// To add a new optional unit family F:
//   1. Define `__impl_div_pairs_with_F_as_base!` here with F's full unit list.
//   2. Add one `#[cfg(all(feature = "F", feature = "X"))]` call per existing
//      family X, invoking whichever family's macro covers the larger set.
// Nothing else in this file needs to change.

#[cfg(feature = "astro")]
macro_rules! __impl_div_pairs_with_astro_as_base {
    ($($extra:ty),+ $(,)?) => {
        crate::__impl_div_pairs_each_extra_to_bases!(
            {
                crate::units::length::AstronomicalUnit,
                crate::units::length::LightYear,
                crate::units::length::Parsec,
                crate::units::length::Kiloparsec,
                crate::units::length::Megaparsec,
                crate::units::length::Gigaparsec,
                crate::units::length::nominal::SolarRadius,
                crate::units::length::nominal::SolarDiameter,
                crate::units::length::nominal::EarthRadius,
                crate::units::length::nominal::EarthEquatorialRadius,
                crate::units::length::nominal::EarthPolarRadius,
                crate::units::length::nominal::JupiterRadius,
                crate::units::length::nominal::LunarRadius,
                crate::units::length::nominal::LunarDistance,
                crate::units::time::SiderealDay,
                crate::units::time::SynodicMonth,
                crate::units::time::SiderealYear,
                crate::units::mass::SolarMass,
                crate::units::angular::Arcminute,
                crate::units::angular::Arcsecond,
                crate::units::angular::MilliArcsecond,
                crate::units::angular::MicroArcsecond,
                crate::units::angular::HourAngle,
                crate::units::power::SolarLuminosity
            }
            $($extra),+
        );
    };
}

#[cfg(feature = "customary")]
macro_rules! __impl_div_pairs_with_customary_as_base {
    ($($extra:ty),+ $(,)?) => {
        crate::__impl_div_pairs_each_extra_to_bases!(
            {
                crate::units::length::Inch,
                crate::units::length::Foot,
                crate::units::length::Yard,
                crate::units::length::Mile,
                crate::units::mass::Carat,
                crate::units::mass::Grain,
                crate::units::mass::Pound,
                crate::units::mass::Ounce,
                crate::units::mass::Stone,
                crate::units::mass::ShortTon,
                crate::units::mass::LongTon,
                crate::units::power::HorsepowerMetric,
                crate::units::power::HorsepowerElectric,
                crate::units::area::SquareInch,
                crate::units::area::SquareFoot,
                crate::units::area::SquareYard,
                crate::units::area::SquareMile,
                crate::units::volume::CubicInch,
                crate::units::volume::CubicFoot,
                crate::units::volume::UsGallon,
                crate::units::volume::UsFluidOunce,
                crate::units::force::PoundForce,
                crate::units::energy::Calorie,
                crate::units::energy::Kilocalorie
            }
            $($extra),+
        );
    };
}

#[cfg(feature = "fundamental-physics")]
macro_rules! __impl_div_pairs_with_fundamental_physics_as_base {
    ($($extra:ty),+ $(,)?) => {
        crate::__impl_div_pairs_each_extra_to_bases!(
            {
                crate::units::length::BohrRadius,
                crate::units::length::ClassicalElectronRadius,
                crate::units::length::PlanckLength,
                crate::units::length::ElectronReducedComptonWavelength,
                crate::units::mass::AtomicMassUnit,
                crate::units::power::ErgPerSecond,
                crate::units::force::Dyne,
                crate::units::energy::Erg,
                crate::units::energy::Electronvolt,
                crate::units::energy::Kiloelectronvolt,
                crate::units::energy::Megaelectronvolt
            }
            $($extra),+
        );
    };
}

#[cfg(feature = "navigation")]
macro_rules! __impl_div_pairs_with_navigation_as_base {
    ($($extra:ty),+ $(,)?) => {
        crate::__impl_div_pairs_each_extra_to_bases!(
            {
                crate::units::length::NauticalMile,
                crate::units::length::Chain,
                crate::units::length::Rod,
                crate::units::length::Link,
                crate::units::length::Fathom,
                crate::units::length::EarthMeridionalCircumference,
                crate::units::length::EarthEquatorialCircumference,
                crate::units::angular::Gradian
            }
            $($extra),+
        );
    };
}

#[cfg(feature = "land-area")]
macro_rules! __impl_div_pairs_with_land_area_as_base {
    ($($extra:ty),+ $(,)?) => {
        crate::__impl_div_pairs_each_extra_to_bases!(
            {
                crate::units::area::Hectare,
                crate::units::area::Are,
                crate::units::area::Acre
            }
            $($extra),+
        );
    };
}

#[cfg(feature = "frequency")]
macro_rules! __impl_div_pairs_with_frequency_as_base {
    ($($extra:ty),+ $(,)?) => {
        crate::__impl_div_pairs_each_extra_to_bases!(
            {
                crate::units::frequency::Hertz,
                crate::units::frequency::Millihertz,
                crate::units::frequency::Kilohertz,
                crate::units::frequency::Megahertz,
                crate::units::frequency::Gigahertz,
                crate::units::frequency::Terahertz
            }
            $($extra),+
        );
    };
}

#[cfg(feature = "chemistry")]
macro_rules! __impl_div_pairs_with_chemistry_as_base {
    ($($extra:ty),+ $(,)?) => {
        crate::__impl_div_pairs_each_extra_to_bases!(
            {
                crate::units::amount::Nanomole,
                crate::units::amount::Micromole,
                crate::units::amount::Millimole,
                crate::units::amount::Mole,
                crate::units::amount::Kilomole
            }
            $($extra),+
        );
    };
}

#[cfg(feature = "electrical")]
macro_rules! __impl_div_pairs_with_electrical_as_base {
    ($($extra:ty),+ $(,)?) => {
        crate::__impl_div_pairs_each_extra_to_bases!(
            {
                crate::units::electrical::Ampere,
                crate::units::electrical::Microampere,
                crate::units::electrical::Milliampere,
                crate::units::electrical::Kiloampere,
                crate::units::electrical::Coulomb,
                crate::units::electrical::Millicoulomb,
                crate::units::electrical::Microcoulomb,
                crate::units::electrical::Kilocoulomb,
                crate::units::electrical::Volt,
                crate::units::electrical::Microvolt,
                crate::units::electrical::Millivolt,
                crate::units::electrical::Kilovolt,
                crate::units::electrical::Megavolt,
                crate::units::electrical::Ohm,
                crate::units::electrical::Milliohm,
                crate::units::electrical::Kilohm,
                crate::units::electrical::Megaohm,
                crate::units::electrical::Farad,
                crate::units::electrical::Picofarad,
                crate::units::electrical::Nanofarad,
                crate::units::electrical::Microfarad,
                crate::units::electrical::Millifarad,
                crate::units::electrical::Henry,
                crate::units::electrical::Microhenry,
                crate::units::electrical::Millihenry,
                crate::units::electrical::Weber,
                crate::units::electrical::Milliweber,
                crate::units::electrical::Tesla,
                crate::units::electrical::Microtesla,
                crate::units::electrical::Millitesla
            }
            $($extra),+
        );
    };
}

#[cfg(feature = "density")]
macro_rules! __impl_div_pairs_with_density_as_base {
    ($($extra:ty),+ $(,)?) => {
        crate::__impl_div_pairs_each_extra_to_bases!(
            {
                crate::units::density::KilogramPerCubicMeter,
                crate::units::density::GramPerCubicCentimeter,
                crate::units::density::GramPerMilliliter
            }
            $($extra),+
        );
    };
}

#[cfg(feature = "photometry")]
macro_rules! __impl_div_pairs_with_photometry_as_base {
    ($($extra:ty),+ $(,)?) => {
        crate::__impl_div_pairs_each_extra_to_bases!(
            {
                crate::units::photometry::Candela,
                crate::units::photometry::Lumen,
                crate::units::photometry::Millilumen,
                crate::units::photometry::Kilolumen,
                crate::units::photometry::Lux,
                crate::units::photometry::Millilux,
                crate::units::photometry::Kilolux
            }
            $($extra),+
        );
    };
}

// ── Cross-feature UnitDiv pairs ──────────────────────────────────────────────
//
// When two optional feature families are both enabled, units from each family
// need `UnitDiv` impls against each other so that cross-unit division "just
// works".  Multiplication is already covered by the `BuiltinUnit` blanket impl;
// division requires explicit pairs because the `U / U → Unitless` blanket
// overlaps with a hypothetical `A / B → Per<A, B>` blanket.
//
// For each pair the macro of the *larger* family is invoked so that the smaller
// family's (shorter) unit list is written at the call site.  Both `A / B` and
// `B / A` impls are generated by one call — see `__impl_div_pairs_each_extra_to_bases!`.
//
// Pair count grows as C(N, 2) with N feature families; unit lists do not grow
// with the pairing count because each list lives in exactly one macro above.

// astro (24 units) — largest optional family; always the base for its pairs.
#[cfg(all(feature = "astro", feature = "julian-time"))]
__impl_div_pairs_with_astro_as_base!(
    crate::units::time::JulianYear,
    crate::units::time::JulianCentury
);

#[cfg(all(feature = "astro", feature = "customary"))]
__impl_div_pairs_with_astro_as_base!(
    crate::units::length::Inch,
    crate::units::length::Foot,
    crate::units::length::Yard,
    crate::units::length::Mile,
    crate::units::mass::Carat,
    crate::units::mass::Grain,
    crate::units::mass::Pound,
    crate::units::mass::Ounce,
    crate::units::mass::Stone,
    crate::units::mass::ShortTon,
    crate::units::mass::LongTon,
    crate::units::power::HorsepowerMetric,
    crate::units::power::HorsepowerElectric,
    crate::units::area::SquareInch,
    crate::units::area::SquareFoot,
    crate::units::area::SquareYard,
    crate::units::area::SquareMile,
    crate::units::volume::CubicInch,
    crate::units::volume::CubicFoot,
    crate::units::volume::UsGallon,
    crate::units::volume::UsFluidOunce,
    crate::units::force::PoundForce,
    crate::units::energy::Calorie,
    crate::units::energy::Kilocalorie
);

#[cfg(all(feature = "astro", feature = "navigation"))]
__impl_div_pairs_with_astro_as_base!(
    crate::units::length::NauticalMile,
    crate::units::length::Chain,
    crate::units::length::Rod,
    crate::units::length::Link,
    crate::units::length::Fathom,
    crate::units::length::EarthMeridionalCircumference,
    crate::units::length::EarthEquatorialCircumference,
    crate::units::angular::Gradian
);

#[cfg(all(feature = "astro", feature = "fundamental-physics"))]
__impl_div_pairs_with_astro_as_base!(
    crate::units::length::BohrRadius,
    crate::units::length::ClassicalElectronRadius,
    crate::units::length::PlanckLength,
    crate::units::length::ElectronReducedComptonWavelength,
    crate::units::mass::AtomicMassUnit,
    crate::units::power::ErgPerSecond,
    crate::units::force::Dyne,
    crate::units::energy::Erg,
    crate::units::energy::Electronvolt,
    crate::units::energy::Kiloelectronvolt,
    crate::units::energy::Megaelectronvolt
);

#[cfg(all(feature = "astro", feature = "land-area"))]
__impl_div_pairs_with_astro_as_base!(
    crate::units::area::Hectare,
    crate::units::area::Are,
    crate::units::area::Acre
);

// customary (23 units) — base for its pairs with smaller families.
#[cfg(all(feature = "julian-time", feature = "customary"))]
__impl_div_pairs_with_customary_as_base!(
    crate::units::time::JulianYear,
    crate::units::time::JulianCentury
);

#[cfg(all(feature = "customary", feature = "navigation"))]
__impl_div_pairs_with_customary_as_base!(
    crate::units::length::NauticalMile,
    crate::units::length::Chain,
    crate::units::length::Rod,
    crate::units::length::Link,
    crate::units::length::Fathom,
    crate::units::length::EarthMeridionalCircumference,
    crate::units::length::EarthEquatorialCircumference,
    crate::units::angular::Gradian
);

#[cfg(all(feature = "customary", feature = "fundamental-physics"))]
__impl_div_pairs_with_customary_as_base!(
    crate::units::length::BohrRadius,
    crate::units::length::ClassicalElectronRadius,
    crate::units::length::PlanckLength,
    crate::units::length::ElectronReducedComptonWavelength,
    crate::units::mass::AtomicMassUnit,
    crate::units::power::ErgPerSecond,
    crate::units::force::Dyne,
    crate::units::energy::Erg,
    crate::units::energy::Electronvolt,
    crate::units::energy::Kiloelectronvolt,
    crate::units::energy::Megaelectronvolt
);

#[cfg(all(feature = "customary", feature = "land-area"))]
__impl_div_pairs_with_customary_as_base!(
    crate::units::area::Hectare,
    crate::units::area::Are,
    crate::units::area::Acre
);

// fundamental-physics (11 units) — base for its pairs with navigation and smaller families.
#[cfg(all(feature = "julian-time", feature = "fundamental-physics"))]
__impl_div_pairs_with_fundamental_physics_as_base!(
    crate::units::time::JulianYear,
    crate::units::time::JulianCentury
);

#[cfg(all(feature = "navigation", feature = "fundamental-physics"))]
__impl_div_pairs_with_fundamental_physics_as_base!(
    crate::units::length::NauticalMile,
    crate::units::length::Chain,
    crate::units::length::Rod,
    crate::units::length::Link,
    crate::units::length::Fathom,
    crate::units::length::EarthMeridionalCircumference,
    crate::units::length::EarthEquatorialCircumference,
    crate::units::angular::Gradian
);

#[cfg(all(feature = "fundamental-physics", feature = "land-area"))]
__impl_div_pairs_with_fundamental_physics_as_base!(
    crate::units::area::Hectare,
    crate::units::area::Are,
    crate::units::area::Acre
);

// navigation (8 units) — base for its pairs with land-area and julian-time.
#[cfg(all(feature = "julian-time", feature = "navigation"))]
__impl_div_pairs_with_navigation_as_base!(
    crate::units::time::JulianYear,
    crate::units::time::JulianCentury
);

#[cfg(all(feature = "navigation", feature = "land-area"))]
__impl_div_pairs_with_navigation_as_base!(
    crate::units::area::Hectare,
    crate::units::area::Are,
    crate::units::area::Acre
);

// land-area (3 units) — base for the julian-time pair (land-area > julian-time).
#[cfg(all(feature = "julian-time", feature = "land-area"))]
__impl_div_pairs_with_land_area_as_base!(
    crate::units::time::JulianYear,
    crate::units::time::JulianCentury
);
