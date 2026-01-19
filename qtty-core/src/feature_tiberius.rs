//! Tiberius SQL Server support for `Quantity` types (feature-gated).
//!
//! This module is enabled by the `tiberius` feature. It provides SQL Server database
//! integration for `Quantity<U, f64>` types through the Tiberius driver.

use crate::{Quantity, Unit};
use tiberius::{ColumnData, FromSql, ToSql};

impl<U: Unit + Send + Sync> ToSql for Quantity<U, f64> {
    fn to_sql(&self) -> ColumnData<'_> {
        ColumnData::F64(Some(self.value()))
    }
}

impl<U: Unit> FromSql<'_> for Quantity<U, f64> {
    fn from_sql(value: &ColumnData<'_>) -> tiberius::Result<Option<Self>> {
        match value {
            ColumnData::F64(Some(val)) => Ok(Some(Quantity::new(*val))),
            ColumnData::F32(Some(val)) => Ok(Some(Quantity::new(*val as f64))),
            ColumnData::I16(Some(val)) => Ok(Some(Quantity::new(*val as f64))),
            ColumnData::I32(Some(val)) => Ok(Some(Quantity::new(*val as f64))),
            ColumnData::I64(Some(val)) => Ok(Some(Quantity::new(*val as f64))),
            ColumnData::U8(Some(val)) => Ok(Some(Quantity::new(*val as f64))),
            _ => Ok(None),
        }
    }
}
