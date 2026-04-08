// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Internal macro for generating scalar-specific type aliases.
//!
//! This module is not public; each `f32`/`i8`/… module invokes the macro to
//! produce the same breadth of aliases as the default `f64` surface.

/// Generate type aliases for every built-in unit at a given scalar type.
macro_rules! define_scalar_aliases {
    ($scalar:ty) => {
        // ── Angular ──────────────────────────────────────────────────────
        /// Degree.
        pub type Degree<S = $scalar> = $crate::Quantity<$crate::unit::Degree, S>;
        /// Radian.
        pub type Radian<S = $scalar> = $crate::Quantity<$crate::unit::Radian, S>;
        /// Milliradian.
        pub type Milliradian<S = $scalar> = $crate::Quantity<$crate::unit::Milliradian, S>;
        /// Turn.
        pub type Turn<S = $scalar> = $crate::Quantity<$crate::unit::Turn, S>;

        #[cfg(feature = "astro")]
        /// Arcminute.
        pub type Arcminute<S = $scalar> = $crate::Quantity<$crate::unit::Arcminute, S>;
        #[cfg(feature = "astro")]
        /// Arcsecond.
        pub type Arcsecond<S = $scalar> = $crate::Quantity<$crate::unit::Arcsecond, S>;
        #[cfg(feature = "astro")]
        /// Milliarcseconds.
        pub type MilliArcsecond<S = $scalar> = $crate::Quantity<$crate::unit::MilliArcsecond, S>;
        #[cfg(feature = "astro")]
        /// Microarcseconds.
        pub type MicroArcsecond<S = $scalar> = $crate::Quantity<$crate::unit::MicroArcsecond, S>;
        #[cfg(feature = "astro")]
        /// Hour angles.
        pub type HourAngle<S = $scalar> = $crate::Quantity<$crate::unit::HourAngle, S>;

        #[cfg(feature = "navigation")]
        /// Gradian.
        pub type Gradian<S = $scalar> = $crate::Quantity<$crate::unit::Gradian, S>;

        // ── Length ───────────────────────────────────────────────────────
        /// Meter.
        pub type Meter<S = $scalar> = $crate::Quantity<$crate::unit::Meter, S>;
        /// Kilometer.
        pub type Kilometer<S = $scalar> = $crate::Quantity<$crate::unit::Kilometer, S>;
        /// Centimeter.
        pub type Centimeter<S = $scalar> = $crate::Quantity<$crate::unit::Centimeter, S>;
        /// Millimeter.
        pub type Millimeter<S = $scalar> = $crate::Quantity<$crate::unit::Millimeter, S>;
        /// Micrometer.
        pub type Micrometer<S = $scalar> = $crate::Quantity<$crate::unit::Micrometer, S>;
        /// Nanometer.
        pub type Nanometer<S = $scalar> = $crate::Quantity<$crate::unit::Nanometer, S>;
        /// Picometer.
        pub type Picometer<S = $scalar> = $crate::Quantity<$crate::unit::Picometer, S>;
        /// Femtometer.
        pub type Femtometer<S = $scalar> = $crate::Quantity<$crate::unit::Femtometer, S>;
        /// Attometer.
        pub type Attometer<S = $scalar> = $crate::Quantity<$crate::unit::Attometer, S>;
        /// Zeptometer.
        pub type Zeptometer<S = $scalar> = $crate::Quantity<$crate::unit::Zeptometer, S>;
        /// Yoctometer.
        pub type Yoctometer<S = $scalar> = $crate::Quantity<$crate::unit::Yoctometer, S>;
        /// Megameter.
        pub type Megameter<S = $scalar> = $crate::Quantity<$crate::unit::Megameter, S>;
        /// Decimeter.
        pub type Decimeter<S = $scalar> = $crate::Quantity<$crate::unit::Decimeter, S>;
        /// Decameter.
        pub type Decameter<S = $scalar> = $crate::Quantity<$crate::unit::Decameter, S>;
        /// Hectometer.
        pub type Hectometer<S = $scalar> = $crate::Quantity<$crate::unit::Hectometer, S>;
        /// Gigameter.
        pub type Gigameter<S = $scalar> = $crate::Quantity<$crate::unit::Gigameter, S>;
        /// Terameter.
        pub type Terameter<S = $scalar> = $crate::Quantity<$crate::unit::Terameter, S>;
        /// Petameter.
        pub type Petameter<S = $scalar> = $crate::Quantity<$crate::unit::Petameter, S>;
        /// Exameter.
        pub type Exameter<S = $scalar> = $crate::Quantity<$crate::unit::Exameter, S>;
        /// Zettameter.
        pub type Zettameter<S = $scalar> = $crate::Quantity<$crate::unit::Zettameter, S>;
        /// Yottameter.
        pub type Yottameter<S = $scalar> = $crate::Quantity<$crate::unit::Yottameter, S>;

        #[cfg(feature = "astro")]
        /// Astronomical units.
        pub type AstronomicalUnit<S = $scalar> =
            $crate::Quantity<$crate::unit::AstronomicalUnit, S>;
        #[cfg(feature = "astro")]
        /// Light years.
        pub type LightYear<S = $scalar> = $crate::Quantity<$crate::unit::LightYear, S>;
        #[cfg(feature = "astro")]
        /// Parsec.
        pub type Parsec<S = $scalar> = $crate::Quantity<$crate::unit::Parsec, S>;
        #[cfg(feature = "astro")]
        /// Kiloparsec.
        pub type Kiloparsec<S = $scalar> = $crate::Quantity<$crate::unit::Kiloparsec, S>;
        #[cfg(feature = "astro")]
        /// Megaparsec.
        pub type Megaparsec<S = $scalar> = $crate::Quantity<$crate::unit::Megaparsec, S>;
        #[cfg(feature = "astro")]
        /// Gigaparsec.
        pub type Gigaparsec<S = $scalar> = $crate::Quantity<$crate::unit::Gigaparsec, S>;

        #[cfg(feature = "customary")]
        /// Inch.
        pub type Inch<S = $scalar> = $crate::Quantity<$crate::unit::Inch, S>;
        #[cfg(feature = "customary")]
        /// Foot.
        pub type Foot<S = $scalar> = $crate::Quantity<$crate::unit::Foot, S>;
        #[cfg(feature = "customary")]
        /// Yard.
        pub type Yard<S = $scalar> = $crate::Quantity<$crate::unit::Yard, S>;
        #[cfg(feature = "customary")]
        /// Mile.
        pub type Mile<S = $scalar> = $crate::Quantity<$crate::unit::Mile, S>;

        #[cfg(feature = "navigation")]
        /// Nautical miles.
        pub type NauticalMile<S = $scalar> = $crate::Quantity<$crate::unit::NauticalMile, S>;
        #[cfg(feature = "navigation")]
        /// Chain.
        pub type Chain<S = $scalar> = $crate::Quantity<$crate::unit::Chain, S>;
        #[cfg(feature = "navigation")]
        /// Rod.
        pub type Rod<S = $scalar> = $crate::Quantity<$crate::unit::Rod, S>;
        #[cfg(feature = "navigation")]
        /// Link.
        pub type Link<S = $scalar> = $crate::Quantity<$crate::unit::Link, S>;
        #[cfg(feature = "navigation")]
        /// Fathom.
        pub type Fathom<S = $scalar> = $crate::Quantity<$crate::unit::Fathom, S>;
        #[cfg(feature = "navigation")]
        /// Earth meridional circumferences.
        pub type EarthMeridionalCircumference<S = $scalar> =
            $crate::Quantity<$crate::unit::EarthMeridionalCircumference, S>;
        #[cfg(feature = "navigation")]
        /// Earth equatorial circumferences.
        pub type EarthEquatorialCircumference<S = $scalar> =
            $crate::Quantity<$crate::unit::EarthEquatorialCircumference, S>;

        #[cfg(feature = "fundamental-physics")]
        /// Bohr radii.
        pub type BohrRadius<S = $scalar> = $crate::Quantity<$crate::unit::BohrRadius, S>;
        #[cfg(feature = "fundamental-physics")]
        /// Classical electron radii.
        pub type ClassicalElectronRadius<S = $scalar> =
            $crate::Quantity<$crate::unit::ClassicalElectronRadius, S>;
        #[cfg(feature = "fundamental-physics")]
        /// Planck lengths.
        pub type PlanckLength<S = $scalar> = $crate::Quantity<$crate::unit::PlanckLength, S>;
        #[cfg(feature = "fundamental-physics")]
        /// Electron reduced Compton wavelengths.
        pub type ElectronReducedComptonWavelength<S = $scalar> =
            $crate::Quantity<$crate::unit::ElectronReducedComptonWavelength, S>;

        // ── Length: nominal ──────────────────────────────────────────────
        #[cfg(feature = "astro")]
        /// Solar radiuses (nominal).
        pub type SolarRadius<S = $scalar> = $crate::Quantity<$crate::unit::SolarRadius, S>;
        #[cfg(feature = "astro")]
        /// Earth radii (nominal).
        pub type EarthRadius<S = $scalar> = $crate::Quantity<$crate::unit::EarthRadius, S>;
        #[cfg(feature = "astro")]
        /// Earth equatorial radii (nominal).
        pub type EarthEquatorialRadius<S = $scalar> =
            $crate::Quantity<$crate::unit::EarthEquatorialRadius, S>;
        #[cfg(feature = "astro")]
        /// Earth polar radii (nominal).
        pub type EarthPolarRadius<S = $scalar> =
            $crate::Quantity<$crate::unit::EarthPolarRadius, S>;
        #[cfg(feature = "astro")]
        /// Lunar radii (nominal).
        pub type LunarRadius<S = $scalar> = $crate::Quantity<$crate::unit::LunarRadius, S>;
        #[cfg(feature = "astro")]
        /// Jupiter radii (nominal).
        pub type JupiterRadius<S = $scalar> = $crate::Quantity<$crate::unit::JupiterRadius, S>;
        #[cfg(feature = "astro")]
        /// Lunar distances (nominal).
        pub type LunarDistance<S = $scalar> = $crate::Quantity<$crate::unit::LunarDistance, S>;
        #[cfg(feature = "astro")]
        /// Solar diameters (nominal).
        pub type SolarDiameter<S = $scalar> = $crate::Quantity<$crate::unit::SolarDiameter, S>;

        // ── Time ─────────────────────────────────────────────────────────
        /// Attosecond.
        pub type Attosecond<S = $scalar> = $crate::Quantity<$crate::unit::Attosecond, S>;
        /// Femtosecond.
        pub type Femtosecond<S = $scalar> = $crate::Quantity<$crate::unit::Femtosecond, S>;
        /// Picosecond.
        pub type Picosecond<S = $scalar> = $crate::Quantity<$crate::unit::Picosecond, S>;
        /// Nanosecond.
        pub type Nanosecond<S = $scalar> = $crate::Quantity<$crate::unit::Nanosecond, S>;
        /// Microsecond.
        pub type Microsecond<S = $scalar> = $crate::Quantity<$crate::unit::Microsecond, S>;
        /// Millisecond.
        pub type Millisecond<S = $scalar> = $crate::Quantity<$crate::unit::Millisecond, S>;
        /// Centisecond.
        pub type Centisecond<S = $scalar> = $crate::Quantity<$crate::unit::Centisecond, S>;
        /// Decisecond.
        pub type Decisecond<S = $scalar> = $crate::Quantity<$crate::unit::Decisecond, S>;
        /// Second.
        pub type Second<S = $scalar> = $crate::Quantity<$crate::unit::Second, S>;
        /// Decasecond.
        pub type Decasecond<S = $scalar> = $crate::Quantity<$crate::unit::Decasecond, S>;
        /// Hectosecond.
        pub type Hectosecond<S = $scalar> = $crate::Quantity<$crate::unit::Hectosecond, S>;
        /// Kilosecond.
        pub type Kilosecond<S = $scalar> = $crate::Quantity<$crate::unit::Kilosecond, S>;
        /// Megasecond.
        pub type Megasecond<S = $scalar> = $crate::Quantity<$crate::unit::Megasecond, S>;
        /// Gigasecond.
        pub type Gigasecond<S = $scalar> = $crate::Quantity<$crate::unit::Gigasecond, S>;
        /// Terasecond.
        pub type Terasecond<S = $scalar> = $crate::Quantity<$crate::unit::Terasecond, S>;
        /// Minute.
        pub type Minute<S = $scalar> = $crate::Quantity<$crate::unit::Minute, S>;
        /// Hour.
        pub type Hour<S = $scalar> = $crate::Quantity<$crate::unit::Hour, S>;
        /// Day.
        pub type Day<S = $scalar> = $crate::Quantity<$crate::unit::Day, S>;
        /// Week.
        pub type Week<S = $scalar> = $crate::Quantity<$crate::unit::Week, S>;
        /// Fortnight.
        pub type Fortnight<S = $scalar> = $crate::Quantity<$crate::unit::Fortnight, S>;
        /// Year.
        pub type Year<S = $scalar> = $crate::Quantity<$crate::unit::Year, S>;
        /// Decade.
        pub type Decade<S = $scalar> = $crate::Quantity<$crate::unit::Decade, S>;
        /// Century.
        pub type Century<S = $scalar> = $crate::Quantity<$crate::unit::Century, S>;
        /// Millennium.
        pub type Millennium<S = $scalar> = $crate::Quantity<$crate::unit::Millennium, S>;
        #[cfg(feature = "julian-time")]
        /// Julian years.
        pub type JulianYear<S = $scalar> = $crate::Quantity<$crate::unit::JulianYear, S>;
        #[cfg(feature = "julian-time")]
        /// Julian centuries.
        pub type JulianCentury<S = $scalar> = $crate::Quantity<$crate::unit::JulianCentury, S>;
        #[cfg(feature = "astro")]
        /// Sidereal days.
        pub type SiderealDay<S = $scalar> = $crate::Quantity<$crate::unit::SiderealDay, S>;
        #[cfg(feature = "astro")]
        /// Synodic months.
        pub type SynodicMonth<S = $scalar> = $crate::Quantity<$crate::unit::SynodicMonth, S>;
        #[cfg(feature = "astro")]
        /// Sidereal years.
        pub type SiderealYear<S = $scalar> = $crate::Quantity<$crate::unit::SiderealYear, S>;

        // ── Mass ─────────────────────────────────────────────────────────
        /// Gram.
        pub type Gram<S = $scalar> = $crate::Quantity<$crate::unit::Gram, S>;
        /// Yoctogram.
        pub type Yoctogram<S = $scalar> = $crate::Quantity<$crate::unit::Yoctogram, S>;
        /// Zeptogram.
        pub type Zeptogram<S = $scalar> = $crate::Quantity<$crate::unit::Zeptogram, S>;
        /// Attogram.
        pub type Attogram<S = $scalar> = $crate::Quantity<$crate::unit::Attogram, S>;
        /// Femtogram.
        pub type Femtogram<S = $scalar> = $crate::Quantity<$crate::unit::Femtogram, S>;
        /// Picogram.
        pub type Picogram<S = $scalar> = $crate::Quantity<$crate::unit::Picogram, S>;
        /// Nanogram.
        pub type Nanogram<S = $scalar> = $crate::Quantity<$crate::unit::Nanogram, S>;
        /// Microgram.
        pub type Microgram<S = $scalar> = $crate::Quantity<$crate::unit::Microgram, S>;
        /// Milligram.
        pub type Milligram<S = $scalar> = $crate::Quantity<$crate::unit::Milligram, S>;
        /// Centigram.
        pub type Centigram<S = $scalar> = $crate::Quantity<$crate::unit::Centigram, S>;
        /// Decigram.
        pub type Decigram<S = $scalar> = $crate::Quantity<$crate::unit::Decigram, S>;
        /// Decagram.
        pub type Decagram<S = $scalar> = $crate::Quantity<$crate::unit::Decagram, S>;
        /// Hectogram.
        pub type Hectogram<S = $scalar> = $crate::Quantity<$crate::unit::Hectogram, S>;
        /// Kilogram.
        pub type Kilogram<S = $scalar> = $crate::Quantity<$crate::unit::Kilogram, S>;
        /// Megagram.
        pub type Megagram<S = $scalar> = $crate::Quantity<$crate::unit::Megagram, S>;
        /// Gigagram.
        pub type Gigagram<S = $scalar> = $crate::Quantity<$crate::unit::Gigagram, S>;
        /// Teragram.
        pub type Teragram<S = $scalar> = $crate::Quantity<$crate::unit::Teragram, S>;
        /// Petagram.
        pub type Petagram<S = $scalar> = $crate::Quantity<$crate::unit::Petagram, S>;
        /// Exagram.
        pub type Exagram<S = $scalar> = $crate::Quantity<$crate::unit::Exagram, S>;
        /// Zettagram.
        pub type Zettagram<S = $scalar> = $crate::Quantity<$crate::unit::Zettagram, S>;
        /// Yottagram.
        pub type Yottagram<S = $scalar> = $crate::Quantity<$crate::unit::Yottagram, S>;
        /// Tonne (metric tons).
        pub type Tonne<S = $scalar> = $crate::Quantity<$crate::unit::Tonne, S>;
        #[cfg(feature = "customary")]
        /// Carat.
        pub type Carat<S = $scalar> = $crate::Quantity<$crate::unit::Carat, S>;
        #[cfg(feature = "customary")]
        /// Grain.
        pub type Grain<S = $scalar> = $crate::Quantity<$crate::unit::Grain, S>;
        #[cfg(feature = "customary")]
        /// Pound.
        pub type Pound<S = $scalar> = $crate::Quantity<$crate::unit::Pound, S>;
        #[cfg(feature = "customary")]
        /// Ounce.
        pub type Ounce<S = $scalar> = $crate::Quantity<$crate::unit::Ounce, S>;
        #[cfg(feature = "customary")]
        /// Stone.
        pub type Stone<S = $scalar> = $crate::Quantity<$crate::unit::Stone, S>;
        #[cfg(feature = "customary")]
        /// Short tons (US tons).
        pub type ShortTon<S = $scalar> = $crate::Quantity<$crate::unit::ShortTon, S>;
        #[cfg(feature = "customary")]
        /// Long tons (Imperial tons).
        pub type LongTon<S = $scalar> = $crate::Quantity<$crate::unit::LongTon, S>;
        #[cfg(feature = "fundamental-physics")]
        /// Atomic mass units (daltons).
        pub type AtomicMassUnit<S = $scalar> = $crate::Quantity<$crate::unit::AtomicMassUnit, S>;
        #[cfg(feature = "astro")]
        /// Solar masses.
        pub type SolarMass<S = $scalar> = $crate::Quantity<$crate::unit::SolarMass, S>;

        // ── Power ────────────────────────────────────────────────────────
        /// Watt.
        pub type Watt<S = $scalar> = $crate::Quantity<$crate::unit::Watt, S>;
        /// Yoctowatt.
        pub type Yoctowatt<S = $scalar> = $crate::Quantity<$crate::unit::Yoctowatt, S>;
        /// Zeptowatt.
        pub type Zeptowatt<S = $scalar> = $crate::Quantity<$crate::unit::Zeptowatt, S>;
        /// Attowatt.
        pub type Attowatt<S = $scalar> = $crate::Quantity<$crate::unit::Attowatt, S>;
        /// Femtowatt.
        pub type Femtowatt<S = $scalar> = $crate::Quantity<$crate::unit::Femtowatt, S>;
        /// Picowatt.
        pub type Picowatt<S = $scalar> = $crate::Quantity<$crate::unit::Picowatt, S>;
        /// Nanowatt.
        pub type Nanowatt<S = $scalar> = $crate::Quantity<$crate::unit::Nanowatt, S>;
        /// Microwatt.
        pub type Microwatt<S = $scalar> = $crate::Quantity<$crate::unit::Microwatt, S>;
        /// Milliwatt.
        pub type Milliwatt<S = $scalar> = $crate::Quantity<$crate::unit::Milliwatt, S>;
        /// Deciwatt.
        pub type Deciwatt<S = $scalar> = $crate::Quantity<$crate::unit::Deciwatt, S>;
        /// Decawatt.
        pub type Decawatt<S = $scalar> = $crate::Quantity<$crate::unit::Decawatt, S>;
        /// Hectowatt.
        pub type Hectowatt<S = $scalar> = $crate::Quantity<$crate::unit::Hectowatt, S>;
        /// Kilowatt.
        pub type Kilowatt<S = $scalar> = $crate::Quantity<$crate::unit::Kilowatt, S>;
        /// Megawatt.
        pub type Megawatt<S = $scalar> = $crate::Quantity<$crate::unit::Megawatt, S>;
        /// Gigawatt.
        pub type Gigawatt<S = $scalar> = $crate::Quantity<$crate::unit::Gigawatt, S>;
        /// Terawatt.
        pub type Terawatt<S = $scalar> = $crate::Quantity<$crate::unit::Terawatt, S>;
        /// Petawatt.
        pub type Petawatt<S = $scalar> = $crate::Quantity<$crate::unit::Petawatt, S>;
        /// Exawatt.
        pub type Exawatt<S = $scalar> = $crate::Quantity<$crate::unit::Exawatt, S>;
        /// Zettawatt.
        pub type Zettawatt<S = $scalar> = $crate::Quantity<$crate::unit::Zettawatt, S>;
        /// Yottawatt.
        pub type Yottawatt<S = $scalar> = $crate::Quantity<$crate::unit::Yottawatt, S>;
        #[cfg(feature = "fundamental-physics")]
        /// Ergs per second.
        pub type ErgPerSecond<S = $scalar> = $crate::Quantity<$crate::unit::ErgPerSecond, S>;
        #[cfg(feature = "customary")]
        /// Horsepower (metric).
        pub type HorsepowerMetric<S = $scalar> =
            $crate::Quantity<$crate::unit::HorsepowerMetric, S>;
        #[cfg(feature = "customary")]
        /// Horsepower (electric).
        pub type HorsepowerElectric<S = $scalar> =
            $crate::Quantity<$crate::unit::HorsepowerElectric, S>;
        #[cfg(feature = "astro")]
        /// Solar luminosities.
        pub type SolarLuminosity<S = $scalar> = $crate::Quantity<$crate::unit::SolarLuminosity, S>;

        // ── Area ─────────────────────────────────────────────────────────
        /// Square meters.
        pub type SquareMeter<S = $scalar> = $crate::Quantity<$crate::unit::SquareMeter, S>;
        /// Square kilometers.
        pub type SquareKilometer<S = $scalar> = $crate::Quantity<$crate::unit::SquareKilometer, S>;
        /// Square centimeters.
        pub type SquareCentimeter<S = $scalar> =
            $crate::Quantity<$crate::unit::SquareCentimeter, S>;
        /// Square millimeters.
        pub type SquareMillimeter<S = $scalar> =
            $crate::Quantity<$crate::unit::SquareMillimeter, S>;
        #[cfg(feature = "land-area")]
        /// Hectare.
        pub type Hectare<S = $scalar> = $crate::Quantity<$crate::unit::Hectare, S>;
        #[cfg(feature = "land-area")]
        /// Are.
        pub type Are<S = $scalar> = $crate::Quantity<$crate::unit::Are, S>;
        #[cfg(feature = "customary")]
        /// Square inches.
        pub type SquareInch<S = $scalar> = $crate::Quantity<$crate::unit::SquareInch, S>;
        #[cfg(feature = "customary")]
        /// Square feet.
        pub type SquareFoot<S = $scalar> = $crate::Quantity<$crate::unit::SquareFoot, S>;
        #[cfg(feature = "customary")]
        /// Square yards.
        pub type SquareYard<S = $scalar> = $crate::Quantity<$crate::unit::SquareYard, S>;
        #[cfg(feature = "customary")]
        /// Square miles.
        pub type SquareMile<S = $scalar> = $crate::Quantity<$crate::unit::SquareMile, S>;
        #[cfg(feature = "land-area")]
        /// Acre.
        pub type Acre<S = $scalar> = $crate::Quantity<$crate::unit::Acre, S>;

        // ── Volume ───────────────────────────────────────────────────────
        /// Cubic meters.
        pub type CubicMeter<S = $scalar> = $crate::Quantity<$crate::unit::CubicMeter, S>;
        /// Cubic kilometers.
        pub type CubicKilometer<S = $scalar> = $crate::Quantity<$crate::unit::CubicKilometer, S>;
        /// Cubic centimeters.
        pub type CubicCentimeter<S = $scalar> = $crate::Quantity<$crate::unit::CubicCentimeter, S>;
        /// Cubic millimeters.
        pub type CubicMillimeter<S = $scalar> = $crate::Quantity<$crate::unit::CubicMillimeter, S>;
        /// Liter.
        pub type Liter<S = $scalar> = $crate::Quantity<$crate::unit::Liter, S>;
        /// Milliliter.
        pub type Milliliter<S = $scalar> = $crate::Quantity<$crate::unit::Milliliter, S>;
        /// Microliter.
        pub type Microliter<S = $scalar> = $crate::Quantity<$crate::unit::Microliter, S>;
        /// Centiliter.
        pub type Centiliter<S = $scalar> = $crate::Quantity<$crate::unit::Centiliter, S>;
        /// Deciliter.
        pub type Deciliter<S = $scalar> = $crate::Quantity<$crate::unit::Deciliter, S>;
        #[cfg(feature = "customary")]
        /// Cubic inches.
        pub type CubicInch<S = $scalar> = $crate::Quantity<$crate::unit::CubicInch, S>;
        #[cfg(feature = "customary")]
        /// Cubic feet.
        pub type CubicFoot<S = $scalar> = $crate::Quantity<$crate::unit::CubicFoot, S>;
        #[cfg(feature = "customary")]
        /// US gallons.
        pub type UsGallon<S = $scalar> = $crate::Quantity<$crate::unit::UsGallon, S>;
        #[cfg(feature = "customary")]
        /// US fluid ounces.
        pub type UsFluidOunce<S = $scalar> = $crate::Quantity<$crate::unit::UsFluidOunce, S>;
    };
}

pub(crate) use define_scalar_aliases;
