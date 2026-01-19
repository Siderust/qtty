//! Tiberius SQL Server support for `Quantity` types (feature-gated).
//!
//! This module is enabled by the `tiberius` feature. It provides SQL Server database
//! integration for `Quantity<U, S>` types through the Tiberius driver.

use crate::{Quantity, Unit};
use crate::scalar::Real;
use tiberius::{ColumnData, FromSql, ToSql};

impl<U: Unit + Send + Sync, S: Real + Send + Sync> ToSql for Quantity<U, S> {
    fn to_sql(&self) -> ColumnData<'_> {
        ColumnData::F64(Some(self.value().to_f64()))
    }
}

impl<U: Unit, S: Real> FromSql<'_> for Quantity<U, S> {
    fn from_sql(value: &ColumnData<'_>) -> tiberius::Result<Option<Self>> {
        match value {
            ColumnData::F64(Some(val)) => Ok(Some(Quantity::new(S::from_f64(*val)))),
            ColumnData::F32(Some(val)) => Ok(Some(Quantity::new(S::from_f64(*val as f64)))),
            ColumnData::I16(Some(val)) => Ok(Some(Quantity::new(S::from_f64(*val as f64)))),
            ColumnData::I32(Some(val)) => Ok(Some(Quantity::new(S::from_f64(*val as f64)))),
            ColumnData::I64(Some(val)) => Ok(Some(Quantity::new(S::from_f64(*val as f64)))),
            ColumnData::U8(Some(val)) => Ok(Some(Quantity::new(S::from_f64(*val as f64)))),
            _ => Ok(None),
        }
    }
}
