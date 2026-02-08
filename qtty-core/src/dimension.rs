//! Dimension types and traits.
//!
//! Dimensions are modelled as a single generic struct [`Dim`] parameterised by eight
//! [`typenum`] signed integers representing the exponents of the fundamental physical
//! dimensions. Multiplying two dimensions adds exponents; dividing subtracts them.
//! Because typenum arithmetic resolves at compile time, compound types like `Length * Length`
//! automatically become `Area` (exponent 2), and `Area / Length` collapses back to `Length`
//! (exponent 1) — all verified by the type checker.
//!
//! # Layout
//!
//! The eight exponent slots are:
//!
//! | Position | Base quantity            | SI symbol |
//! |----------|--------------------------|-----------|
//! | `L`      | Length                   | m         |
//! | `T`      | Time                     | s         |
//! | `M`      | Mass                     | kg        |
//! | `Th`     | Thermodynamic temperature| K         |
//! | `I`      | Electric current         | A         |
//! | `N`      | Amount of substance      | mol       |
//! | `J`      | Luminous intensity       | cd        |
//! | `A`      | Plane angle (auxiliary)  | rad/deg   |

use core::marker::PhantomData;
use core::ops::{Add, Sub};
use typenum::Integer;

/// Marker trait for **dimensions**.
///
/// Implemented automatically for every [`Dim<L,T,M,Th,I,N,J,A>`] whose type
/// parameters satisfy the required bounds.
pub trait Dimension: 'static {}

// ─────────────────────────────────────────────────────────────────────────────
// Core dimension struct
// ─────────────────────────────────────────────────────────────────────────────

/// A physical dimension encoded as eight typenum integer exponents.
///
/// This is a zero-sized type: no runtime cost.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dim<L, T, M, Th, I, N, J, A>(
    PhantomData<L>,
    PhantomData<T>,
    PhantomData<M>,
    PhantomData<Th>,
    PhantomData<I>,
    PhantomData<N>,
    PhantomData<J>,
    PhantomData<A>,
)
where
    L: Integer,
    T: Integer,
    M: Integer,
    Th: Integer,
    I: Integer,
    N: Integer,
    J: Integer,
    A: Integer;

impl<L, T, M, Th, I, N, J, A> Dimension for Dim<L, T, M, Th, I, N, J, A>
where
    L: Integer + 'static,
    T: Integer + 'static,
    M: Integer + 'static,
    Th: Integer + 'static,
    I: Integer + 'static,
    N: Integer + 'static,
    J: Integer + 'static,
    A: Integer + 'static,
{
}

// ─────────────────────────────────────────────────────────────────────────────
// Dimension multiplication (adds exponents)
// ─────────────────────────────────────────────────────────────────────────────

/// Trait for multiplying two dimensions (adds exponents).
pub trait DimMul<Rhs: Dimension>: Dimension {
    /// The resulting dimension.
    type Output: Dimension;
}

impl<L1, T1, M1, Th1, I1, N1, J1, A1, L2, T2, M2, Th2, I2, N2, J2, A2>
    DimMul<Dim<L2, T2, M2, Th2, I2, N2, J2, A2>> for Dim<L1, T1, M1, Th1, I1, N1, J1, A1>
where
    L1: Integer + Add<L2> + 'static,
    T1: Integer + Add<T2> + 'static,
    M1: Integer + Add<M2> + 'static,
    Th1: Integer + Add<Th2> + 'static,
    I1: Integer + Add<I2> + 'static,
    N1: Integer + Add<N2> + 'static,
    J1: Integer + Add<J2> + 'static,
    A1: Integer + Add<A2> + 'static,
    L2: Integer + 'static,
    T2: Integer + 'static,
    M2: Integer + 'static,
    Th2: Integer + 'static,
    I2: Integer + 'static,
    N2: Integer + 'static,
    J2: Integer + 'static,
    A2: Integer + 'static,
    <L1 as Add<L2>>::Output: Integer + 'static,
    <T1 as Add<T2>>::Output: Integer + 'static,
    <M1 as Add<M2>>::Output: Integer + 'static,
    <Th1 as Add<Th2>>::Output: Integer + 'static,
    <I1 as Add<I2>>::Output: Integer + 'static,
    <N1 as Add<N2>>::Output: Integer + 'static,
    <J1 as Add<J2>>::Output: Integer + 'static,
    <A1 as Add<A2>>::Output: Integer + 'static,
{
    type Output = Dim<
        <L1 as Add<L2>>::Output,
        <T1 as Add<T2>>::Output,
        <M1 as Add<M2>>::Output,
        <Th1 as Add<Th2>>::Output,
        <I1 as Add<I2>>::Output,
        <N1 as Add<N2>>::Output,
        <J1 as Add<J2>>::Output,
        <A1 as Add<A2>>::Output,
    >;
}

// ─────────────────────────────────────────────────────────────────────────────
// Dimension division (subtracts exponents)
// ─────────────────────────────────────────────────────────────────────────────

/// Trait for dividing two dimensions (subtracts exponents).
pub trait DimDiv<Rhs: Dimension>: Dimension {
    /// The resulting dimension.
    type Output: Dimension;
}

impl<L1, T1, M1, Th1, I1, N1, J1, A1, L2, T2, M2, Th2, I2, N2, J2, A2>
    DimDiv<Dim<L2, T2, M2, Th2, I2, N2, J2, A2>> for Dim<L1, T1, M1, Th1, I1, N1, J1, A1>
where
    L1: Integer + Sub<L2> + 'static,
    T1: Integer + Sub<T2> + 'static,
    M1: Integer + Sub<M2> + 'static,
    Th1: Integer + Sub<Th2> + 'static,
    I1: Integer + Sub<I2> + 'static,
    N1: Integer + Sub<N2> + 'static,
    J1: Integer + Sub<J2> + 'static,
    A1: Integer + Sub<A2> + 'static,
    L2: Integer + 'static,
    T2: Integer + 'static,
    M2: Integer + 'static,
    Th2: Integer + 'static,
    I2: Integer + 'static,
    N2: Integer + 'static,
    J2: Integer + 'static,
    A2: Integer + 'static,
    <L1 as Sub<L2>>::Output: Integer + 'static,
    <T1 as Sub<T2>>::Output: Integer + 'static,
    <M1 as Sub<M2>>::Output: Integer + 'static,
    <Th1 as Sub<Th2>>::Output: Integer + 'static,
    <I1 as Sub<I2>>::Output: Integer + 'static,
    <N1 as Sub<N2>>::Output: Integer + 'static,
    <J1 as Sub<J2>>::Output: Integer + 'static,
    <A1 as Sub<A2>>::Output: Integer + 'static,
{
    type Output = Dim<
        <L1 as Sub<L2>>::Output,
        <T1 as Sub<T2>>::Output,
        <M1 as Sub<M2>>::Output,
        <Th1 as Sub<Th2>>::Output,
        <I1 as Sub<I2>>::Output,
        <N1 as Sub<N2>>::Output,
        <J1 as Sub<J2>>::Output,
        <A1 as Sub<A2>>::Output,
    >;
}

// ─────────────────────────────────────────────────────────────────────────────
// Base dimension aliases
// ─────────────────────────────────────────────────────────────────────────────

use typenum::{N1, N2, N3, P1, P2, P3, Z0};

/// Dimensionless (all exponents zero).
pub type Dimensionless = Dim<Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;

/// Length (L¹).
pub type Length = Dim<P1, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;

/// Time (T¹).
pub type Time = Dim<Z0, P1, Z0, Z0, Z0, Z0, Z0, Z0>;

/// Mass (M¹).
pub type Mass = Dim<Z0, Z0, P1, Z0, Z0, Z0, Z0, Z0>;

/// Thermodynamic temperature (Θ¹).
pub type Temperature = Dim<Z0, Z0, Z0, P1, Z0, Z0, Z0, Z0>;

/// Electric current (I¹).
pub type Current = Dim<Z0, Z0, Z0, Z0, P1, Z0, Z0, Z0>;

/// Amount of substance (N¹).
pub type AmountOfSubstance = Dim<Z0, Z0, Z0, Z0, Z0, P1, Z0, Z0>;

/// Luminous intensity (J¹).
pub type LuminousIntensity = Dim<Z0, Z0, Z0, Z0, Z0, Z0, P1, Z0>;

/// Plane angle (A¹) — treated as an independent dimension for type safety.
pub type Angular = Dim<Z0, Z0, Z0, Z0, Z0, Z0, Z0, P1>;

// ─────────────────────────────────────────────────────────────────────────────
// Derived dimension aliases
// ─────────────────────────────────────────────────────────────────────────────

/// Area (L²).
pub type Area = Dim<P2, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;

/// Volume (L³).
pub type Volume = Dim<P3, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;

/// Velocity (L¹ · T⁻¹).
pub type VelocityDim = Dim<P1, N1, Z0, Z0, Z0, Z0, Z0, Z0>;

/// Acceleration (L¹ · T⁻²).
pub type Acceleration = Dim<P1, N2, Z0, Z0, Z0, Z0, Z0, Z0>;

/// Force (M¹ · L¹ · T⁻²).
pub type Force = Dim<P1, N2, P1, Z0, Z0, Z0, Z0, Z0>;

/// Energy (M¹ · L² · T⁻²).
pub type Energy = Dim<P2, N2, P1, Z0, Z0, Z0, Z0, Z0>;

/// Power (M¹ · L² · T⁻³).
pub type Power = Dim<P2, N3, P1, Z0, Z0, Z0, Z0, Z0>;

/// Frequency — angular per time (A¹ · T⁻¹).
pub type FrequencyDim = Dim<Z0, N1, Z0, Z0, Z0, Z0, Z0, P1>;

// ─────────────────────────────────────────────────────────────────────────────
// Legacy compatibility alias
// ─────────────────────────────────────────────────────────────────────────────

/// Backward-compatible alias: `DivDim<N, D>` resolves to `<N as DimDiv<D>>::Output`.
///
/// This preserves source compatibility for code that previously used `DivDim<Length, Time>`.
/// It now resolves to the same concrete type as the `Velocity` alias, since typenum
/// arithmetic produces the identical `Dim<P1, N1, …>`.
pub type DivDim<N, D> = <N as DimDiv<D>>::Output;

/// Backward-compatible alias: `MulDim<A, B>` resolves to `<A as DimMul<B>>::Output`.
pub type MulDim<A, B> = <A as DimMul<B>>::Output;
