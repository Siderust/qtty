//! Serde support for `Quantity` types (feature-gated).
//!
//! This module is enabled by the `serde` feature. It provides serialization and deserialization
//! for `Quantity<U, S>` types, including helper modules for different serialization formats.

use crate::scalar::{Real, Scalar};
use crate::{Quantity, Unit};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// Default serde: serialize as f64 (backward compatible)
impl<U: Unit, S: Real> Serialize for Quantity<U, S> {
    fn serialize<Ser>(&self, serializer: Ser) -> core::result::Result<Ser::Ok, Ser::Error>
    where
        Ser: Serializer,
    {
        // Strategy A: Always serialize as f64 for backward compatibility
        self.value().to_f64().serialize(serializer)
    }
}

impl<'de, U: Unit, S: Real> Deserialize<'de> for Quantity<U, S> {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Strategy A: Deserialize from f64 and convert
        let value = f64::deserialize(deserializer)?;
        Ok(Quantity::new(S::from_f64(value)))
    }
}

/// Serde helper module for serializing the raw scalar value.
///
/// This module serializes the scalar type directly instead of converting to f64.
/// Use this when you need to preserve the exact scalar representation.
///
/// # Example
///
/// ```rust,ignore
/// use qtty_core::length::Meters;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct Config {
///     #[serde(with = "qtty_core::serde_scalar")]
///     distance: Meters,  // Serializes the scalar directly
/// }
/// ```
pub mod serde_scalar {
    use super::*;

    /// Serializes the quantity's scalar value directly.
    #[allow(dead_code)]
    pub fn serialize<U, S, Ser>(
        quantity: &Quantity<U, S>,
        serializer: Ser,
    ) -> Result<Ser::Ok, Ser::Error>
    where
        U: Unit,
        S: Scalar + Serialize,
        Ser: Serializer,
    {
        quantity.value_ref().serialize(serializer)
    }

    /// Deserializes the quantity from a scalar value directly.
    #[allow(dead_code)]
    pub fn deserialize<'de, U, S, D>(deserializer: D) -> Result<Quantity<U, S>, D::Error>
    where
        U: Unit,
        S: Scalar + Deserialize<'de>,
        D: Deserializer<'de>,
    {
        let value = S::deserialize(deserializer)?;
        Ok(Quantity::new(value))
    }
}

/// Serde helper module for serializing quantities with unit information.
///
/// Use this with the `#[serde(with = "...")]` attribute to preserve unit symbols
/// in serialized data. This is useful for external APIs, configuration files, or
/// self-documenting data formats.
///
/// # Examples
///
/// ```rust
/// use qtty_core::length::Meters;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct Config {
///     #[serde(with = "qtty_core::serde_with_unit")]
///     max_distance: Meters,  // Serializes as {"value": 100.0, "unit": "m"}
///     
///     min_distance: Meters,  // Serializes as 50.0 (default, compact)
/// }
/// ```
#[cfg(feature = "std")]
pub mod serde_with_unit {
    extern crate alloc;
    use alloc::format;
    use alloc::string::String;

    use super::*;
    use serde::de::{self, Deserializer, MapAccess, Visitor};
    use serde::ser::{SerializeStruct, Serializer};

    /// Serializes a `Quantity<U, S>` as a struct with `value` and `unit` fields.
    ///
    /// # Example JSON Output
    /// ```json
    /// {"value": 42.5, "unit": "m"}
    /// ```
    pub fn serialize<U, S, Ser>(
        quantity: &Quantity<U, S>,
        serializer: Ser,
    ) -> Result<Ser::Ok, Ser::Error>
    where
        U: Unit,
        S: Real,
        Ser: Serializer,
    {
        let mut state = serializer.serialize_struct("Quantity", 2)?;
        state.serialize_field("value", &quantity.value().to_f64())?;
        state.serialize_field("unit", U::SYMBOL)?;
        state.end()
    }

    /// Deserializes a `Quantity<U, S>` from a struct with `value` and optionally `unit` fields.
    ///
    /// The `unit` field is validated if present but not required for backwards compatibility.
    /// If provided and doesn't match `U::SYMBOL`, an error is returned.
    pub fn deserialize<'de, U, S, D>(deserializer: D) -> Result<Quantity<U, S>, D::Error>
    where
        U: Unit,
        S: Real,
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Value,
            Unit,
        }

        struct QuantityVisitor<U, S>(core::marker::PhantomData<(U, S)>);

        impl<'de, U: Unit, S: Real> Visitor<'de> for QuantityVisitor<U, S> {
            type Value = Quantity<U, S>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("struct Quantity with value and unit fields")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Quantity<U, S>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut value: Option<f64> = None;
                let mut unit: Option<String> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Value => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                        Field::Unit => {
                            if unit.is_some() {
                                return Err(de::Error::duplicate_field("unit"));
                            }
                            unit = Some(map.next_value()?);
                        }
                    }
                }

                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;

                // Validate unit if provided (optional for backwards compatibility)
                if let Some(ref unit_str) = unit {
                    if unit_str != U::SYMBOL {
                        return Err(de::Error::custom(format!(
                            "unit mismatch: expected '{}', found '{}'",
                            U::SYMBOL,
                            unit_str
                        )));
                    }
                }

                Ok(Quantity::new(S::from_f64(value)))
            }
        }

        deserializer.deserialize_struct(
            "Quantity",
            &["value", "unit"],
            QuantityVisitor(core::marker::PhantomData),
        )
    }
}
