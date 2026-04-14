// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Macros for defining units and conversions.

/// Generates bidirectional `From` trait implementations for all pairs of units within a dimension.
///
/// The generated impls are generic over any [`Real`](crate::scalar::Real) scalar,
/// so they work for `f64`, `f32`, and any future `Real` type.
#[macro_export]
macro_rules! impl_unit_from_conversions {
    // Base case: single unit, no conversions needed
    ($unit:ty) => {};

    // Recursive case: implement conversions from first to all others, then recurse
    ($first:ty, $($rest:ty),+ $(,)?) => {
        $(
            impl<S: $crate::scalar::Real> From<$crate::Quantity<$first, S>> for $crate::Quantity<$rest, S> {
                fn from(value: $crate::Quantity<$first, S>) -> Self {
                    value.to::<$rest>()
                }
            }

            impl<S: $crate::scalar::Real> From<$crate::Quantity<$rest, S>> for $crate::Quantity<$first, S> {
                fn from(value: $crate::Quantity<$rest, S>) -> Self {
                    value.to::<$first>()
                }
            }

        )+

        // Recurse with the rest of the units
        $crate::impl_unit_from_conversions!($($rest),+);
    };
}

/// Generates cross-unit `PartialEq` and `PartialOrd` implementations for all pairs of units.
///
/// This enables direct `==`, `!=`, `<`, `>`, `<=`, `>=` comparisons across
/// different units in the same dimension by converting the right-hand side into
/// the left-hand side unit before comparing.
#[macro_export]
macro_rules! impl_unit_cross_unit_ops {
    // Base case: single unit, no cross-unit comparisons needed
    ($unit:ty) => {};

    // Recursive case: implement comparisons from first to all others, then recurse
    ($first:ty, $($rest:ty),+ $(,)?) => {
        $(
            // Cross-unit PartialEq: first == rest
            //
            // To avoid multiplying near-MAX values by large absolute ratios
            // (which overflows to ±inf, collapsing distinct quantities to
            // inf == inf → true), we always multiply the value in the
            // *smaller*-RATIO unit by (smaller_ratio / larger_ratio) ≤ 1.
            //
            // Both `first == rest` and `rest == first` therefore reduce to the
            // same floating-point expression, preserving IEEE 754 symmetry.
            impl<S: $crate::scalar::Real> PartialEq<$crate::Quantity<$rest, S>> for $crate::Quantity<$first, S> {
                #[inline]
                fn eq(&self, other: &$crate::Quantity<$rest, S>) -> bool {
                    const R_FIRST: f64 = <$first as $crate::Unit>::RATIO;
                    const R_REST:  f64 = <$rest  as $crate::Unit>::RATIO;
                    if R_FIRST >= R_REST {
                        // first is the larger unit; multiply *rest* val by ratio ≤ 1
                        self.value() == other.value() * S::from_f64(R_REST / R_FIRST)
                    } else {
                        // rest is the larger unit; multiply *first* val by ratio ≤ 1
                        self.value() * S::from_f64(R_FIRST / R_REST) == other.value()
                    }
                }
            }

            // Cross-unit PartialEq: rest == first
            impl<S: $crate::scalar::Real> PartialEq<$crate::Quantity<$first, S>> for $crate::Quantity<$rest, S> {
                #[inline]
                fn eq(&self, other: &$crate::Quantity<$first, S>) -> bool {
                    const R_FIRST: f64 = <$first as $crate::Unit>::RATIO;
                    const R_REST:  f64 = <$rest  as $crate::Unit>::RATIO;
                    if R_REST >= R_FIRST {
                        self.value() == other.value() * S::from_f64(R_FIRST / R_REST)
                    } else {
                        self.value() * S::from_f64(R_REST / R_FIRST) == other.value()
                    }
                }
            }

            // Cross-unit PartialOrd: first <=> rest
            impl<S: $crate::scalar::Real> PartialOrd<$crate::Quantity<$rest, S>> for $crate::Quantity<$first, S> {
                #[inline]
                fn partial_cmp(&self, other: &$crate::Quantity<$rest, S>) -> Option<core::cmp::Ordering> {
                    const R_FIRST: f64 = <$first as $crate::Unit>::RATIO;
                    const R_REST:  f64 = <$rest  as $crate::Unit>::RATIO;
                    if R_FIRST >= R_REST {
                        self.value().partial_cmp(&(other.value() * S::from_f64(R_REST / R_FIRST)))
                    } else {
                        (self.value() * S::from_f64(R_FIRST / R_REST)).partial_cmp(&other.value())
                    }
                }
            }

            // Cross-unit PartialOrd: rest <=> first
            impl<S: $crate::scalar::Real> PartialOrd<$crate::Quantity<$first, S>> for $crate::Quantity<$rest, S> {
                #[inline]
                fn partial_cmp(&self, other: &$crate::Quantity<$first, S>) -> Option<core::cmp::Ordering> {
                    const R_FIRST: f64 = <$first as $crate::Unit>::RATIO;
                    const R_REST:  f64 = <$rest  as $crate::Unit>::RATIO;
                    if R_REST >= R_FIRST {
                        self.value().partial_cmp(&(other.value() * S::from_f64(R_FIRST / R_REST)))
                    } else {
                        (self.value() * S::from_f64(R_REST / R_FIRST)).partial_cmp(&other.value())
                    }
                }
            }
        )+

        // Recurse with the rest of the units
        $crate::impl_unit_cross_unit_ops!($($rest),+);
    };
}

/// Compile-time assertion that every listed unit type implements
/// [`BuiltinUnit`](crate::unit_arithmetic::BuiltinUnit).
///
/// Each dimension module calls its own inventory macro with this callback under
/// `#[cfg(test)]`. The generated code uses a supertrait bound: if a type does
/// not implement `BuiltinUnit`, the `impl _AssertBuiltin for $unit {}` line
/// fails to compile, catching drift between an inventory and
/// `register_builtin_units!` in [`unit_arithmetic`](crate::unit_arithmetic).
///
/// This only catches the forward direction (unit in inventory but missing from
/// `register_builtin_units!`). The reverse (unit in registry but missing from
/// inventory) produces unusable arithmetic and is caught by downstream tests.
///
/// ```rust,ignore
/// #[cfg(test)]
/// time_units!(crate::assert_units_are_builtin);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! assert_units_are_builtin {
    ($($unit:ty),+ $(,)?) => {
        const _: () = {
            trait _AssertBuiltin: $crate::unit_arithmetic::BuiltinUnit {}
            $(impl _AssertBuiltin for $unit {})+
        };
    };
}

/// Generates all pairwise conversions and cross-unit comparisons.
///
/// Prefer `impl_unit_from_conversions!` + optional `impl_unit_cross_unit_ops!`
/// in large unit catalogs to avoid generating cross-unit comparison impls when
/// they are not needed.
#[macro_export]
macro_rules! impl_unit_conversions {
    ($($unit:ty),+ $(,)?) => {
        $crate::impl_unit_from_conversions!($($unit),+);
        $crate::impl_unit_cross_unit_ops!($($unit),+);
    };
}

// ─────────────────────────────────────────────────────────────────────────────
// Between-group helpers (used for feature-gated unit families)
// ─────────────────────────────────────────────────────────────────────────────

/// Helper: generate `From` between one extra unit and every base unit.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_from_one_to_many {
    ($one:ty; $($base:ty),+ $(,)?) => {
        $(
            impl<S: $crate::scalar::Real> From<$crate::Quantity<$one, S>> for $crate::Quantity<$base, S> {
                fn from(value: $crate::Quantity<$one, S>) -> Self {
                    value.to::<$base>()
                }
            }

            impl<S: $crate::scalar::Real> From<$crate::Quantity<$base, S>> for $crate::Quantity<$one, S> {
                fn from(value: $crate::Quantity<$base, S>) -> Self {
                    value.to::<$one>()
                }
            }
        )+
    };
}

/// Generates `From` implementations between every unit in the **extra** group
/// and every unit in the **base** group, *plus* all intra-extra pairs.
///
/// Does *not* regenerate intra-base pairs (those must be emitted separately
/// via [`impl_unit_from_conversions!`]).
///
/// Syntax: `impl_unit_from_conversions_between!(Base1, Base2; Extra1, Extra2);`
#[macro_export]
macro_rules! impl_unit_from_conversions_between {
    ($($base:ty),+; $($extra:ty),+ $(,)?) => {
        // extra <-> base (recursive to avoid repetition-count mismatch)
        $crate::__impl_from_each_extra_to_bases!({$($base),+} $($extra),+);
        // intra-extra
        $crate::impl_unit_from_conversions!($($extra),+);
    };
    // Single extra unit (no intra-extra needed)
    ($($base:ty),+; $extra:ty $(,)?) => {
        $crate::__impl_from_one_to_many!($extra; $($base),+);
    };
}

/// Recursive helper: iterate over extras one at a time, emitting cross-pairs
/// with the full base list each time.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_from_each_extra_to_bases {
    // Base case: single extra remaining.
    ({$($base:ty),+} $extra:ty) => {
        $crate::__impl_from_one_to_many!($extra; $($base),+);
    };
    // Recursive case: peel the first extra, recurse on the rest.
    ({$($base:ty),+} $first:ty, $($rest:ty),+) => {
        $crate::__impl_from_one_to_many!($first; $($base),+);
        $crate::__impl_from_each_extra_to_bases!({$($base),+} $($rest),+);
    };
}

/// Helper: generate cross-unit `PartialEq` + `PartialOrd` between one extra
/// unit and every base unit.
///
/// Uses the same smaller-ratio algorithm as [`impl_unit_cross_unit_ops!`] to
/// avoid multiplying large values by absolute ratios that can overflow to `±inf`,
/// which would collapse distinct quantities into spurious equality.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_cross_ops_one_to_many {
    ($one:ty; $($base:ty),+ $(,)?) => {
        $(
            impl<S: $crate::scalar::Real> PartialEq<$crate::Quantity<$base, S>> for $crate::Quantity<$one, S> {
                #[inline]
                fn eq(&self, other: &$crate::Quantity<$base, S>) -> bool {
                    const R_ONE:  f64 = <$one  as $crate::Unit>::RATIO;
                    const R_BASE: f64 = <$base as $crate::Unit>::RATIO;
                    if R_ONE >= R_BASE {
                        // one is the larger unit; scale *base* value by ratio ≤ 1
                        self.value() == other.value() * S::from_f64(R_BASE / R_ONE)
                    } else {
                        // base is the larger unit; scale *one* value by ratio ≤ 1
                        self.value() * S::from_f64(R_ONE / R_BASE) == other.value()
                    }
                }
            }

            impl<S: $crate::scalar::Real> PartialEq<$crate::Quantity<$one, S>> for $crate::Quantity<$base, S> {
                #[inline]
                fn eq(&self, other: &$crate::Quantity<$one, S>) -> bool {
                    const R_ONE:  f64 = <$one  as $crate::Unit>::RATIO;
                    const R_BASE: f64 = <$base as $crate::Unit>::RATIO;
                    if R_BASE >= R_ONE {
                        self.value() == other.value() * S::from_f64(R_ONE / R_BASE)
                    } else {
                        self.value() * S::from_f64(R_BASE / R_ONE) == other.value()
                    }
                }
            }

            impl<S: $crate::scalar::Real> PartialOrd<$crate::Quantity<$base, S>> for $crate::Quantity<$one, S> {
                #[inline]
                fn partial_cmp(&self, other: &$crate::Quantity<$base, S>) -> Option<core::cmp::Ordering> {
                    const R_ONE:  f64 = <$one  as $crate::Unit>::RATIO;
                    const R_BASE: f64 = <$base as $crate::Unit>::RATIO;
                    if R_ONE >= R_BASE {
                        self.value().partial_cmp(&(other.value() * S::from_f64(R_BASE / R_ONE)))
                    } else {
                        (self.value() * S::from_f64(R_ONE / R_BASE)).partial_cmp(&other.value())
                    }
                }
            }

            impl<S: $crate::scalar::Real> PartialOrd<$crate::Quantity<$one, S>> for $crate::Quantity<$base, S> {
                #[inline]
                fn partial_cmp(&self, other: &$crate::Quantity<$one, S>) -> Option<core::cmp::Ordering> {
                    const R_ONE:  f64 = <$one  as $crate::Unit>::RATIO;
                    const R_BASE: f64 = <$base as $crate::Unit>::RATIO;
                    if R_BASE >= R_ONE {
                        self.value().partial_cmp(&(other.value() * S::from_f64(R_ONE / R_BASE)))
                    } else {
                        (self.value() * S::from_f64(R_BASE / R_ONE)).partial_cmp(&other.value())
                    }
                }
            }
        )+
    };
}

/// Generates cross-unit `PartialEq` and `PartialOrd` implementations between
/// every unit in the **extra** group and every unit in the **base** group,
/// *plus* all intra-extra pairs.
///
/// Syntax: `impl_unit_cross_unit_ops_between!(Base1, Base2; Extra1, Extra2);`
#[macro_export]
macro_rules! impl_unit_cross_unit_ops_between {
    ($($base:ty),+; $($extra:ty),+ $(,)?) => {
        // extra <-> base (recursive to avoid repetition-count mismatch)
        $crate::__impl_cross_ops_each_extra_to_bases!({$($base),+} $($extra),+);
        // intra-extra
        $crate::impl_unit_cross_unit_ops!($($extra),+);
    };
    // Single extra unit (no intra-extra needed)
    ($($base:ty),+; $extra:ty $(,)?) => {
        $crate::__impl_cross_ops_one_to_many!($extra; $($base),+);
    };
}

/// Recursive helper: iterate over extras one at a time, emitting cross-ops
/// with the full base list each time.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_cross_ops_each_extra_to_bases {
    // Base case: single extra remaining.
    ({$($base:ty),+} $extra:ty) => {
        $crate::__impl_cross_ops_one_to_many!($extra; $($base),+);
    };
    // Recursive case: peel the first extra, recurse on the rest.
    ({$($base:ty),+} $first:ty, $($rest:ty),+) => {
        $crate::__impl_cross_ops_one_to_many!($first; $($base),+);
        $crate::__impl_cross_ops_each_extra_to_bases!({$($base),+} $($rest),+);
    };
}
