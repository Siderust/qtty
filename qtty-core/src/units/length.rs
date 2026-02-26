//! Length units.
//!
//! The canonical scaling unit for this dimension is [`Meter`] (`Meter::RATIO == 1.0`). All other
//! length units are expressed as exact or best‑available ratios to metres.
//!
//! This module provides:
//!
//! - **SI ladder**: the full metric prefix family for metres from yocto‑ to yotta‑.
//! - **Common defined units**: inch, foot, yard, (statute) mile, nautical mile, surveying units.
//! - **Astronomy**: astronomical unit (au), light‑year (ly), parsec (pc) and its multiples.
//! - **Geodesy and navigation**: Earth circumferences and related standards distances.
//! - **Fundamental physics lengths**: Bohr radius, Planck length, and related constants.
//! - **Nominal radii and distances**: available under the [`nominal`] submodule.
//!
//! Notes on definitions used here:
//!
//! - **Astronomical unit (au)** is **exactly** `149_597_870_700 m` (IAU 2012).
//! - **Parsec (pc)** is defined from au via `pc = au * 648000 / π` (exact, given au).
//! - **Light‑year (ly)** is derived from the exact speed of light `c = 299_792_458 m/s` and one
//!   **Julian year** (`365.25 d`, `d = 86400 s`).
//! - **Imperial and surveying units** follow the current international definitions (e.g. the
//!   international inch is exactly `0.0254 m`).
//! - **Nominal** astronomical/geodetic radii are grouped into [`nominal`] to avoid mixing them with
//!   strictly defined units.
//!
//! This module aims to avoid avoidable precision loss by preferring rational expressions and exact
//! relationships over rounded convenience factors wherever practical.
//!
//! ```rust
//! use qtty_core::length::{AstronomicalUnits, Kilometer};
//!
//! let au = AstronomicalUnits::new(1.0);
//! let km = au.to::<Kilometer>();
//! assert_eq!(km.value(), 149_597_870.7);
//! ```
//!
//! ## All length units
//!
//! ```rust
//! use qtty_core::length::*;
//! use qtty_core::length::nominal::*;
//!
//! macro_rules! touch {
//!     ($T:ty, $v:expr) => {{
//!         let q = <$T>::new($v);
//!         let _cloned = q;
//!         assert!(q == q);
//!     }};
//! }
//!
//! // SI sub-meter
//! touch!(Meters, 1.0); touch!(Decimeters, 1.0); touch!(Centimeters, 1.0);
//! touch!(Millimeters, 1.0); touch!(Micrometers, 1.0); touch!(Nanometers, 1.0);
//! touch!(Picometers, 1.0); touch!(Femtometers, 1.0); touch!(Attometers, 1.0);
//! touch!(Zeptometers, 1.0); touch!(Yoctometers, 1.0);
//! // SI super-meter
//! touch!(Decameters, 1.0); touch!(Hectometers, 1.0); touch!(Kilometers, 1.0);
//! touch!(Megameters, 1.0); touch!(Gigameters, 1.0); touch!(Terameters, 1.0);
//! touch!(Petameters, 1.0); touch!(Exameters, 1.0); touch!(Zettameters, 1.0);
//! touch!(Yottameters, 1.0);
//! // Astronomical
//! touch!(AstronomicalUnits, 1.0); touch!(LightYears, 1.0); touch!(Parsecs, 1.0);
//! touch!(Kiloparsecs, 1.0); touch!(Megaparsecs, 1.0); touch!(Gigaparsecs, 1.0);
//! // Imperial
//! touch!(Inches, 1.0); touch!(Feet, 1.0); touch!(Yards, 1.0);
//! touch!(Miles, 1.0); touch!(NauticalMiles, 1.0); touch!(Chains, 1.0);
//! touch!(Rods, 1.0); touch!(Links, 1.0); touch!(Fathoms, 1.0);
//! // Geodesy
//! touch!(EarthMeridionalCircumferences, 1.0);
//! touch!(EarthEquatorialCircumferences, 1.0);
//! // Physics
//! touch!(BohrRadii, 1.0); touch!(ClassicalElectronRadii, 1.0);
//! touch!(PlanckLengths, 1.0); touch!(ElectronReducedComptonWavelengths, 1.0);
//! // Nominal
//! touch!(SolarRadiuses, 1.0); touch!(EarthRadii, 1.0);
//! touch!(EarthEquatorialRadii, 1.0); touch!(EarthPolarRadii, 1.0);
//! touch!(LunarRadii, 1.0); touch!(JupiterRadii, 1.0);
//! touch!(LunarDistances, 1.0); touch!(SolarDiameters, 1.0);
//! ```

use crate::{Quantity, Unit};
use core::f64::consts::PI;
use qtty_derive::Unit;

/// Re-export from the dimension module.
pub use crate::dimension::Length;

/// Marker trait for any [`Unit`] whose dimension is [`Length`].
pub trait LengthUnit: Unit<Dim = Length> {}
impl<T: Unit<Dim = Length>> LengthUnit for T {}

// ─────────────────────────────────────────────────────────────────────────────
// SI base unit and core helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Metre (SI base unit).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "m", dimension = Length, ratio = 1.0)]
pub struct Meter;
/// A quantity measured in metres.
pub type Meters = Quantity<Meter>;
/// One metre.
pub const M: Meters = Meters::new(1.0);

/// Kilometre (`1000 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "km", dimension = Length, ratio = 1_000.0)]
pub struct Kilometer;
/// Type alias shorthand for [`Kilometer`].
pub type Km = Kilometer;
/// A quantity measured in kilometres.
pub type Kilometers = Quantity<Km>;
/// One kilometre.
pub const KM: Kilometers = Kilometers::new(1.0);

/// Centimetre (`1e-2 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "cm", dimension = Length, ratio = 1e-2)]
pub struct Centimeter;
/// Type alias shorthand for [`Centimeter`].
pub type Cm = Centimeter;
/// A quantity measured in centimetres.
pub type Centimeters = Quantity<Cm>;
/// One centimetre.
pub const CM: Centimeters = Centimeters::new(1.0);

/// Millimetre (`1e-3 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mm", dimension = Length, ratio = 1e-3)]
pub struct Millimeter;
/// Type alias shorthand for [`Millimeter`].
pub type Mm = Millimeter;
/// A quantity measured in millimetres.
pub type Millimeters = Quantity<Mm>;
/// One millimetre.
pub const MM: Millimeters = Millimeters::new(1.0);

/// Micrometre (`1e-6 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "μm", dimension = Length, ratio = 1e-6)]
pub struct Micrometer;
/// Type alias shorthand for [`Micrometer`].
pub type Um = Micrometer;
/// A quantity measured in micrometres.
pub type Micrometers = Quantity<Um>;
/// One micrometre.
pub const UM: Micrometers = Micrometers::new(1.0);

/// Nanometre (`1e-9 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "nm", dimension = Length, ratio = 1e-9)]
pub struct Nanometer;
/// Type alias shorthand for [`Nanometer`].
pub type Nm = Nanometer;
/// A quantity measured in nanometres.
pub type Nanometers = Quantity<Nm>;
/// One nanometre.
pub const NM: Nanometers = Nanometers::new(1.0);

/// Picometre (`1e-12 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "pm", dimension = Length, ratio = 1e-12)]
pub struct Picometer;
/// A quantity measured in picometres.
pub type Picometers = Quantity<Picometer>;
/// One picometre.
pub const PMETER: Picometers = Picometers::new(1.0);

/// Femtometre (`1e-15 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "fm", dimension = Length, ratio = 1e-15)]
pub struct Femtometer;
/// A quantity measured in femtometres.
pub type Femtometers = Quantity<Femtometer>;
/// One femtometre.
pub const FM: Femtometers = Femtometers::new(1.0);

/// Attometre (`1e-18 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "am", dimension = Length, ratio = 1e-18)]
pub struct Attometer;
/// A quantity measured in attometres.
pub type Attometers = Quantity<Attometer>;
/// One attometre.
pub const AM: Attometers = Attometers::new(1.0);

/// Zeptometre (`1e-21 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "zm", dimension = Length, ratio = 1e-21)]
pub struct Zeptometer;
/// A quantity measured in zeptometres.
pub type Zeptometers = Quantity<Zeptometer>;
/// One zeptometre.
pub const ZMETER: Zeptometers = Zeptometers::new(1.0);

/// Yoctometre (`1e-24 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ym", dimension = Length, ratio = 1e-24)]
pub struct Yoctometer;
/// A quantity measured in yoctometres.
pub type Yoctometers = Quantity<Yoctometer>;
/// One yoctometre.
pub const YMETER: Yoctometers = Yoctometers::new(1.0);

/// Megametre (`1e6 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Mm", dimension = Length, ratio = 1e6)]
pub struct Megameter;
/// Type alias shorthand for [`Megameter`].
pub type MegaMeter = Megameter;
/// A quantity measured in megametres.
pub type Megameters = Quantity<Megameter>;
/// One megametre.
pub const MEGAMETER: Megameters = Megameters::new(1.0);

/// Decimetre (`1e-1 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "dm", dimension = Length, ratio = 1e-1)]
pub struct Decimeter;
/// A quantity measured in decimetres.
pub type Decimeters = Quantity<Decimeter>;
/// One decimetre.
pub const DM: Decimeters = Decimeters::new(1.0);

/// Decametre (`1e1 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "dam", dimension = Length, ratio = 1e1)]
pub struct Decameter;
/// A quantity measured in decametres.
pub type Decameters = Quantity<Decameter>;
/// One decametre.
pub const DAM: Decameters = Decameters::new(1.0);

/// Hectometre (`1e2 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "hm", dimension = Length, ratio = 1e2)]
pub struct Hectometer;
/// A quantity measured in hectometres.
pub type Hectometers = Quantity<Hectometer>;
/// One hectometre.
pub const HM: Hectometers = Hectometers::new(1.0);

/// Gigametre (`1e9 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Gm", dimension = Length, ratio = 1e9)]
pub struct Gigameter;
/// A quantity measured in gigametres.
pub type Gigameters = Quantity<Gigameter>;
/// One gigametre.
pub const GM: Gigameters = Gigameters::new(1.0);

/// Terametre (`1e12 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Tm", dimension = Length, ratio = 1e12)]
pub struct Terameter;
/// A quantity measured in terametres.
pub type Terameters = Quantity<Terameter>;
/// One terametre.
pub const TM: Terameters = Terameters::new(1.0);

/// Petametre (`1e15 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Pm", dimension = Length, ratio = 1e15)]
pub struct Petameter;
/// A quantity measured in petametres.
pub type Petameters = Quantity<Petameter>;
/// One petametre.
pub const PM: Petameters = Petameters::new(1.0);

/// Exametre (`1e18 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Em", dimension = Length, ratio = 1e18)]
pub struct Exameter;
/// A quantity measured in exametres.
pub type Exameters = Quantity<Exameter>;
/// One exametre.
pub const EM: Exameters = Exameters::new(1.0);

/// Zettametre (`1e21 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Zm", dimension = Length, ratio = 1e21)]
pub struct Zettameter;
/// A quantity measured in zettametres.
pub type Zettameters = Quantity<Zettameter>;
/// One zettametre.
pub const ZM: Zettameters = Zettameters::new(1.0);

/// Yottametre (`1e24 m`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Ym", dimension = Length, ratio = 1e24)]
pub struct Yottameter;
/// A quantity measured in yottametres.
pub type Yottameters = Quantity<Yottameter>;
/// One yottametre.
pub const YM: Yottameters = Yottameters::new(1.0);

// ─────────────────────────────────────────────────────────────────────────────
// Astronomical distance units
// ─────────────────────────────────────────────────────────────────────────────

/// Astronomical unit (au). Exact (IAU 2012): metres per au.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "au", dimension = Length, ratio = 149_597_870_700.0)]
pub struct AstronomicalUnit;
/// Type alias shorthand for [`AstronomicalUnit`].
pub type Au = AstronomicalUnit;
/// A quantity measured in astronomical units.
pub type AstronomicalUnits = Quantity<Au>;
/// One astronomical unit.
pub const AU: AstronomicalUnits = AstronomicalUnits::new(1.0);

// Exact speed of light and Julian year, used to derive the light‑year ratio.
const SPEED_OF_LIGHT_M_PER_S: f64 = 299_792_458.0;
const SECONDS_PER_DAY: f64 = 86_400.0;
const DAYS_PER_JULIAN_YEAR: f64 = 36525.0 / 100.0; // 365.25 d
const SECONDS_PER_JULIAN_YEAR: f64 = SECONDS_PER_DAY * DAYS_PER_JULIAN_YEAR;
const METERS_PER_LIGHT_YEAR: f64 = SPEED_OF_LIGHT_M_PER_S * SECONDS_PER_JULIAN_YEAR;

/// Light-year (ly): distance light travels in one Julian year (`365.25 d`) at `c = 299_792_458 m/s`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ly", dimension = Length, ratio = METERS_PER_LIGHT_YEAR)]
pub struct LightYear;
/// Type alias shorthand for [`LightYear`].
pub type Ly = LightYear;
/// A quantity measured in light-years.
pub type LightYears = Quantity<Ly>;
/// One light-year.
pub const LY: LightYears = LightYears::new(1.0);

/// Parsec (pc): `pc = au * 648000 / π` (exact given au).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "pc", dimension = Length, ratio = 149_597_870_700.0 * (648_000.0 / PI))]
pub struct Parsec;
/// Type alias shorthand for [`Parsec`].
pub type Pc = Parsec;
/// A quantity measured in parsecs.
pub type Parsecs = Quantity<Pc>;
/// One parsec.
pub const PC: Parsecs = Parsecs::new(1.0);

/// Kiloparsec (kpc): `1e3 pc`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "kpc", dimension = Length, ratio = 1_000.0 * 149_597_870_700.0 * (648_000.0 / PI))]
pub struct Kiloparsec;
/// A quantity measured in kiloparsecs.
pub type Kiloparsecs = Quantity<Kiloparsec>;
/// One kiloparsec.
pub const KPC: Kiloparsecs = Kiloparsecs::new(1.0);

/// Megaparsec (Mpc): `1e6 pc`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Mpc", dimension = Length, ratio = 1_000_000.0 * 149_597_870_700.0 * (648_000.0 / PI))]
pub struct Megaparsec;
/// A quantity measured in megaparsecs.
pub type Megaparsecs = Quantity<Megaparsec>;
/// One megaparsec.
pub const MPC: Megaparsecs = Megaparsecs::new(1.0);

/// Gigaparsec (Gpc): `1e9 pc`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Gpc", dimension = Length, ratio = 1_000_000_000.0 * 149_597_870_700.0 * (648_000.0 / PI))]
pub struct Gigaparsec;
/// A quantity measured in gigaparsecs.
pub type Gigaparsecs = Quantity<Gigaparsec>;
/// One gigaparsec.
pub const GPC: Gigaparsecs = Gigaparsecs::new(1.0);

// ─────────────────────────────────────────────────────────────────────────────
// Imperial, US customary, and surveying units
// ─────────────────────────────────────────────────────────────────────────────

/// Inch (`0.0254 m` exactly).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "in", dimension = Length, ratio = 254.0 / 10_000.0)]
pub struct Inch;
/// A quantity measured in inches.
pub type Inches = Quantity<Inch>;
/// One inch.
pub const INCH: Inches = Inches::new(1.0);

/// Foot (`0.3048 m` exactly).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ft", dimension = Length, ratio = 3048.0 / 10_000.0)]
pub struct Foot;
/// A quantity measured in feet.
pub type Feet = Quantity<Foot>;
/// One foot.
pub const FT: Feet = Feet::new(1.0);

/// Yard (`0.9144 m` exactly).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "yd", dimension = Length, ratio = 9144.0 / 10_000.0)]
pub struct Yard;
/// A quantity measured in yards.
pub type Yards = Quantity<Yard>;
/// One yard.
pub const YD: Yards = Yards::new(1.0);

/// (Statute) mile (`1609.344 m` exactly).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mi", dimension = Length, ratio = 1_609_344.0 / 1_000.0)]
pub struct Mile;
/// A quantity measured in miles.
pub type Miles = Quantity<Mile>;
/// One mile.
pub const MI: Miles = Miles::new(1.0);

/// Nautical mile (`1852 m` exactly).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "nmi", dimension = Length, ratio = 1_852.0)]
pub struct NauticalMile;
/// A quantity measured in nautical miles.
pub type NauticalMiles = Quantity<NauticalMile>;
/// One nautical mile.
pub const NMI: NauticalMiles = NauticalMiles::new(1.0);

/// Chain (`66 ft` exactly).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ch", dimension = Length, ratio = 66.0 * (3048.0 / 10_000.0))]
pub struct Chain;
/// A quantity measured in chains.
pub type Chains = Quantity<Chain>;
/// One chain.
pub const CHAIN: Chains = Chains::new(1.0);

/// Rod / pole / perch (`16.5 ft` exactly).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "rd", dimension = Length, ratio = 16.5 * (3048.0 / 10_000.0))]
pub struct Rod;
/// A quantity measured in rods/poles/perches.
pub type Rods = Quantity<Rod>;
/// One rod.
pub const ROD: Rods = Rods::new(1.0);

/// Link (`1/100 of a chain`, i.e. `0.66 ft`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "lk", dimension = Length, ratio = (66.0 / 100.0) * (3048.0 / 10_000.0))]
pub struct Link;
/// A quantity measured in links.
pub type Links = Quantity<Link>;
/// One link.
pub const LINK: Links = Links::new(1.0);

/// Fathom (`6 ft` exactly).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ftm", dimension = Length, ratio = 6.0 * (3048.0 / 10_000.0))]
pub struct Fathom;
/// A quantity measured in fathoms.
pub type Fathoms = Quantity<Fathom>;
/// One fathom.
pub const FTM: Fathoms = Fathoms::new(1.0);

// ─────────────────────────────────────────────────────────────────────────────
// Geodesy and navigation
// ─────────────────────────────────────────────────────────────────────────────

/// Earth meridional circumference (approximate mean value).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Cmer", dimension = Length, ratio = 40_007_863.0)]
pub struct EarthMeridionalCircumference;
/// A quantity measured in Earth meridional circumferences.
pub type EarthMeridionalCircumferences = Quantity<EarthMeridionalCircumference>;
/// One Earth meridional circumference.
pub const C_MERIDIONAL: EarthMeridionalCircumferences = EarthMeridionalCircumferences::new(1.0);

/// Earth equatorial circumference.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Ceq", dimension = Length, ratio = 40_075_017.0)]
pub struct EarthEquatorialCircumference;
/// A quantity measured in Earth equatorial circumferences.
pub type EarthEquatorialCircumferences = Quantity<EarthEquatorialCircumference>;
/// One Earth equatorial circumference.
pub const C_EQUATORIAL: EarthEquatorialCircumferences = EarthEquatorialCircumferences::new(1.0);

// ─────────────────────────────────────────────────────────────────────────────
// Fundamental physics lengths (CODATA values)
// ─────────────────────────────────────────────────────────────────────────────

/// Bohr radius (`a0`). CODATA 2018 value in metres.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "a0", dimension = Length, ratio = 5.291_772_109_03e-11)]
pub struct BohrRadius;
/// A quantity measured in Bohr radii.
pub type BohrRadii = Quantity<BohrRadius>;
/// One Bohr radius.
pub const A0: BohrRadii = BohrRadii::new(1.0);

/// Classical electron radius (`re`). CODATA 2018 value in metres.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "re", dimension = Length, ratio = 2.817_940_326_2e-15)]
pub struct ClassicalElectronRadius;
/// A quantity measured in classical electron radii.
pub type ClassicalElectronRadii = Quantity<ClassicalElectronRadius>;
/// One classical electron radius.
pub const RE: ClassicalElectronRadii = ClassicalElectronRadii::new(1.0);

/// Planck length (`lp`). CODATA 2018 value in metres.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "lp", dimension = Length, ratio = 1.616_255e-35)]
pub struct PlanckLength;
/// A quantity measured in Planck lengths.
pub type PlanckLengths = Quantity<PlanckLength>;
/// One Planck length.
pub const LP: PlanckLengths = PlanckLengths::new(1.0);

/// Reduced Compton wavelength of the electron (`lambda_bar_e`). CODATA 2018 value in metres.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "lambda_bar_e", dimension = Length, ratio = 3.861_592_679_6e-13)]
pub struct ElectronReducedComptonWavelength;
/// A quantity measured in reduced Compton wavelengths of the electron.
pub type ElectronReducedComptonWavelengths = Quantity<ElectronReducedComptonWavelength>;
/// One reduced Compton wavelength of the electron.
pub const LAMBDA_BAR_E: ElectronReducedComptonWavelengths =
    ElectronReducedComptonWavelengths::new(1.0);

// ─────────────────────────────────────────────────────────────────────────────
// Nominal radii and distances
// ─────────────────────────────────────────────────────────────────────────────

/// Nominal astronomical and planetary radii and related distances.
///
/// Values in this module are **nominal** (conventionally rounded) and are kept separate from the
/// main length namespace to avoid confusion with strictly defined units.
pub mod nominal {
    use super::*;

    /// Solar radius (R☉). Nominal value: metres per R☉.
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
    #[unit(symbol = "Rsun", dimension = Length, ratio = 695_700_000.0)]
    pub struct SolarRadius;
    /// A quantity measured in solar radii.
    pub type SolarRadiuses = Quantity<SolarRadius>;
    /// One solar radius.
    pub const RSUN: SolarRadiuses = SolarRadiuses::new(1.0);

    /// Earth mean radius (nominal).
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
    #[unit(symbol = "Rearth", dimension = Length, ratio = 6_371_000.0)]
    pub struct EarthRadius;
    /// A quantity measured in Earth radii.
    pub type EarthRadii = Quantity<EarthRadius>;
    /// One Earth radius (mean).
    pub const R_EARTH: EarthRadii = EarthRadii::new(1.0);

    /// Earth equatorial radius (WGS84).
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
    #[unit(symbol = "Rearth_eq", dimension = Length, ratio = 6_378_137.0)]
    pub struct EarthEquatorialRadius;
    /// A quantity measured in Earth equatorial radii.
    pub type EarthEquatorialRadii = Quantity<EarthEquatorialRadius>;
    /// One Earth equatorial radius.
    pub const R_EARTH_EQ: EarthEquatorialRadii = EarthEquatorialRadii::new(1.0);

    /// Earth polar radius.
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
    #[unit(symbol = "Rearth_p", dimension = Length, ratio = 6_356_752.314_2)]
    pub struct EarthPolarRadius;
    /// A quantity measured in Earth polar radii.
    pub type EarthPolarRadii = Quantity<EarthPolarRadius>;
    /// One Earth polar radius.
    pub const R_EARTH_P: EarthPolarRadii = EarthPolarRadii::new(1.0);

    /// Lunar radius (mean, nominal).
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
    #[unit(symbol = "Rmoon", dimension = Length, ratio = 1_737_400.0)]
    pub struct LunarRadius;
    /// A quantity measured in lunar radii.
    pub type LunarRadii = Quantity<LunarRadius>;
    /// One lunar radius.
    pub const R_MOON: LunarRadii = LunarRadii::new(1.0);

    /// Jupiter equatorial radius (nominal).
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
    #[unit(symbol = "Rjup", dimension = Length, ratio = 71_492_000.0)]
    pub struct JupiterRadius;
    /// A quantity measured in Jupiter radii.
    pub type JupiterRadii = Quantity<JupiterRadius>;
    /// One Jupiter radius.
    pub const R_JUPITER: JupiterRadii = JupiterRadii::new(1.0);

    /// Lunar distance (Earth–Moon mean distance, LD).
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
    #[unit(symbol = "LD", dimension = Length, ratio = 384_400_000.0)]
    pub struct LunarDistance;
    /// A quantity measured in lunar distances.
    pub type LunarDistances = Quantity<LunarDistance>;
    /// One lunar distance.
    pub const LD: LunarDistances = LunarDistances::new(1.0);

    /// Solar diameter (twice the solar radius).
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
    #[unit(symbol = "Dsun", dimension = Length, ratio = 2.0 * 695_700_000.0)]
    pub struct SolarDiameter;
    /// A quantity measured in solar diameters.
    pub type SolarDiameters = Quantity<SolarDiameter>;
    /// One solar diameter.
    pub const D_SUN: SolarDiameters = SolarDiameters::new(1.0);

    // Allow convenient conversions between selected nominal units and core
    // length units (e.g., SolarRadius <-> Kilometer) without polluting the
    // main length namespace with nominal types.
    crate::impl_unit_from_conversions!(SolarRadius, Kilometer);
    #[cfg(feature = "cross-unit-ops")]
    crate::impl_unit_cross_unit_ops!(SolarRadius, Kilometer);
}

// Generate all bidirectional From implementations between length units.
//
// This single invocation ensures that any quantity measured in one length unit can be
// converted into any other via `From`/`Into`, mirroring the previous behavior while
// including the extended unit set.
crate::impl_unit_from_conversions!(
    Meter,
    Decimeter,
    Centimeter,
    Millimeter,
    Micrometer,
    Nanometer,
    Picometer,
    Femtometer,
    Attometer,
    Zeptometer,
    Yoctometer,
    Decameter,
    Hectometer,
    Kilometer,
    Megameter,
    Gigameter,
    Terameter,
    Petameter,
    Exameter,
    Zettameter,
    Yottameter,
    AstronomicalUnit,
    LightYear,
    Parsec,
    Kiloparsec,
    Megaparsec,
    Gigaparsec,
    Inch,
    Foot,
    Yard,
    Mile,
    NauticalMile,
    Chain,
    Rod,
    Link,
    Fathom,
    EarthMeridionalCircumference,
    EarthEquatorialCircumference,
    BohrRadius,
    ClassicalElectronRadius,
    PlanckLength,
    ElectronReducedComptonWavelength
);

// Optional cross-unit operator support (`==`, `<`, etc.).
#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops!(
    Meter,
    Decimeter,
    Centimeter,
    Millimeter,
    Micrometer,
    Nanometer,
    Picometer,
    Femtometer,
    Attometer,
    Zeptometer,
    Yoctometer,
    Decameter,
    Hectometer,
    Kilometer,
    Megameter,
    Gigameter,
    Terameter,
    Petameter,
    Exameter,
    Zettameter,
    Yottameter,
    AstronomicalUnit,
    LightYear,
    Parsec,
    Kiloparsec,
    Megaparsec,
    Gigaparsec,
    Inch,
    Foot,
    Yard,
    Mile,
    NauticalMile,
    Chain,
    Rod,
    Link,
    Fathom,
    EarthMeridionalCircumference,
    EarthEquatorialCircumference,
    BohrRadius,
    ClassicalElectronRadius,
    PlanckLength,
    ElectronReducedComptonWavelength
);

#[cfg(test)]
mod tests {
    use super::nominal::SolarRadiuses;
    use super::*;
    use approx::{assert_abs_diff_eq, assert_relative_eq};
    use proptest::prelude::*;

    // ─────────────────────────────────────────────────────────────────────────────
    // Basic conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn kilometer_to_meter() {
        let km = Kilometers::new(1.0);
        let m = km.to::<Meter>();
        assert_abs_diff_eq!(m.value(), 1000.0, epsilon = 1e-9);
    }

    #[test]
    fn meter_to_kilometer() {
        let m = Meters::new(1000.0);
        let km = m.to::<Kilometer>();
        assert_abs_diff_eq!(km.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn au_to_meters() {
        let au = AstronomicalUnits::new(1.0);
        let m = au.to::<Meter>();
        // 1 AU = 149,597,870,700 meters (exact, IAU 2012).
        assert_abs_diff_eq!(m.value(), 149_597_870_700.0, epsilon = 1e-6);
    }

    #[test]
    fn au_to_kilometers() {
        let au = AstronomicalUnits::new(1.0);
        let km = au.to::<Kilometer>();
        // 1 AU = 149,597,870,700 m => 149,597,870.7 km.
        assert_relative_eq!(km.value(), 149_597_870.7, max_relative = 1e-12);
    }

    #[test]
    fn light_year_to_meters() {
        let ly = LightYears::new(1.0);
        let m = ly.to::<Meter>();
        // 1 LY = c * 365.25 d, where d = 86400 s
        assert_relative_eq!(m.value(), METERS_PER_LIGHT_YEAR, max_relative = 1e-12);
    }

    #[test]
    fn light_year_to_kilometers() {
        let ly = LightYears::new(1.0);
        let km = ly.to::<Kilometer>();
        // 1 LY ≈ 9.461e12 km
        assert_relative_eq!(km.value(), 9_460_730_472_580.000_8, max_relative = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // AU <-> LY conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn au_to_light_year() {
        let au = AstronomicalUnits::new(1.0);
        let ly = au.to::<LightYear>();
        // 1 AU ≈ 1.582e-5 LY
        assert_relative_eq!(ly.value(), 1.582e-5, max_relative = 1e-3);
    }

    #[test]
    fn light_year_to_au() {
        let ly = LightYears::new(1.0);
        let au = ly.to::<AstronomicalUnit>();
        // 1 LY ≈ 63,241 AU
        assert_relative_eq!(au.value(), 63241.0, max_relative = 1e-3);
    }

    #[test]
    fn from_impl_au_to_ly() {
        let au = 1.0 * AU;
        let ly: LightYears = au.into();
        assert_relative_eq!(ly.value(), 1.582e-5, max_relative = 1e-3);
    }

    #[test]
    fn from_impl_ly_to_au() {
        let ly = 1.0 * LY;
        let au: AstronomicalUnits = ly.into();
        assert_relative_eq!(au.value(), 63241.0, max_relative = 1e-3);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Parsec conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn parsec_to_light_year() {
        let pc = Parsecs::new(1.0);
        let ly = pc.to::<LightYear>();
        // 1 pc expressed in light-years, using the exact AU-based definition.
        let expected = (AstronomicalUnit::RATIO * (648_000.0 / PI)) / LightYear::RATIO;
        assert_relative_eq!(ly.value(), expected, max_relative = 1e-15);
    }

    #[test]
    fn parsec_to_au() {
        let pc = Parsecs::new(1.0);
        let au = pc.to::<AstronomicalUnit>();
        // 1 pc ≈ 206,265 AU (using exact definition: 1 pc = 3.26 LY, 1 LY ≈ 63241 AU)
        // So 1 pc ≈ 3.26 * 63241 ≈ 206,165 AU
        assert_relative_eq!(au.value(), 3.26 * 63241.0, max_relative = 1e-2);
    }

    #[test]
    fn parsec_ratio_sanity() {
        // Parsec is defined from AU: pc = au * 648000 / π
        let lhs = Parsec::RATIO / AstronomicalUnit::RATIO;
        let rhs = 648_000.0 / PI;
        assert_relative_eq!(lhs, rhs, max_relative = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Solar radius
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn solar_radius_to_meters() {
        let sr = SolarRadiuses::new(1.0);
        let m = sr.to::<Meter>();
        // 1 R☉ = 695,700 km = 695,700,000 m
        assert_abs_diff_eq!(m.value(), 695_700_000.0, epsilon = 1e-3);
    }

    #[test]
    fn solar_radius_to_km() {
        let sr = SolarRadiuses::new(1.0);
        let km = sr.to::<Kilometer>();
        // 1 R☉ = 695,700 km
        assert_abs_diff_eq!(km.value(), 695_700.0, epsilon = 1e-6);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Roundtrip conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn roundtrip_km_m() {
        let original = Kilometers::new(42.5);
        let converted = original.to::<Meter>();
        let back = converted.to::<Kilometer>();
        assert_abs_diff_eq!(back.value(), original.value(), epsilon = 1e-12);
    }

    #[test]
    fn roundtrip_au_ly() {
        let original = AstronomicalUnits::new(10000.0);
        let converted = original.to::<LightYear>();
        let back = converted.to::<AstronomicalUnit>();
        assert_relative_eq!(back.value(), original.value(), max_relative = 1e-12);
    }

    #[test]
    fn roundtrip_parsec_ly() {
        let original = Parsecs::new(5.0);
        let converted = original.to::<LightYear>();
        let back = converted.to::<Parsec>();
        assert_relative_eq!(back.value(), original.value(), max_relative = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Exact relationship tests for new units
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn inch_to_meter_exact_ratio() {
        let inch = Inches::new(1.0);
        let m = inch.to::<Meter>();
        // International inch: exactly 0.0254 m
        assert_relative_eq!(m.value(), 0.0254, max_relative = 1e-16);
    }

    #[test]
    fn nautical_mile_to_meter_exact_ratio() {
        let nmi = NauticalMiles::new(1.0);
        let m = nmi.to::<Meter>();
        // International nautical mile: exactly 1852 m
        assert_abs_diff_eq!(m.value(), 1852.0, epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Roundtrip sanity for representative units
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn roundtrip_inch_meter() {
        let original = Inches::new(123.456);
        let converted = original.to::<Meter>();
        let back = converted.to::<Inch>();
        assert_relative_eq!(back.value(), original.value(), max_relative = 1e-12);
    }

    #[test]
    fn roundtrip_nautical_mile_meter() {
        let original = NauticalMiles::new(3.75);
        let converted = original.to::<Meter>();
        let back = converted.to::<NauticalMile>();
        assert_relative_eq!(back.value(), original.value(), max_relative = 1e-12);
    }

    #[test]
    fn roundtrip_parsec_kpc() {
        let original = Parsecs::new(12_345.0);
        let converted = original.to::<Kiloparsec>();
        let back = converted.to::<Parsec>();
        assert_relative_eq!(back.value(), original.value(), max_relative = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_roundtrip_km_m(k in -1e6..1e6f64) {
            let original = Kilometers::new(k);
            let converted = original.to::<Meter>();
            let back = converted.to::<Kilometer>();
            prop_assert!((back.value() - original.value()).abs() < 1e-9 * k.abs().max(1.0));
        }

        #[test]
        fn prop_km_m_ratio(k in 1e-6..1e6f64) {
            let km = Kilometers::new(k);
            let m = km.to::<Meter>();
            // 1 km = 1000 m
            prop_assert!((m.value() / km.value() - 1000.0).abs() < 1e-9);
        }

        #[test]
        fn prop_roundtrip_au_km(a in 1e-6..1e6f64) {
            let original = AstronomicalUnits::new(a);
            let converted = original.to::<Kilometer>();
            let back = converted.to::<AstronomicalUnit>();
            prop_assert!((back.value() - original.value()).abs() / original.value() < 1e-12);
        }

        #[test]
        fn prop_roundtrip_inch_m(i in -1e6..1e6f64) {
            let original = Inches::new(i);
            let converted = original.to::<Meter>();
            let back = converted.to::<Inch>();
            let scale = i.abs().max(1.0);
            prop_assert!((back.value() - original.value()).abs() < 1e-9 * scale);
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // SI sub-meter ladder
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn decimeter_to_meter() {
        let q = Decimeters::new(10.0);
        assert_relative_eq!(q.to::<Meter>().value(), 1.0, max_relative = 1e-15);
    }

    #[test]
    fn centimeter_to_meter() {
        let q = Centimeters::new(100.0);
        assert_relative_eq!(q.to::<Meter>().value(), 1.0, max_relative = 1e-15);
    }

    #[test]
    fn millimeter_to_centimeter() {
        let q = Millimeters::new(10.0);
        assert_relative_eq!(q.to::<Centimeter>().value(), 1.0, max_relative = 1e-15);
    }

    #[test]
    fn micrometer_to_millimeter() {
        let q = Micrometers::new(1_000.0);
        assert_relative_eq!(q.to::<Millimeter>().value(), 1.0, max_relative = 1e-15);
    }

    #[test]
    fn nanometer_to_micrometer() {
        let q = Nanometers::new(1_000.0);
        assert_relative_eq!(q.to::<Micrometer>().value(), 1.0, max_relative = 1e-15);
    }

    #[test]
    fn picometer_to_nanometer() {
        let q = Picometers::new(1_000.0);
        assert_relative_eq!(q.to::<Nanometer>().value(), 1.0, max_relative = 1e-15);
    }

    #[test]
    fn femtometer_to_picometer() {
        let q = Femtometers::new(1_000.0);
        assert_relative_eq!(q.to::<Picometer>().value(), 1.0, max_relative = 1e-15);
    }

    #[test]
    fn attometer_to_femtometer() {
        let q = Attometers::new(1_000.0);
        assert_relative_eq!(q.to::<Femtometer>().value(), 1.0, max_relative = 1e-15);
    }

    #[test]
    fn zeptometer_to_attometer() {
        let q = Zeptometers::new(1_000.0);
        assert_relative_eq!(q.to::<Attometer>().value(), 1.0, max_relative = 1e-15);
    }

    #[test]
    fn yoctometer_to_zeptometer() {
        let q = Yoctometers::new(1_000.0);
        assert_relative_eq!(q.to::<Zeptometer>().value(), 1.0, max_relative = 1e-15);
    }

    // ─────────────────────────────────────────────────────────────────────────
    // SI super-meter ladder
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn decameter_to_meter() {
        let q = Decameters::new(1.0);
        assert_relative_eq!(q.to::<Meter>().value(), 10.0, max_relative = 1e-15);
    }

    #[test]
    fn hectometer_to_meter() {
        let q = Hectometers::new(1.0);
        assert_relative_eq!(q.to::<Meter>().value(), 100.0, max_relative = 1e-15);
    }

    #[test]
    fn megameter_to_kilometer() {
        let q = Megameters::new(1.0);
        assert_relative_eq!(q.to::<Kilometer>().value(), 1_000.0, max_relative = 1e-15);
    }

    #[test]
    fn gigameter_to_megameter() {
        let q = Gigameters::new(1.0);
        assert_relative_eq!(q.to::<Megameter>().value(), 1_000.0, max_relative = 1e-15);
    }

    #[test]
    fn terameter_to_gigameter() {
        let q = Terameters::new(1.0);
        assert_relative_eq!(q.to::<Gigameter>().value(), 1_000.0, max_relative = 1e-15);
    }

    #[test]
    fn petameter_to_terameter() {
        let q = Petameters::new(1.0);
        assert_relative_eq!(q.to::<Terameter>().value(), 1_000.0, max_relative = 1e-15);
    }

    #[test]
    fn exameter_to_petameter() {
        let q = Exameters::new(1.0);
        assert_relative_eq!(q.to::<Petameter>().value(), 1_000.0, max_relative = 1e-15);
    }

    #[test]
    fn zettameter_to_exameter() {
        let q = Zettameters::new(1.0);
        assert_relative_eq!(q.to::<Exameter>().value(), 1_000.0, max_relative = 1e-15);
    }

    #[test]
    fn yottameter_to_zettameter() {
        let q = Yottameters::new(1.0);
        assert_relative_eq!(q.to::<Zettameter>().value(), 1_000.0, max_relative = 1e-15);
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Imperial / surveying units
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn foot_to_meter() {
        let q = Feet::new(1.0);
        // 1 ft = 0.3048 m exactly
        assert_relative_eq!(q.to::<Meter>().value(), 0.3048, max_relative = 1e-15);
    }

    #[test]
    fn yard_to_meter() {
        let q = Yards::new(1.0);
        // 1 yd = 0.9144 m exactly
        assert_relative_eq!(q.to::<Meter>().value(), 0.9144, max_relative = 1e-15);
    }

    #[test]
    fn mile_to_kilometer() {
        let q = Miles::new(1.0);
        // 1 mi = 1609.344 m exactly
        assert_relative_eq!(q.to::<Kilometer>().value(), 1.609_344, max_relative = 1e-15);
    }

    #[test]
    fn fathom_to_foot() {
        let q = Fathoms::new(1.0);
        // 1 fathom = 6 ft
        assert_relative_eq!(q.to::<Foot>().value(), 6.0, max_relative = 1e-14);
    }

    #[test]
    fn chain_to_foot() {
        let q = Chains::new(1.0);
        // 1 chain = 66 ft
        assert_relative_eq!(q.to::<Foot>().value(), 66.0, max_relative = 1e-14);
    }

    #[test]
    fn rod_to_foot() {
        let q = Rods::new(1.0);
        // 1 rod = 16.5 ft
        assert_relative_eq!(q.to::<Foot>().value(), 16.5, max_relative = 1e-14);
    }

    #[test]
    fn link_to_foot() {
        let q = Links::new(100.0);
        // 100 links = 1 chain = 66 ft
        assert_relative_eq!(q.to::<Foot>().value(), 66.0, max_relative = 1e-14);
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Larger astronomical parsec multiples
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn megaparsec_to_kiloparsec() {
        let q = Megaparsecs::new(1.0);
        assert_relative_eq!(q.to::<Kiloparsec>().value(), 1_000.0, max_relative = 1e-12);
    }

    #[test]
    fn gigaparsec_to_megaparsec() {
        let q = Gigaparsecs::new(1.0);
        assert_relative_eq!(q.to::<Megaparsec>().value(), 1_000.0, max_relative = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Geodesy
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn earth_meridional_circumference_to_km() {
        let q = EarthMeridionalCircumferences::new(1.0);
        // ≈ 40_007.863 km
        assert_relative_eq!(q.to::<Kilometer>().value(), 40_007.863, max_relative = 1e-6);
    }

    #[test]
    fn earth_equatorial_circumference_to_km() {
        let q = EarthEquatorialCircumferences::new(1.0);
        // ≈ 40_075.017 km
        assert_relative_eq!(q.to::<Kilometer>().value(), 40_075.017, max_relative = 1e-6);
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Physics lengths
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn bohr_radius_to_picometers() {
        let q = BohrRadii::new(1.0);
        // a0 ≈ 52.9177 pm
        assert_relative_eq!(q.to::<Picometer>().value(), 52.917_72, max_relative = 1e-5);
    }

    #[test]
    fn classical_electron_radius_to_femtometers() {
        let q = ClassicalElectronRadii::new(1.0);
        // re ≈ 2.81794 fm
        assert_relative_eq!(q.to::<Femtometer>().value(), 2.817_940_326_2, max_relative = 1e-9);
    }

    #[test]
    fn planck_length_ratio() {
        // Just check ratio round-trips without numeric overflow
        let q = PlanckLengths::new(1.0);
        let back = q.to::<Meter>().to::<PlanckLength>();
        assert_relative_eq!(back.value(), 1.0, max_relative = 1e-9);
    }

    #[test]
    fn electron_compton_wavelength_to_femtometers() {
        let q = ElectronReducedComptonWavelengths::new(1.0);
        // λ̄_e ≈ 386.159 fm
        assert_relative_eq!(q.to::<Femtometer>().value(), 386.159_267_96, max_relative = 1e-7);
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Nominal submodule
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn earth_radius_to_km() {
        let q = nominal::EarthRadii::new(1.0);
        assert_relative_eq!(q.to::<Kilometer>().value(), 6_371.0, max_relative = 1e-9);
    }

    #[test]
    fn earth_equatorial_radius_to_km() {
        let q = nominal::EarthEquatorialRadii::new(1.0);
        assert_relative_eq!(q.to::<Kilometer>().value(), 6_378.137, max_relative = 1e-9);
    }

    #[test]
    fn earth_polar_radius_to_km() {
        let q = nominal::EarthPolarRadii::new(1.0);
        assert_relative_eq!(q.to::<Kilometer>().value(), 6_356.752_314_2, max_relative = 1e-9);
    }

    #[test]
    fn lunar_radius_to_km() {
        let q = nominal::LunarRadii::new(1.0);
        assert_relative_eq!(q.to::<Kilometer>().value(), 1_737.4, max_relative = 1e-9);
    }

    #[test]
    fn jupiter_radius_to_km() {
        let q = nominal::JupiterRadii::new(1.0);
        assert_relative_eq!(q.to::<Kilometer>().value(), 71_492.0, max_relative = 1e-9);
    }

    #[test]
    fn lunar_distance_to_km() {
        let q = nominal::LunarDistances::new(1.0);
        assert_relative_eq!(q.to::<Kilometer>().value(), 384_400.0, max_relative = 1e-9);
    }

    #[test]
    fn solar_diameter_to_solar_radius() {
        let diameters = nominal::SolarDiameters::new(1.0);
        let radii = diameters.to::<nominal::SolarRadius>();
        assert_relative_eq!(radii.value(), 2.0, max_relative = 1e-14);
    }

    #[test]
    fn symbols_are_correct() {
        assert_eq!(Meter::SYMBOL, "m");
        assert_eq!(Kilometer::SYMBOL, "km");
        assert_eq!(Centimeter::SYMBOL, "cm");
        assert_eq!(Inch::SYMBOL, "in");
        assert_eq!(AstronomicalUnit::SYMBOL, "au");
        assert_eq!(Parsec::SYMBOL, "pc");
    }
}
