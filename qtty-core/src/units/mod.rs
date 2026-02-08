//! Predefined unit modules grouped by dimension.
//!
//! `qtty-core` ships a small set of built-in units so that conversions and formatting work out of the box without
//! downstream crates having to fight Rust’s orphan rules.
//!
//! ## Modules
//!
//! - [`angular`]: angle units plus wrapping and trig helpers.
//! - [`time`]: time units (SI second is canonical scaling unit).
//! - [`length`]: length units (SI metre is canonical scaling unit) plus astronomy/geodesy helpers.
//! - [`mass`]: mass units (gram is canonical scaling unit).
//! - [`power`]: power units (watt is canonical scaling unit).
//! - [`velocity`]: velocity aliases (`Length / Time`) built from [`length`] and [`time`].
//! - [`frequency`]: angular frequency aliases (`Angular / Time`) built from [`angular`] and [`time`].
//! - [`unitless`]: helpers for dimensionless quantities.

pub mod angular;
pub mod area;
pub mod frequency;
pub mod length;
pub mod mass;
pub mod power;
pub mod time;
pub mod unitless;
pub mod velocity;
pub mod volume;
