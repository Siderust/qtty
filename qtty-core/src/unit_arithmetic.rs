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
//! For all built-in unit pairs (plain marker types), fallback impls produce the
//! default composite types:
//!
//! - `A / B → Per<A, B>` (when `A ≠ B`)
//! - `A * B → Prod<A, B>`
//!
//! Downstream crates can use the exported macros
//! [`impl_unit_division_pairs!`], [`impl_unit_multiplication_pairs!`], and
//! [`impl_unit_arithmetic_pairs!`] to register their own custom units into the
//! same fallback tables.

use crate::dimension::{DimDiv, DimMul, Dimension};
use crate::unit::{Per, Prod, Unit, Unitless};

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

// Marker for plain built-in units. This lets built-in multiplication use a
// single generic impl instead of an O(n²) generated impl table.
pub(crate) trait BuiltinUnit: Unit {}

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

macro_rules! register_builtin_units {
    ($($unit:ty),+ $(,)?) => {
        $(
            impl BuiltinUnit for $unit {}
        )+

        impl_unit_division_pairs!($($unit),+);
    };
}

// The flat unit list is generated by build.rs from the per-dimension inventory
// macros (angular_units!, length_units!, etc.) so that adding a unit to an
// inventory automatically registers it here.
include!(concat!(env!("OUT_DIR"), "/all_builtin_units.rs"));

all_builtin_unit_types!(register_builtin_units);
