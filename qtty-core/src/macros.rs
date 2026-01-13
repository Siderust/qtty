//! Macros for defining units and conversions.

/// Generates `From` trait implementations for all pairs of units within a dimension.
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
        )+

        // Recurse with the rest of the units
        $crate::impl_unit_conversions!($($rest),+);
    };
}
