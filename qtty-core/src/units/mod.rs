// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

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
//! - [`area`]: area units (SI square metre is canonical scaling unit) plus [`SquareOf`](area::SquareOf) alias.
//! - [`volume`]: volume units (SI cubic metre is canonical scaling unit) plus [`CubeOf`](volume::CubeOf) alias.
//! - [`velocity`]: velocity aliases (`Length / Time`) built from [`length`] and [`time`].
//! - [`acceleration`]: acceleration aliases and named SI unit (`Length / Time²`).
//! - [`force`]: force units (SI newton is canonical scaling unit).
//! - [`energy`]: energy units (SI joule is canonical scaling unit).
//! - [`power`]: power units (watt is canonical scaling unit).
//! - [`angular_rate`]: angular-rate aliases (`Angular / Time`) built from [`angular`] and [`time`].
//!   This is **not** SI Hertz-style inverse-time frequency (`T⁻¹`); see the module docs.

pub mod acceleration;
pub mod angular;
pub mod angular_rate;
pub mod area;
pub mod energy;
pub mod force;
pub mod length;
pub mod mass;
pub mod power;
pub mod time;
pub mod velocity;
pub mod volume;
