// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

// --- Astronomical mean units (explicitly approximate) ---

/// Mean sidereal day (Earth), expressed in SI seconds.
///
/// Convention used: `1 sidereal day ≈ 86_164.0905 s`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "sd", dimension = Time, ratio = 86_164.090_5)]
pub struct SiderealDay;
/// A quantity measured in sidereal days.
pub type SiderealDays = Quantity<SiderealDay>;
/// A constant representing one sidereal day.
pub const SIDEREAL_DAY: SiderealDays = SiderealDays::new(1.0);

/// Mean synodic month (lunar phase cycle), expressed in seconds.
///
/// Convention used: `1 synodic month ≈ 29.530588 d`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "synmo", dimension = Time, ratio = 29.530_588 * SECONDS_PER_DAY)]
pub struct SynodicMonth;
/// A quantity measured in synodic months.
pub type SynodicMonths = Quantity<SynodicMonth>;
/// A constant representing one synodic month.
pub const SYNODIC_MONTH: SynodicMonths = SynodicMonths::new(1.0);

/// Mean sidereal year (Earth), expressed in seconds.
///
/// Common convention: `1 sidereal year ≈ 365.256363004 d`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "syr", dimension = Time, ratio = 365.256_363_004 * SECONDS_PER_DAY)]
pub struct SiderealYear;
/// A quantity measured in sidereal years.
pub type SiderealYears = Quantity<SiderealYear>;
/// A constant representing one sidereal year.
pub const SIDEREAL_YEAR: SiderealYears = SiderealYears::new(1.0);
