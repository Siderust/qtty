// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Unit types and traits.

use crate::dimension::{DimDiv, DimMul, Dimension, Dimensionless};
use crate::scalar::Scalar;
use crate::Quantity;
use core::fmt::{Debug, Display, Formatter, LowerExp, Result, UpperExp};
use core::marker::PhantomData;

/// Trait implemented by every **unit** type.
///
/// * `RATIO` is the conversion factor from this unit to the *canonical scaling unit* of the same dimension.
///   Example: if metres are canonical (`Meter::RATIO == 1.0`), then kilometres use `Kilometer::RATIO == 1000.0`
///   because `1 km = 1000 m`.
///
/// * `SYMBOL` is the printable string (e.g. `"m"` or `"km"`).
///
/// * `Dim` ties the unit to its underlying [`Dimension`].
///
/// # Invariants
///
/// - Implementations should be zero-sized marker types (this crate's built-in units are unit structs with no fields).
/// - `RATIO` should be finite and non-zero.
pub trait Unit: Copy + PartialEq + Debug + 'static {
    /// Unit-to-canonical conversion factor.
    const RATIO: f64;

    /// Dimension to which this unit belongs.
    type Dim: Dimension;

    /// Printable symbol, shown by [`core::fmt::Display`].
    const SYMBOL: &'static str;
}

/// Unit representing the division of two other units.
///
/// `Per<N, D>` corresponds to `N / D` and carries both the
/// dimensional information and the scaling ratio between the
/// constituent units. It is generic over any numerator and
/// denominator units, which allows implementing arithmetic
/// generically for all pairs without bespoke macros.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Per<N: Unit, D: Unit>(PhantomData<(N, D)>);

impl<N: Unit, D: Unit> Unit for Per<N, D>
where
    N::Dim: DimDiv<D::Dim>,
    <N::Dim as DimDiv<D::Dim>>::Output: Dimension,
{
    const RATIO: f64 = N::RATIO / D::RATIO;
    type Dim = <N::Dim as DimDiv<D::Dim>>::Output;
    const SYMBOL: &'static str = "";
}

impl<N: Unit, D: Unit, S: Scalar + Display> Display for Quantity<Per<N, D>, S>
where
    N::Dim: DimDiv<D::Dim>,
    <N::Dim as DimDiv<D::Dim>>::Output: Dimension,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.value(), f)?;
        write!(f, " {}/{}", N::SYMBOL, D::SYMBOL)
    }
}

impl<N: Unit, D: Unit, S: Scalar + LowerExp> LowerExp for Quantity<Per<N, D>, S>
where
    N::Dim: DimDiv<D::Dim>,
    <N::Dim as DimDiv<D::Dim>>::Output: Dimension,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        LowerExp::fmt(&self.value(), f)?;
        write!(f, " {}/{}", N::SYMBOL, D::SYMBOL)
    }
}

impl<N: Unit, D: Unit, S: Scalar + UpperExp> UpperExp for Quantity<Per<N, D>, S>
where
    N::Dim: DimDiv<D::Dim>,
    <N::Dim as DimDiv<D::Dim>>::Output: Dimension,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        UpperExp::fmt(&self.value(), f)?;
        write!(f, " {}/{}", N::SYMBOL, D::SYMBOL)
    }
}

/// Unit representing the product of two other units.
///
/// `Prod<A, B>` corresponds to `A * B` and carries both the
/// dimensional information and the scaling ratio between the
/// constituent units.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Prod<A: Unit, B: Unit>(PhantomData<(A, B)>);

impl<A: Unit, B: Unit> Unit for Prod<A, B>
where
    A::Dim: DimMul<B::Dim>,
    <A::Dim as DimMul<B::Dim>>::Output: Dimension,
{
    const RATIO: f64 = A::RATIO * B::RATIO;
    type Dim = <A::Dim as DimMul<B::Dim>>::Output;
    const SYMBOL: &'static str = "";
}

impl<A: Unit, B: Unit, S: Scalar + Display> Display for Quantity<Prod<A, B>, S>
where
    A::Dim: DimMul<B::Dim>,
    <A::Dim as DimMul<B::Dim>>::Output: Dimension,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.value(), f)?;
        write!(f, " {}·{}", A::SYMBOL, B::SYMBOL)
    }
}

impl<A: Unit, B: Unit, S: Scalar + LowerExp> LowerExp for Quantity<Prod<A, B>, S>
where
    A::Dim: DimMul<B::Dim>,
    <A::Dim as DimMul<B::Dim>>::Output: Dimension,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        LowerExp::fmt(&self.value(), f)?;
        write!(f, " {}·{}", A::SYMBOL, B::SYMBOL)
    }
}

impl<A: Unit, B: Unit, S: Scalar + UpperExp> UpperExp for Quantity<Prod<A, B>, S>
where
    A::Dim: DimMul<B::Dim>,
    <A::Dim as DimMul<B::Dim>>::Output: Dimension,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        UpperExp::fmt(&self.value(), f)?;
        write!(f, " {}·{}", A::SYMBOL, B::SYMBOL)
    }
}

/// Zero-sized marker type for dimensionless quantities.
///
/// `Unitless` represents a dimensionless unit with a conversion ratio of 1.0
/// and an empty symbol. It is used to model same-unit ratios (e.g.,
/// `Meters / Meters`) as a plain "number-like" `Quantity<Unitless>`.
///
/// Unlike a type alias to `f64`, this is a proper zero-sized type, which ensures
/// that only explicitly constructed `Quantity<Unitless>` values are treated as
/// dimensionless, not bare `f64` primitives.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Unitless;

impl Unit for Unitless {
    const RATIO: f64 = 1.0;
    type Dim = Dimensionless;
    const SYMBOL: &'static str = "";
}

impl<S: Scalar + Display> Display for Quantity<Unitless, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.value(), f)
    }
}

impl<S: Scalar + LowerExp> LowerExp for Quantity<Unitless, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        LowerExp::fmt(&self.value(), f)
    }
}

impl<S: Scalar + UpperExp> UpperExp for Quantity<Unitless, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        UpperExp::fmt(&self.value(), f)
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use crate::units::length::{Kilometer, Meter};
    use crate::units::time::Second;
    use crate::Quantity;

    // ── Per: Display, LowerExp, UpperExp ──────────────────────────────────────

    #[test]
    fn per_display_formats_value_and_symbol() {
        // 10 m/s with Display
        let qty: Quantity<Per<Meter, Second>> = Quantity::new(10.0);
        let s = format!("{qty}");
        assert_eq!(s, "10 m/s");
    }

    #[test]
    fn per_display_with_precision() {
        let qty: Quantity<Per<Meter, Second>> = Quantity::new(1.5);
        let s = format!("{qty:.2}");
        assert_eq!(s, "1.50 m/s");
    }

    #[test]
    fn per_lower_exp_formats_correctly() {
        let qty: Quantity<Per<Meter, Second>> = Quantity::new(1000.0);
        let s = format!("{qty:e}");
        assert!(s.contains("e"), "Expected scientific notation, got: {s}");
        assert!(s.ends_with("m/s"), "Expected 'm/s' suffix, got: {s}");
    }

    #[test]
    fn per_upper_exp_formats_correctly() {
        let qty: Quantity<Per<Meter, Second>> = Quantity::new(1000.0);
        let s = format!("{qty:E}");
        assert!(s.contains("E"), "Expected uppercase-E notation, got: {s}");
        assert!(s.ends_with("m/s"), "Expected 'm/s' suffix, got: {s}");
    }

    // ── Prod: Display, LowerExp, UpperExp ─────────────────────────────────────

    #[test]
    fn prod_display_formats_value_and_symbol() {
        let qty: Quantity<Prod<Meter, Second>> = Quantity::new(3.0);
        let s = format!("{qty}");
        assert_eq!(s, "3 m·s");
    }

    #[test]
    fn prod_display_with_precision() {
        let qty: Quantity<Prod<Meter, Second>> = Quantity::new(2.5);
        let s = format!("{qty:.3}");
        assert_eq!(s, "2.500 m·s");
    }

    #[test]
    fn prod_lower_exp_formats_correctly() {
        let qty: Quantity<Prod<Kilometer, Second>> = Quantity::new(5000.0);
        let s = format!("{qty:.2e}");
        assert!(s.contains("e"), "Expected scientific notation, got: {s}");
        assert!(s.ends_with("km·s"), "Expected 'km·s' suffix, got: {s}");
    }

    #[test]
    fn prod_upper_exp_formats_correctly() {
        let qty: Quantity<Prod<Kilometer, Second>> = Quantity::new(5000.0);
        let s = format!("{qty:.2E}");
        assert!(s.contains("E"), "Expected uppercase-E notation, got: {s}");
        assert!(s.ends_with("km·s"), "Expected 'km·s' suffix, got: {s}");
    }

    // ── Unitless: LowerExp, UpperExp ──────────────────────────────────────────

    #[test]
    fn unitless_lower_exp_formats_correctly() {
        let qty: Quantity<Unitless> = Quantity::new(0.5);
        let s = format!("{qty:e}");
        assert!(s.contains("e"), "Expected scientific notation, got: {s}");
        // No unit symbol for Unitless
        assert!(
            !s.contains(' '),
            "Unitless should not have a space, got: {s}"
        );
    }

    #[test]
    fn unitless_upper_exp_formats_correctly() {
        let qty: Quantity<Unitless> = Quantity::new(0.5);
        let s = format!("{qty:E}");
        assert!(s.contains("E"), "Expected uppercase-E notation, got: {s}");
    }
}
