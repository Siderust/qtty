//! Internal macro for generating scalar-specific type aliases.
//!
//! This module is not public; each `f32`/`i8`/… module invokes the macro to
//! produce the same breadth of aliases as the default `f64` surface.

/// Generate type aliases for every built-in unit at a given scalar type.
macro_rules! define_scalar_aliases {
    ($scalar:ty) => {
        use $crate::Quantity;

        // ── Angular ──────────────────────────────────────────────────────
        /// Degrees.
        pub type Degrees = Quantity<$crate::angular::Degree, $scalar>;
        /// Radians.
        pub type Radians = Quantity<$crate::angular::Radian, $scalar>;
        /// Arcminutes.
        pub type Arcminutes = Quantity<$crate::angular::Arcminute, $scalar>;
        /// Arcseconds.
        pub type Arcseconds = Quantity<$crate::angular::Arcsecond, $scalar>;
        /// Milliradians.
        pub type Milliradians = Quantity<$crate::angular::Milliradian, $scalar>;
        /// Milliarcseconds.
        pub type MilliArcseconds = Quantity<$crate::angular::MilliArcsecond, $scalar>;
        /// Microarcseconds.
        pub type MicroArcseconds = Quantity<$crate::angular::MicroArcsecond, $scalar>;
        /// Gradians.
        pub type Gradians = Quantity<$crate::angular::Gradian, $scalar>;
        /// Turns.
        pub type Turns = Quantity<$crate::angular::Turn, $scalar>;
        /// Hour angles.
        pub type HourAngles = Quantity<$crate::angular::HourAngle, $scalar>;

        // ── Length ───────────────────────────────────────────────────────
        /// Meters.
        pub type Meters = Quantity<$crate::length::Meter, $scalar>;
        /// Kilometers.
        pub type Kilometers = Quantity<$crate::length::Kilometer, $scalar>;
        /// Centimeters.
        pub type Centimeters = Quantity<$crate::length::Centimeter, $scalar>;
        /// Millimeters.
        pub type Millimeters = Quantity<$crate::length::Millimeter, $scalar>;
        /// Micrometers.
        pub type Micrometers = Quantity<$crate::length::Micrometer, $scalar>;
        /// Nanometers.
        pub type Nanometers = Quantity<$crate::length::Nanometer, $scalar>;
        /// Picometers.
        pub type Picometers = Quantity<$crate::length::Picometer, $scalar>;
        /// Femtometers.
        pub type Femtometers = Quantity<$crate::length::Femtometer, $scalar>;
        /// Attometers.
        pub type Attometers = Quantity<$crate::length::Attometer, $scalar>;
        /// Zeptometers.
        pub type Zeptometers = Quantity<$crate::length::Zeptometer, $scalar>;
        /// Yoctometers.
        pub type Yoctometers = Quantity<$crate::length::Yoctometer, $scalar>;
        /// Megameters.
        pub type Megameters = Quantity<$crate::length::Megameter, $scalar>;
        /// Decimeters.
        pub type Decimeters = Quantity<$crate::length::Decimeter, $scalar>;
        /// Decameters.
        pub type Decameters = Quantity<$crate::length::Decameter, $scalar>;
        /// Hectometers.
        pub type Hectometers = Quantity<$crate::length::Hectometer, $scalar>;
        /// Gigameters.
        pub type Gigameters = Quantity<$crate::length::Gigameter, $scalar>;
        /// Terameters.
        pub type Terameters = Quantity<$crate::length::Terameter, $scalar>;
        /// Petameters.
        pub type Petameters = Quantity<$crate::length::Petameter, $scalar>;
        /// Exameters.
        pub type Exameters = Quantity<$crate::length::Exameter, $scalar>;
        /// Zettameters.
        pub type Zettameters = Quantity<$crate::length::Zettameter, $scalar>;
        /// Yottameters.
        pub type Yottameters = Quantity<$crate::length::Yottameter, $scalar>;
        /// Astronomical units.
        pub type AstronomicalUnits = Quantity<$crate::length::AstronomicalUnit, $scalar>;
        /// Light years.
        pub type LightYears = Quantity<$crate::length::LightYear, $scalar>;
        /// Parsecs.
        pub type Parsecs = Quantity<$crate::length::Parsec, $scalar>;
        /// Kiloparsecs.
        pub type Kiloparsecs = Quantity<$crate::length::Kiloparsec, $scalar>;
        /// Megaparsecs.
        pub type Megaparsecs = Quantity<$crate::length::Megaparsec, $scalar>;
        /// Gigaparsecs.
        pub type Gigaparsecs = Quantity<$crate::length::Gigaparsec, $scalar>;
        /// Inches.
        pub type Inches = Quantity<$crate::length::Inch, $scalar>;
        /// Feet.
        pub type Feet = Quantity<$crate::length::Foot, $scalar>;
        /// Yards.
        pub type Yards = Quantity<$crate::length::Yard, $scalar>;
        /// Miles.
        pub type Miles = Quantity<$crate::length::Mile, $scalar>;
        /// Nautical miles.
        pub type NauticalMiles = Quantity<$crate::length::NauticalMile, $scalar>;
        /// Chains.
        pub type Chains = Quantity<$crate::length::Chain, $scalar>;
        /// Rods.
        pub type Rods = Quantity<$crate::length::Rod, $scalar>;
        /// Links.
        pub type Links = Quantity<$crate::length::Link, $scalar>;
        /// Fathoms.
        pub type Fathoms = Quantity<$crate::length::Fathom, $scalar>;
        /// Earth meridional circumferences.
        pub type EarthMeridionalCircumferences = Quantity<$crate::length::EarthMeridionalCircumference, $scalar>;
        /// Earth equatorial circumferences.
        pub type EarthEquatorialCircumferences = Quantity<$crate::length::EarthEquatorialCircumference, $scalar>;
        /// Bohr radii.
        pub type BohrRadii = Quantity<$crate::length::BohrRadius, $scalar>;
        /// Classical electron radii.
        pub type ClassicalElectronRadii = Quantity<$crate::length::ClassicalElectronRadius, $scalar>;
        /// Planck lengths.
        pub type PlanckLengths = Quantity<$crate::length::PlanckLength, $scalar>;
        /// Electron reduced Compton wavelengths.
        pub type ElectronReducedComptonWavelengths = Quantity<$crate::length::ElectronReducedComptonWavelength, $scalar>;

        // ── Length: nominal ──────────────────────────────────────────────
        /// Solar radiuses (nominal).
        pub type SolarRadiuses = Quantity<$crate::length::nominal::SolarRadius, $scalar>;
        /// Earth radii (nominal).
        pub type EarthRadii = Quantity<$crate::length::nominal::EarthRadius, $scalar>;
        /// Earth equatorial radii (nominal).
        pub type EarthEquatorialRadii = Quantity<$crate::length::nominal::EarthEquatorialRadius, $scalar>;
        /// Earth polar radii (nominal).
        pub type EarthPolarRadii = Quantity<$crate::length::nominal::EarthPolarRadius, $scalar>;
        /// Lunar radii (nominal).
        pub type LunarRadii = Quantity<$crate::length::nominal::LunarRadius, $scalar>;
        /// Jupiter radii (nominal).
        pub type JupiterRadii = Quantity<$crate::length::nominal::JupiterRadius, $scalar>;
        /// Lunar distances (nominal).
        pub type LunarDistances = Quantity<$crate::length::nominal::LunarDistance, $scalar>;
        /// Solar diameters (nominal).
        pub type SolarDiameters = Quantity<$crate::length::nominal::SolarDiameter, $scalar>;

        // ── Time ─────────────────────────────────────────────────────────
        /// Attoseconds.
        pub type Attoseconds = Quantity<$crate::time::Attosecond, $scalar>;
        /// Femtoseconds.
        pub type Femtoseconds = Quantity<$crate::time::Femtosecond, $scalar>;
        /// Picoseconds.
        pub type Picoseconds = Quantity<$crate::time::Picosecond, $scalar>;
        /// Nanoseconds.
        pub type Nanoseconds = Quantity<$crate::time::Nanosecond, $scalar>;
        /// Microseconds.
        pub type Microseconds = Quantity<$crate::time::Microsecond, $scalar>;
        /// Milliseconds.
        pub type Milliseconds = Quantity<$crate::time::Millisecond, $scalar>;
        /// Centiseconds.
        pub type Centiseconds = Quantity<$crate::time::Centisecond, $scalar>;
        /// Deciseconds.
        pub type Deciseconds = Quantity<$crate::time::Decisecond, $scalar>;
        /// Seconds.
        pub type Seconds = Quantity<$crate::time::Second, $scalar>;
        /// Decaseconds.
        pub type Decaseconds = Quantity<$crate::time::Decasecond, $scalar>;
        /// Hectoseconds.
        pub type Hectoseconds = Quantity<$crate::time::Hectosecond, $scalar>;
        /// Kiloseconds.
        pub type Kiloseconds = Quantity<$crate::time::Kilosecond, $scalar>;
        /// Megaseconds.
        pub type Megaseconds = Quantity<$crate::time::Megasecond, $scalar>;
        /// Gigaseconds.
        pub type Gigaseconds = Quantity<$crate::time::Gigasecond, $scalar>;
        /// Teraseconds.
        pub type Teraseconds = Quantity<$crate::time::Terasecond, $scalar>;
        /// Minutes.
        pub type Minutes = Quantity<$crate::time::Minute, $scalar>;
        /// Hours.
        pub type Hours = Quantity<$crate::time::Hour, $scalar>;
        /// Days.
        pub type Days = Quantity<$crate::time::Day, $scalar>;
        /// Weeks.
        pub type Weeks = Quantity<$crate::time::Week, $scalar>;
        /// Fortnights.
        pub type Fortnights = Quantity<$crate::time::Fortnight, $scalar>;
        /// Years.
        pub type Years = Quantity<$crate::time::Year, $scalar>;
        /// Decades.
        pub type Decades = Quantity<$crate::time::Decade, $scalar>;
        /// Centuries.
        pub type Centuries = Quantity<$crate::time::Century, $scalar>;
        /// Millennia.
        pub type Millennia = Quantity<$crate::time::Millennium, $scalar>;
        /// Julian years.
        pub type JulianYears = Quantity<$crate::time::JulianYear, $scalar>;
        /// Julian centuries.
        pub type JulianCenturies = Quantity<$crate::time::JulianCentury, $scalar>;
        /// Sidereal days.
        pub type SiderealDays = Quantity<$crate::time::SiderealDay, $scalar>;
        /// Synodic months.
        pub type SynodicMonths = Quantity<$crate::time::SynodicMonth, $scalar>;
        /// Sidereal years.
        pub type SiderealYears = Quantity<$crate::time::SiderealYear, $scalar>;

        // ── Mass ─────────────────────────────────────────────────────────
        /// Grams.
        pub type Grams = Quantity<$crate::mass::Gram, $scalar>;
        /// Yoctograms.
        pub type Yoctograms = Quantity<$crate::mass::Yoctogram, $scalar>;
        /// Zeptograms.
        pub type Zeptograms = Quantity<$crate::mass::Zeptogram, $scalar>;
        /// Attograms.
        pub type Attograms = Quantity<$crate::mass::Attogram, $scalar>;
        /// Femtograms.
        pub type Femtograms = Quantity<$crate::mass::Femtogram, $scalar>;
        /// Picograms.
        pub type Picograms = Quantity<$crate::mass::Picogram, $scalar>;
        /// Nanograms.
        pub type Nanograms = Quantity<$crate::mass::Nanogram, $scalar>;
        /// Micrograms.
        pub type Micrograms = Quantity<$crate::mass::Microgram, $scalar>;
        /// Milligrams.
        pub type Milligrams = Quantity<$crate::mass::Milligram, $scalar>;
        /// Centigrams.
        pub type Centigrams = Quantity<$crate::mass::Centigram, $scalar>;
        /// Decigrams.
        pub type Decigrams = Quantity<$crate::mass::Decigram, $scalar>;
        /// Decagrams.
        pub type Decagrams = Quantity<$crate::mass::Decagram, $scalar>;
        /// Hectograms.
        pub type Hectograms = Quantity<$crate::mass::Hectogram, $scalar>;
        /// Kilograms.
        pub type Kilograms = Quantity<$crate::mass::Kilogram, $scalar>;
        /// Megagrams.
        pub type Megagrams = Quantity<$crate::mass::Megagram, $scalar>;
        /// Gigagrams.
        pub type Gigagrams = Quantity<$crate::mass::Gigagram, $scalar>;
        /// Teragrams.
        pub type Teragrams = Quantity<$crate::mass::Teragram, $scalar>;
        /// Petagrams.
        pub type Petagrams = Quantity<$crate::mass::Petagram, $scalar>;
        /// Exagrams.
        pub type Exagrams = Quantity<$crate::mass::Exagram, $scalar>;
        /// Zettagrams.
        pub type Zettagrams = Quantity<$crate::mass::Zettagram, $scalar>;
        /// Yottagrams.
        pub type Yottagrams = Quantity<$crate::mass::Yottagram, $scalar>;
        /// Tonnes (metric tons).
        pub type Tonnes = Quantity<$crate::mass::Tonne, $scalar>;
        /// Carats.
        pub type Carats = Quantity<$crate::mass::Carat, $scalar>;
        /// Grains.
        pub type Grains = Quantity<$crate::mass::Grain, $scalar>;
        /// Pounds.
        pub type Pounds = Quantity<$crate::mass::Pound, $scalar>;
        /// Ounces.
        pub type Ounces = Quantity<$crate::mass::Ounce, $scalar>;
        /// Stones.
        pub type Stones = Quantity<$crate::mass::Stone, $scalar>;
        /// Short tons (US tons).
        pub type ShortTons = Quantity<$crate::mass::ShortTon, $scalar>;
        /// Long tons (Imperial tons).
        pub type LongTons = Quantity<$crate::mass::LongTon, $scalar>;
        /// Atomic mass units (daltons).
        pub type AtomicMassUnits = Quantity<$crate::mass::AtomicMassUnit, $scalar>;
        /// Solar masses.
        pub type SolarMasses = Quantity<$crate::mass::SolarMass, $scalar>;

        // ── Power ────────────────────────────────────────────────────────
        /// Watts.
        pub type Watts = Quantity<$crate::power::Watt, $scalar>;
        /// Yoctowatts.
        pub type Yoctowatts = Quantity<$crate::power::Yoctowatt, $scalar>;
        /// Zeptowatts.
        pub type Zeptowatts = Quantity<$crate::power::Zeptowatt, $scalar>;
        /// Attowatts.
        pub type Attowatts = Quantity<$crate::power::Attowatt, $scalar>;
        /// Femtowatts.
        pub type Femtowatts = Quantity<$crate::power::Femtowatt, $scalar>;
        /// Picowatts.
        pub type Picowatts = Quantity<$crate::power::Picowatt, $scalar>;
        /// Nanowatts.
        pub type Nanowatts = Quantity<$crate::power::Nanowatt, $scalar>;
        /// Microwatts.
        pub type Microwatts = Quantity<$crate::power::Microwatt, $scalar>;
        /// Milliwatts.
        pub type Milliwatts = Quantity<$crate::power::Milliwatt, $scalar>;
        /// Deciwatts.
        pub type Deciwatts = Quantity<$crate::power::Deciwatt, $scalar>;
        /// Decawatts.
        pub type Decawatts = Quantity<$crate::power::Decawatt, $scalar>;
        /// Hectowatts.
        pub type Hectowatts = Quantity<$crate::power::Hectowatt, $scalar>;
        /// Kilowatts.
        pub type Kilowatts = Quantity<$crate::power::Kilowatt, $scalar>;
        /// Megawatts.
        pub type Megawatts = Quantity<$crate::power::Megawatt, $scalar>;
        /// Gigawatts.
        pub type Gigawatts = Quantity<$crate::power::Gigawatt, $scalar>;
        /// Terawatts.
        pub type Terawatts = Quantity<$crate::power::Terawatt, $scalar>;
        /// Petawatts.
        pub type Petawatts = Quantity<$crate::power::Petawatt, $scalar>;
        /// Exawatts.
        pub type Exawatts = Quantity<$crate::power::Exawatt, $scalar>;
        /// Zettawatts.
        pub type Zettawatts = Quantity<$crate::power::Zettawatt, $scalar>;
        /// Yottawatts.
        pub type Yottawatts = Quantity<$crate::power::Yottawatt, $scalar>;
        /// Ergs per second.
        pub type ErgsPerSecond = Quantity<$crate::power::ErgPerSecond, $scalar>;
        /// Horsepower (metric).
        pub type HorsepowerMetrics = Quantity<$crate::power::HorsepowerMetric, $scalar>;
        /// Horsepower (electric).
        pub type HorsepowerElectrics = Quantity<$crate::power::HorsepowerElectric, $scalar>;
        /// Solar luminosities.
        pub type SolarLuminosities = Quantity<$crate::power::SolarLuminosity, $scalar>;

        // ── Area ─────────────────────────────────────────────────────────
        /// Square meters.
        pub type SquareMeters = Quantity<$crate::area::SquareMeter, $scalar>;
        /// Square kilometers.
        pub type SquareKilometers = Quantity<$crate::area::SquareKilometer, $scalar>;
        /// Square centimeters.
        pub type SquareCentimeters = Quantity<$crate::area::SquareCentimeter, $scalar>;
        /// Square millimeters.
        pub type SquareMillimeters = Quantity<$crate::area::SquareMillimeter, $scalar>;
        /// Hectares.
        pub type Hectares = Quantity<$crate::area::Hectare, $scalar>;
        /// Ares.
        pub type Ares = Quantity<$crate::area::Are, $scalar>;
        /// Square inches.
        pub type SquareInches = Quantity<$crate::area::SquareInch, $scalar>;
        /// Square feet.
        pub type SquareFeet = Quantity<$crate::area::SquareFoot, $scalar>;
        /// Square yards.
        pub type SquareYards = Quantity<$crate::area::SquareYard, $scalar>;
        /// Square miles.
        pub type SquareMiles = Quantity<$crate::area::SquareMile, $scalar>;
        /// Acres.
        pub type Acres = Quantity<$crate::area::Acre, $scalar>;

        // ── Volume ───────────────────────────────────────────────────────
        /// Cubic meters.
        pub type CubicMeters = Quantity<$crate::volume::CubicMeter, $scalar>;
        /// Cubic kilometers.
        pub type CubicKilometers = Quantity<$crate::volume::CubicKilometer, $scalar>;
        /// Cubic centimeters.
        pub type CubicCentimeters = Quantity<$crate::volume::CubicCentimeter, $scalar>;
        /// Cubic millimeters.
        pub type CubicMillimeters = Quantity<$crate::volume::CubicMillimeter, $scalar>;
        /// Liters.
        pub type Liters = Quantity<$crate::volume::Liter, $scalar>;
        /// Milliliters.
        pub type Milliliters = Quantity<$crate::volume::Milliliter, $scalar>;
        /// Microliters.
        pub type Microliters = Quantity<$crate::volume::Microliter, $scalar>;
        /// Centiliters.
        pub type Centiliters = Quantity<$crate::volume::Centiliter, $scalar>;
        /// Deciliters.
        pub type Deciliters = Quantity<$crate::volume::Deciliter, $scalar>;
        /// Cubic inches.
        pub type CubicInches = Quantity<$crate::volume::CubicInch, $scalar>;
        /// Cubic feet.
        pub type CubicFeet = Quantity<$crate::volume::CubicFoot, $scalar>;
        /// US gallons.
        pub type UsGallons = Quantity<$crate::volume::UsGallon, $scalar>;
        /// US fluid ounces.
        pub type UsFluidOunces = Quantity<$crate::volume::UsFluidOunce, $scalar>;
    };
}

pub(crate) use define_scalar_aliases;
