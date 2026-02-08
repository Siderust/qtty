//! Diesel ORM support for `Quantity` types (feature-gated).
//!
//! This module is enabled by the `diesel` feature. It provides serialization, deserialization,
//! and query support for `Quantity<U, S>` types when used with Diesel ORM.
//!
//! # Supported Scalar Types
//!
//! - `f64` - maps to SQL DOUBLE PRECISION
//! - `f32` - maps to SQL REAL (FLOAT)
//!
//! Note: Decimal and Rational scalar types are not supported for Diesel integration as they
//! don't have direct SQL type representations. Use f64 or f32 for database storage.
//!
//! # Supported Operations
//!
//! - **Serialization/Deserialization**: `Quantity<U, S>` maps to SQL DOUBLE PRECISION or REAL
//! - **Nullable columns**: `Option<Quantity<U, S>>` automatically supported
//! - **Query parameters**: Use in WHERE clauses and INSERT statements
//! - **Result loading**: Use in SELECT queries with `Queryable` structs
//! - **Backend-agnostic**: Works with PostgreSQL, SQLite, MySQL, and other Diesel backends
//!
//! # Examples
//!
//! ```rust,ignore
//! use qtty::Degrees;
//! use diesel::prelude::*;
//!
//! #[derive(Queryable, Selectable)]
//! #[diesel(table_name = observations)]
//! pub struct Observation {
//!     pub id: i32,
//!     pub altitude: Degrees,        // Direct use of Quantity type
//!     pub azimuth: Degrees,
//!     pub min_altitude: Option<Degrees>,  // Optional fields work too
//! }
//!
//! #[derive(Insertable)]
//! #[diesel(table_name = observations)]
//! pub struct NewObservation {
//!     pub altitude: Degrees,
//!     pub azimuth: Degrees,
//!     pub min_altitude: Option<Degrees>,
//! }
//! ```

use crate::scalar::Real;
use crate::{Quantity, Unit};
use diesel::{
    backend::Backend,
    deserialize::{self, FromSql as DieselFromSql},
    expression::AsExpression,
    query_builder::QueryId,
    serialize::{self, Output, ToSql as DieselToSql},
    sql_types::{Double, Float, Nullable},
    Queryable,
};

// ─────────────────────────────────────────────────────────────────────────────
// Core FromSql/ToSql implementations for f64 (Double)
// ─────────────────────────────────────────────────────────────────────────────

/// Deserialize `Quantity<U, f64>` from SQL DOUBLE PRECISION (f64) for any Diesel backend.
impl<U, DB> DieselFromSql<Double, DB> for Quantity<U, f64>
where
    U: Unit,
    DB: Backend,
    f64: DieselFromSql<Double, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let value = f64::from_sql(bytes)?;
        Ok(Quantity::new(value))
    }
}

/// Serialize `Quantity<U, f64>` to SQL DOUBLE PRECISION (f64) for any Diesel backend.
impl<U, DB> DieselToSql<Double, DB> for Quantity<U, f64>
where
    U: Unit,
    DB: Backend,
    f64: DieselToSql<Double, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        <f64 as DieselToSql<Double, DB>>::to_sql(self.value_ref(), out)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Core FromSql/ToSql implementations for f32 (Float)
// ─────────────────────────────────────────────────────────────────────────────

/// Deserialize `Quantity<U, f32>` from SQL REAL (f32) for any Diesel backend.
impl<U, DB> DieselFromSql<Float, DB> for Quantity<U, f32>
where
    U: Unit,
    DB: Backend,
    f32: DieselFromSql<Float, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let value = f32::from_sql(bytes)?;
        Ok(Quantity::new(value))
    }
}

/// Serialize `Quantity<U, f32>` to SQL REAL (f32) for any Diesel backend.
impl<U, DB> DieselToSql<Float, DB> for Quantity<U, f32>
where
    U: Unit,
    DB: Backend,
    f32: DieselToSql<Float, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        <f32 as DieselToSql<Float, DB>>::to_sql(self.value_ref(), out)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Nullable column support for f64
// ─────────────────────────────────────────────────────────────────────────────

/// Support for nullable columns: `Option<Quantity<U, f64>>` maps to SQL DOUBLE PRECISION NULL.
impl<U, DB> DieselFromSql<Nullable<Double>, DB> for Quantity<U, f64>
where
    U: Unit,
    DB: Backend,
    f64: DieselFromSql<Double, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let value = f64::from_sql(bytes)?;
        Ok(Quantity::new(value))
    }
}

impl<U, DB> DieselToSql<Nullable<Double>, DB> for Quantity<U, f64>
where
    U: Unit,
    DB: Backend,
    f64: DieselToSql<Double, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        <f64 as DieselToSql<Double, DB>>::to_sql(self.value_ref(), out)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Nullable column support for f32
// ─────────────────────────────────────────────────────────────────────────────

/// Support for nullable columns: `Option<Quantity<U, f32>>` maps to SQL REAL NULL.
impl<U, DB> DieselFromSql<Nullable<Float>, DB> for Quantity<U, f32>
where
    U: Unit,
    DB: Backend,
    f32: DieselFromSql<Float, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let value = f32::from_sql(bytes)?;
        Ok(Quantity::new(value))
    }
}

impl<U, DB> DieselToSql<Nullable<Float>, DB> for Quantity<U, f32>
where
    U: Unit,
    DB: Backend,
    f32: DieselToSql<Float, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        <f32 as DieselToSql<Float, DB>>::to_sql(self.value_ref(), out)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// AsExpression implementations for f64
// ─────────────────────────────────────────────────────────────────────────────

/// Enable `Quantity<U, f64>` in WHERE clauses and INSERT statements.
impl<U: Unit> AsExpression<Double> for Quantity<U, f64> {
    type Expression = <f64 as AsExpression<Double>>::Expression;

    fn as_expression(self) -> Self::Expression {
        AsExpression::<Double>::as_expression(self.value())
    }
}

impl<U: Unit> AsExpression<Double> for &Quantity<U, f64> {
    type Expression = <f64 as AsExpression<Double>>::Expression;

    fn as_expression(self) -> Self::Expression {
        AsExpression::<Double>::as_expression(self.value())
    }
}

/// Enable `Quantity<U, f64>` in nullable (Option) columns.
impl<U: Unit> AsExpression<Nullable<Double>> for Quantity<U, f64> {
    type Expression = <f64 as AsExpression<Nullable<Double>>>::Expression;

    fn as_expression(self) -> Self::Expression {
        AsExpression::<Nullable<Double>>::as_expression(self.value())
    }
}

impl<U: Unit> AsExpression<Nullable<Double>> for &Quantity<U, f64> {
    type Expression = <f64 as AsExpression<Nullable<Double>>>::Expression;

    fn as_expression(self) -> Self::Expression {
        AsExpression::<Nullable<Double>>::as_expression(self.value())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// AsExpression implementations for f32
// ─────────────────────────────────────────────────────────────────────────────

/// Enable `Quantity<U, f32>` in WHERE clauses and INSERT statements.
impl<U: Unit> AsExpression<Float> for Quantity<U, f32> {
    type Expression = <f32 as AsExpression<Float>>::Expression;

    fn as_expression(self) -> Self::Expression {
        AsExpression::<Float>::as_expression(self.value())
    }
}

impl<U: Unit> AsExpression<Float> for &Quantity<U, f32> {
    type Expression = <f32 as AsExpression<Float>>::Expression;

    fn as_expression(self) -> Self::Expression {
        AsExpression::<Float>::as_expression(self.value())
    }
}

/// Enable `Quantity<U, f32>` in nullable (Option) columns.
impl<U: Unit> AsExpression<Nullable<Float>> for Quantity<U, f32> {
    type Expression = <f32 as AsExpression<Nullable<Float>>>::Expression;

    fn as_expression(self) -> Self::Expression {
        AsExpression::<Nullable<Float>>::as_expression(self.value())
    }
}

impl<U: Unit> AsExpression<Nullable<Float>> for &Quantity<U, f32> {
    type Expression = <f32 as AsExpression<Nullable<Float>>>::Expression;

    fn as_expression(self) -> Self::Expression {
        AsExpression::<Nullable<Float>>::as_expression(self.value())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Queryable implementations for f64
// ─────────────────────────────────────────────────────────────────────────────

/// Enable `Quantity<U, f64>` to be used in Diesel's `Queryable` derive.
impl<U, DB> Queryable<Double, DB> for Quantity<U, f64>
where
    U: Unit,
    DB: Backend,
    f64: Queryable<Double, DB>,
{
    type Row = <f64 as Queryable<Double, DB>>::Row;

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        let value = <f64 as Queryable<Double, DB>>::build(row)?;
        Ok(Quantity::new(value))
    }
}

impl<U, DB> Queryable<Nullable<Double>, DB> for Quantity<U, f64>
where
    U: Unit,
    DB: Backend,
    f64: Queryable<Nullable<Double>, DB>,
{
    type Row = <f64 as Queryable<Nullable<Double>, DB>>::Row;

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        let value = <f64 as Queryable<Nullable<Double>, DB>>::build(row)?;
        Ok(Quantity::new(value))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Queryable implementations for f32
// ─────────────────────────────────────────────────────────────────────────────

/// Enable `Quantity<U, f32>` to be used in Diesel's `Queryable` derive.
impl<U, DB> Queryable<Float, DB> for Quantity<U, f32>
where
    U: Unit,
    DB: Backend,
    f32: Queryable<Float, DB>,
{
    type Row = <f32 as Queryable<Float, DB>>::Row;

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        let value = <f32 as Queryable<Float, DB>>::build(row)?;
        Ok(Quantity::new(value))
    }
}

impl<U, DB> Queryable<Nullable<Float>, DB> for Quantity<U, f32>
where
    U: Unit,
    DB: Backend,
    f32: Queryable<Nullable<Float>, DB>,
{
    type Row = <f32 as Queryable<Nullable<Float>, DB>>::Row;

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        let value = <f32 as Queryable<Nullable<Float>, DB>>::build(row)?;
        Ok(Quantity::new(value))
    }
}

/// QueryId implementation for query caching support.
impl<U: Unit, S: Real> QueryId for Quantity<U, S> {
    type QueryId = Self;
    const HAS_STATIC_QUERY_ID: bool = false;
}
