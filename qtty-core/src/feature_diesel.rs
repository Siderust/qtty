//! Diesel ORM support for `Quantity` types (feature-gated).
//!
//! This module is enabled by the `diesel` feature. It provides serialization, deserialization,
//! and query support for `Quantity<U>` types when used with Diesel ORM.
//!
//! # Supported Operations
//!
//! - **Serialization/Deserialization**: `Quantity<U>` maps to SQL DOUBLE PRECISION (Float8)
//! - **Nullable columns**: `Option<Quantity<U>>` automatically supported
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

use crate::{Quantity, Unit};
use diesel::{
    backend::Backend,
    deserialize::{self, FromSql as DieselFromSql},
    expression::AsExpression,
    query_builder::QueryId,
    serialize::{self, Output, ToSql as DieselToSql},
    sql_types::{Double, Nullable},
    Queryable,
};

// ─────────────────────────────────────────────────────────────────────────────
// Core FromSql/ToSql implementations
// ─────────────────────────────────────────────────────────────────────────────

/// Deserialize `Quantity<U>` from SQL DOUBLE PRECISION (f64) for any Diesel backend.
impl<U, DB> DieselFromSql<Double, DB> for Quantity<U>
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

/// Serialize `Quantity<U>` to SQL DOUBLE PRECISION (f64) for any Diesel backend.
impl<U, DB> DieselToSql<Double, DB> for Quantity<U>
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
// Nullable column support
// ─────────────────────────────────────────────────────────────────────────────

/// Support for nullable columns: `Option<Quantity<U>>` maps to SQL DOUBLE PRECISION NULL.
impl<U, DB> DieselFromSql<Nullable<Double>, DB> for Quantity<U>
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

impl<U, DB> DieselToSql<Nullable<Double>, DB> for Quantity<U>
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
// AsExpression implementations for query parameters
// ─────────────────────────────────────────────────────────────────────────────

/// Enable `Quantity<U>` in WHERE clauses and INSERT statements.
impl<U: Unit> AsExpression<Double> for Quantity<U> {
    type Expression = <f64 as AsExpression<Double>>::Expression;

    fn as_expression(self) -> Self::Expression {
        AsExpression::<Double>::as_expression(self.value())
    }
}

impl<'a, U: Unit> AsExpression<Double> for &'a Quantity<U> {
    type Expression = <f64 as AsExpression<Double>>::Expression;

    fn as_expression(self) -> Self::Expression {
        AsExpression::<Double>::as_expression(self.value())
    }
}

/// Enable `Quantity<U>` in nullable (Option) columns.
impl<U: Unit> AsExpression<Nullable<Double>> for Quantity<U> {
    type Expression = <f64 as AsExpression<Nullable<Double>>>::Expression;

    fn as_expression(self) -> Self::Expression {
        AsExpression::<Nullable<Double>>::as_expression(self.value())
    }
}

impl<'a, U: Unit> AsExpression<Nullable<Double>> for &'a Quantity<U> {
    type Expression = <f64 as AsExpression<Nullable<Double>>>::Expression;

    fn as_expression(self) -> Self::Expression {
        AsExpression::<Nullable<Double>>::as_expression(self.value())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Queryable implementations for SELECT queries
// ─────────────────────────────────────────────────────────────────────────────

/// Enable `Quantity<U>` to be used in Diesel's `Queryable` derive.
///
/// This allows structs with `Quantity<U>` fields to derive `Queryable` and be loaded
/// from database query results.
impl<U, DB> Queryable<Double, DB> for Quantity<U>
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

impl<U, DB> Queryable<Nullable<Double>, DB> for Quantity<U>
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

/// QueryId implementation for query caching support.
impl<U: Unit> QueryId for Quantity<U> {
    type QueryId = Self;
    const HAS_STATIC_QUERY_ID: bool = false;
}
