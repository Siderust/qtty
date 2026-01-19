//! Serde support for `Quantity` types (feature-gated).
//!
//! This module is enabled by the `serde` feature. It provides serialization and deserialization
//! for `Quantity<U>` types, including helper modules for different serialization formats.

use crate::{Quantity, Unit};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<U: Unit> Serialize for Quantity<U> {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.value().serialize(serializer)
    }
}

impl<'de, U: Unit> Deserialize<'de> for Quantity<U> {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = f64::deserialize(deserializer)?;
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
pub mod serde_with_unit {
    use super::*;
    use serde::de::{self, Deserializer, MapAccess, Visitor};
    use serde::ser::{SerializeStruct, Serializer};

    /// Serializes a `Quantity<U>` as a struct with `value` and `unit` fields.
    ///
    /// # Example JSON Output
    /// ```json
    /// {"value": 42.5, "unit": "m"}
    /// ```
    pub fn serialize<U, S>(quantity: &Quantity<U>, serializer: S) -> Result<S::Ok, S::Error>
    where
        U: Unit,
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Quantity", 2)?;
        state.serialize_field("value", &quantity.value())?;
        state.serialize_field("unit", U::SYMBOL)?;
        state.end()
    }

    /// Deserializes a `Quantity<U>` from a struct with `value` and optionally `unit` fields.
    ///
    /// The `unit` field is validated if present but not required for backwards compatibility.
    /// If provided and doesn't match `U::SYMBOL`, a warning could be logged in the future.
    pub fn deserialize<'de, U, D>(deserializer: D) -> Result<Quantity<U>, D::Error>
    where
        U: Unit,
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Value,
            Unit,
        }

        struct QuantityVisitor<U>(core::marker::PhantomData<U>);

        impl<'de, U: Unit> Visitor<'de> for QuantityVisitor<U> {
            type Value = Quantity<U>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("struct Quantity with value and unit fields")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Quantity<U>, V::Error>
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

                Ok(Quantity::new(value))
            }
        }

        deserializer.deserialize_struct(
            "Quantity",
            &["value", "unit"],
            QuantityVisitor(core::marker::PhantomData),
        )
    }
}
