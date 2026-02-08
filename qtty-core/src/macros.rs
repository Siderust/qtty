//! Macros for defining units and conversions.

/// Generates `From` trait implementations for all pairs of units within a dimension.
///
/// Also generates cross-unit `PartialEq` and `PartialOrd` trait implementations
/// so that quantities of different units in the same dimension can be compared
/// directly with `==`, `!=`, `<`, `>`, `<=`, `>=` operators. The right-hand side
/// is converted to the left-hand side's unit before comparison.
#[macro_export]
macro_rules! impl_unit_conversions {
    // Base case: single unit, no conversions needed
    ($unit:ty) => {};

    // Recursive case: implement conversions from first to all others, then recurse
    ($first:ty, $($rest:ty),+ $(,)?) => {
        $(
            impl From<$crate::Quantity<$first>> for $crate::Quantity<$rest> {
                fn from(value: $crate::Quantity<$first>) -> Self {
                    value.to::<$rest>()
                }
            }

            impl From<$crate::Quantity<$rest>> for $crate::Quantity<$first> {
                fn from(value: $crate::Quantity<$rest>) -> Self {
                    value.to::<$first>()
                }
            }

            // Cross-unit PartialEq: first == rest
            impl<S: $crate::scalar::Real> PartialEq<$crate::Quantity<$rest, S>> for $crate::Quantity<$first, S> {
                #[inline]
                fn eq(&self, other: &$crate::Quantity<$rest, S>) -> bool {
                    self.value() == other.to::<$first>().value()
                }
            }

            // Cross-unit PartialEq: rest == first
            impl<S: $crate::scalar::Real> PartialEq<$crate::Quantity<$first, S>> for $crate::Quantity<$rest, S> {
                #[inline]
                fn eq(&self, other: &$crate::Quantity<$first, S>) -> bool {
                    self.value() == other.to::<$rest>().value()
                }
            }

            // Cross-unit PartialOrd: first <=> rest
            impl<S: $crate::scalar::Real> PartialOrd<$crate::Quantity<$rest, S>> for $crate::Quantity<$first, S> {
                #[inline]
                fn partial_cmp(&self, other: &$crate::Quantity<$rest, S>) -> Option<core::cmp::Ordering> {
                    self.value().partial_cmp(&other.to::<$first>().value())
                }
            }

            // Cross-unit PartialOrd: rest <=> first
            impl<S: $crate::scalar::Real> PartialOrd<$crate::Quantity<$first, S>> for $crate::Quantity<$rest, S> {
                #[inline]
                fn partial_cmp(&self, other: &$crate::Quantity<$first, S>) -> Option<core::cmp::Ordering> {
                    self.value().partial_cmp(&other.to::<$rest>().value())
                }
            }
        )+

        // Recurse with the rest of the units
        $crate::impl_unit_conversions!($($rest),+);
    };
}
