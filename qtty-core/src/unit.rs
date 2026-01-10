//! Unit types and traits.

use crate::dimension::{Dimension, Dimensionless, DivDim};
use crate::Quantity;
use core::fmt::{Debug, Display, Formatter, Result};
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

impl<N: Unit, D: Unit> Unit for Per<N, D> {
    const RATIO: f64 = N::RATIO / D::RATIO;
    type Dim = DivDim<N::Dim, D::Dim>;
    const SYMBOL: &'static str = "";
}

impl<N: Unit, D: Unit> Display for Quantity<Per<N, D>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}/{}", self.value(), N::SYMBOL, D::SYMBOL)
    }
}

/// Zero-sized marker type for dimensionless quantities.
///
/// `Unitless` represents a dimensionless unit with a conversion ratio of 1.0
/// and an empty symbol. It is used to model the result of simplifying same-unit
/// ratios (e.g., `Meters / Meters`) into a plain "number-like" `Quantity<Unitless>`.
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

impl Display for Quantity<Unitless> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value())
    }
}

/// Trait for simplifying composite unit types.
///
/// This allows reducing complex unit expressions to simpler forms,
/// such as `Per<U, U>` to `Unitless` or `Per<N, Per<N, D>>` to `D`.
pub trait Simplify {
    /// The simplified unit type.
    type Out: Unit;
    /// Convert this quantity to its simplified unit.
    fn simplify(self) -> Quantity<Self::Out>;
}

impl<U: Unit> Simplify for Quantity<Per<U, U>> {
    type Out = Unitless;
    /// ```rust
    /// use qtty_core::length::Meters;
    /// use qtty_core::{Quantity, Simplify, Unitless};
    ///
    /// let ratio = Meters::new(1.0) / Meters::new(2.0);
    /// let unitless: Quantity<Unitless> = ratio.simplify();
    /// assert!((unitless.value() - 0.5).abs() < 1e-12);
    /// ```
    fn simplify(self) -> Quantity<Unitless> {
        Quantity::new(self.value())
    }
}

impl<N: Unit, D: Unit> Simplify for Quantity<Per<N, Per<N, D>>> {
    type Out = D;
    fn simplify(self) -> Quantity<D> {
        Quantity::new(self.value())
    }
}
