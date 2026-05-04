/**
 * @file qtty_ffi.h
 * @brief C-compatible FFI bindings for qtty physical quantities and unit conversions.
 *
 * This header provides the C API for the qtty-ffi library, enabling C/C++ code
 * to construct and convert physical quantities using qtty's conversion logic.
 *
 * # Example Usage
 *
 * @code{.c}
 * #include "qtty_ffi.h"
 * #include <stdio.h>
 *
 * int main() {
 *     qtty_quantity_t meters, kilometers;
 *     
 *     // Create a quantity: 1000 meters
 *     int32_t status = qtty_quantity_make(1000.0, UNIT_ID_METER, &meters);
 *     if (status != QTTY_OK) {
 *         fprintf(stderr, "Failed to create quantity\n");
 *         return 1;
 *     }
 *     
 *     // Convert to kilometers
 *     status = qtty_quantity_convert(meters, UNIT_ID_KILOMETER, &kilometers);
 *     if (status == QTTY_OK) {
 *         printf("1000 meters = %.2f kilometers\n", kilometers.value);
 *     }
 *     
 *     return 0;
 * }
 * @endcode
 *
 * # Thread Safety
 *
 * All functions are thread-safe. The library contains no global mutable state.
 *
 * # ABI Stability
 *
 * Enum discriminant values and struct layouts are part of the ABI contract
 * and will not change in backward-compatible releases.
 *
 * @version 1.0
 */


#ifndef QTTY_FFI_H
#define QTTY_FFI_H

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>



















































































































































































































































































































































































































































































































































































/*
 Format flag: default decimal notation (e.g. `"1234.57 m"`).
 */
#define QTTY_FMT_DEFAULT 0

/*
 Format flag: scientific notation with lowercase `e` (e.g. `"1.23e3 m"`).
 */
#define QTTY_FMT_LOWER_EXP 1

/*
 Format flag: scientific notation with uppercase `E` (e.g. `"1.23E3 m"`).
 */
#define QTTY_FMT_UPPER_EXP 2

/*
 Status codes returned by every qtty-ffi function.

 Callers must inspect this value before reading any output parameters.

 # ABI Contract

 Discriminant values are frozen; new variants may be added only at the end.

 */
enum QttyStatus
#ifdef __cplusplus
  : int32_t
#endif // __cplusplus
 {
  /*
   Operation completed successfully.
   */
  QTTY_STATUS_OK = 0,
  /*
   The provided unit ID is not recognized or valid.
   */
  QTTY_STATUS_UNKNOWN_UNIT = -1,
  /*
   Conversion requested between incompatible dimensions.
   */
  QTTY_STATUS_INCOMPATIBLE_DIM = -2,
  /*
   A required output pointer was null.
   */
  QTTY_STATUS_NULL_OUT = -3,
  /*
   The provided output buffer is too small.
   */
  QTTY_STATUS_BUFFER_TOO_SMALL = -4,
  /*
   A Rust panic was caught at the FFI boundary.

   This indicates a bug in the underlying library; the panic payload is
   discarded.  Domain errors (`UnknownUnit`, `IncompatibleDim`, etc.) are
   never reported via this variant.
   */
  QTTY_STATUS_INTERNAL_PANIC = -5,
};
#ifndef __cplusplus
typedef int32_t QttyStatus;
#endif // __cplusplus

/*
 Dimension identifier for FFI.

 Represents the physical dimension of a quantity. All discriminant values are
 explicitly assigned and are part of the ABI contract.

 # ABI Contract

 **Discriminant values must never change.** New dimensions may be added with
 new explicit discriminant values.
 */
enum DimensionId
#ifdef __cplusplus
  : uint32_t
#endif // __cplusplus
 {
  /*
   Length dimension (e.g., meters, kilometers).
   */
  DIMENSION_ID_LENGTH = 1,
  /*
   Time dimension (e.g., seconds, hours).
   */
  DIMENSION_ID_TIME = 2,
  /*
   Angle dimension (e.g., radians, degrees).
   */
  DIMENSION_ID_ANGLE = 3,
  /*
   Mass dimension (e.g., grams, kilograms).
   */
  DIMENSION_ID_MASS = 4,
  /*
   Power dimension (e.g., watts, kilowatts).
   */
  DIMENSION_ID_POWER = 5,
  /*
   Area dimension (e.g., square metres, hectares).
   */
  DIMENSION_ID_AREA = 6,
  /*
   Volume dimension (e.g., cubic metres, litres).
   */
  DIMENSION_ID_VOLUME = 7,
  /*
   Acceleration dimension (e.g., m/s², standard gravity).
   */
  DIMENSION_ID_ACCELERATION = 8,
  /*
   Force dimension (e.g., newtons, kilonewtons).
   */
  DIMENSION_ID_FORCE = 9,
  /*
   Energy dimension (e.g., joules, kilojoules).
   */
  DIMENSION_ID_ENERGY = 10,
  /*
   Pressure dimension (e.g., pascals, bars).
   */
  DIMENSION_ID_PRESSURE = 11,
  /*
   Solid-angle dimension (e.g., square degrees, steradians).
   */
  DIMENSION_ID_SOLID_ANGLE = 12,
  /*
   Thermodynamic temperature dimension (e.g., kelvin, rankine).
   */
  DIMENSION_ID_TEMPERATURE = 13,
  /*
   Radiance dimension (e.g., W·m^-2·sr^-1).
   */
  DIMENSION_ID_RADIANCE = 14,
  /*
   Spectral-radiance dimension (e.g., W·m^-2·sr^-1·nm^-1).
   */
  DIMENSION_ID_SPECTRAL_RADIANCE = 15,
  /*
   Photon-radiance dimension (e.g., ph·cm^-2·s^-1·sr^-1).
   */
  DIMENSION_ID_PHOTON_RADIANCE = 16,
  /*
   Spectral-photon-radiance dimension.
   */
  DIMENSION_ID_SPECTRAL_PHOTON_RADIANCE = 17,
  /*
   Inverse-solid-angle dimension (e.g., S10).
   */
  DIMENSION_ID_INVERSE_SOLID_ANGLE = 18,
  /*
   Luminous-intensity dimension (e.g., candela).
   */
  DIMENSION_ID_LUMINOUS_INTENSITY = 19,
  /*
   Luminous-flux dimension (e.g., lumen).
   */
  DIMENSION_ID_LUMINOUS_FLUX = 20,
  /*
   Illuminance dimension (e.g., lux).
   */
  DIMENSION_ID_ILLUMINANCE = 21,
  /*
   Frequency dimension (e.g., hertz).
   */
  DIMENSION_ID_FREQUENCY = 22,
  /*
   Amount-of-substance dimension (e.g., mole).
   */
  DIMENSION_ID_AMOUNT_OF_SUBSTANCE = 23,
  /*
   Electric-current dimension (e.g., ampere).
   */
  DIMENSION_ID_CURRENT = 24,
  /*
   Electric-charge dimension (e.g., coulomb).
   */
  DIMENSION_ID_CHARGE = 25,
  /*
   Voltage dimension (e.g., volt).
   */
  DIMENSION_ID_VOLTAGE = 26,
  /*
   Resistance dimension (e.g., ohm).
   */
  DIMENSION_ID_RESISTANCE = 27,
  /*
   Capacitance dimension (e.g., farad).
   */
  DIMENSION_ID_CAPACITANCE = 28,
  /*
   Inductance dimension (e.g., henry).
   */
  DIMENSION_ID_INDUCTANCE = 29,
  /*
   Magnetic-flux dimension (e.g., weber).
   */
  DIMENSION_ID_MAGNETIC_FLUX = 30,
  /*
   Magnetic-flux-density dimension (e.g., tesla).
   */
  DIMENSION_ID_MAGNETIC_FLUX_DENSITY = 31,
  /*
   Density dimension (e.g., kg/m^3).
   */
  DIMENSION_ID_DENSITY = 32,
};
#ifndef __cplusplus
typedef uint32_t DimensionId;
#endif // __cplusplus

/*
 Unit identifier for FFI.

 Each variant corresponds to a specific unit supported by the FFI layer.
 All discriminant values are explicitly assigned and are part of the ABI contract.
 */
enum UnitId
#ifdef __cplusplus
  : uint32_t
#endif // __cplusplus
 {
  /*
   PlanckLength (Length dimension)
   */
  UNIT_ID_PLANCK_LENGTH = 10000,
  /*
   Yoctometer (Length dimension)
   */
  UNIT_ID_YOCTOMETER = 10001,
  /*
   Zeptometer (Length dimension)
   */
  UNIT_ID_ZEPTOMETER = 10002,
  /*
   Attometer (Length dimension)
   */
  UNIT_ID_ATTOMETER = 10003,
  /*
   Femtometer (Length dimension)
   */
  UNIT_ID_FEMTOMETER = 10004,
  /*
   Picometer (Length dimension)
   */
  UNIT_ID_PICOMETER = 10005,
  /*
   Nanometer (Length dimension)
   */
  UNIT_ID_NANOMETER = 10006,
  /*
   Micrometer (Length dimension)
   */
  UNIT_ID_MICROMETER = 10007,
  /*
   Millimeter (Length dimension)
   */
  UNIT_ID_MILLIMETER = 10008,
  /*
   Centimeter (Length dimension)
   */
  UNIT_ID_CENTIMETER = 10009,
  /*
   Decimeter (Length dimension)
   */
  UNIT_ID_DECIMETER = 10010,
  /*
   Meter (Length dimension)
   */
  UNIT_ID_METER = 10011,
  /*
   Decameter (Length dimension)
   */
  UNIT_ID_DECAMETER = 10012,
  /*
   Hectometer (Length dimension)
   */
  UNIT_ID_HECTOMETER = 10013,
  /*
   Kilometer (Length dimension)
   */
  UNIT_ID_KILOMETER = 10014,
  /*
   Megameter (Length dimension)
   */
  UNIT_ID_MEGAMETER = 10015,
  /*
   Gigameter (Length dimension)
   */
  UNIT_ID_GIGAMETER = 10016,
  /*
   Terameter (Length dimension)
   */
  UNIT_ID_TERAMETER = 10017,
  /*
   Petameter (Length dimension)
   */
  UNIT_ID_PETAMETER = 10018,
  /*
   Exameter (Length dimension)
   */
  UNIT_ID_EXAMETER = 10019,
  /*
   Zettameter (Length dimension)
   */
  UNIT_ID_ZETTAMETER = 10020,
  /*
   Yottameter (Length dimension)
   */
  UNIT_ID_YOTTAMETER = 10021,
  /*
   BohrRadius (Length dimension)
   */
  UNIT_ID_BOHR_RADIUS = 11000,
  /*
   ClassicalElectronRadius (Length dimension)
   */
  UNIT_ID_CLASSICAL_ELECTRON_RADIUS = 11001,
  /*
   ElectronReducedComptonWavelength (Length dimension)
   */
  UNIT_ID_ELECTRON_REDUCED_COMPTON_WAVELENGTH = 11002,
  /*
   AstronomicalUnit (Length dimension)
   */
  UNIT_ID_ASTRONOMICAL_UNIT = 11003,
  /*
   LightYear (Length dimension)
   */
  UNIT_ID_LIGHT_YEAR = 11004,
  /*
   Parsec (Length dimension)
   */
  UNIT_ID_PARSEC = 11005,
  /*
   Kiloparsec (Length dimension)
   */
  UNIT_ID_KILOPARSEC = 11006,
  /*
   Megaparsec (Length dimension)
   */
  UNIT_ID_MEGAPARSEC = 11007,
  /*
   Gigaparsec (Length dimension)
   */
  UNIT_ID_GIGAPARSEC = 11008,
  /*
   Inch (Length dimension)
   */
  UNIT_ID_INCH = 12000,
  /*
   Foot (Length dimension)
   */
  UNIT_ID_FOOT = 12001,
  /*
   Yard (Length dimension)
   */
  UNIT_ID_YARD = 12002,
  /*
   Mile (Length dimension)
   */
  UNIT_ID_MILE = 12003,
  /*
   Link (Length dimension)
   */
  UNIT_ID_LINK = 13000,
  /*
   Fathom (Length dimension)
   */
  UNIT_ID_FATHOM = 13001,
  /*
   Rod (Length dimension)
   */
  UNIT_ID_ROD = 13002,
  /*
   Chain (Length dimension)
   */
  UNIT_ID_CHAIN = 13003,
  /*
   NauticalMile (Length dimension)
   */
  UNIT_ID_NAUTICAL_MILE = 13004,
  /*
   NominalLunarRadius (Length dimension)
   */
  UNIT_ID_NOMINAL_LUNAR_RADIUS = 15000,
  /*
   NominalLunarDistance (Length dimension)
   */
  UNIT_ID_NOMINAL_LUNAR_DISTANCE = 15001,
  /*
   NominalEarthPolarRadius (Length dimension)
   */
  UNIT_ID_NOMINAL_EARTH_POLAR_RADIUS = 15002,
  /*
   NominalEarthRadius (Length dimension)
   */
  UNIT_ID_NOMINAL_EARTH_RADIUS = 15003,
  /*
   NominalEarthEquatorialRadius (Length dimension)
   */
  UNIT_ID_NOMINAL_EARTH_EQUATORIAL_RADIUS = 15004,
  /*
   EarthMeridionalCircumference (Length dimension)
   */
  UNIT_ID_EARTH_MERIDIONAL_CIRCUMFERENCE = 15005,
  /*
   EarthEquatorialCircumference (Length dimension)
   */
  UNIT_ID_EARTH_EQUATORIAL_CIRCUMFERENCE = 15006,
  /*
   NominalJupiterRadius (Length dimension)
   */
  UNIT_ID_NOMINAL_JUPITER_RADIUS = 15007,
  /*
   NominalSolarRadius (Length dimension)
   */
  UNIT_ID_NOMINAL_SOLAR_RADIUS = 15008,
  /*
   NominalSolarDiameter (Length dimension)
   */
  UNIT_ID_NOMINAL_SOLAR_DIAMETER = 15009,
  /*
   Attosecond (Time dimension)
   */
  UNIT_ID_ATTOSECOND = 20000,
  /*
   Femtosecond (Time dimension)
   */
  UNIT_ID_FEMTOSECOND = 20001,
  /*
   Picosecond (Time dimension)
   */
  UNIT_ID_PICOSECOND = 20002,
  /*
   Nanosecond (Time dimension)
   */
  UNIT_ID_NANOSECOND = 20003,
  /*
   Microsecond (Time dimension)
   */
  UNIT_ID_MICROSECOND = 20004,
  /*
   Millisecond (Time dimension)
   */
  UNIT_ID_MILLISECOND = 20005,
  /*
   Centisecond (Time dimension)
   */
  UNIT_ID_CENTISECOND = 20006,
  /*
   Decisecond (Time dimension)
   */
  UNIT_ID_DECISECOND = 20007,
  /*
   Second (Time dimension)
   */
  UNIT_ID_SECOND = 20008,
  /*
   Decasecond (Time dimension)
   */
  UNIT_ID_DECASECOND = 20009,
  /*
   Hectosecond (Time dimension)
   */
  UNIT_ID_HECTOSECOND = 20010,
  /*
   Kilosecond (Time dimension)
   */
  UNIT_ID_KILOSECOND = 20011,
  /*
   Megasecond (Time dimension)
   */
  UNIT_ID_MEGASECOND = 20012,
  /*
   Gigasecond (Time dimension)
   */
  UNIT_ID_GIGASECOND = 20013,
  /*
   Terasecond (Time dimension)
   */
  UNIT_ID_TERASECOND = 20014,
  /*
   Minute (Time dimension)
   */
  UNIT_ID_MINUTE = 21000,
  /*
   Hour (Time dimension)
   */
  UNIT_ID_HOUR = 21001,
  /*
   Day (Time dimension)
   */
  UNIT_ID_DAY = 21002,
  /*
   Week (Time dimension)
   */
  UNIT_ID_WEEK = 21003,
  /*
   Fortnight (Time dimension)
   */
  UNIT_ID_FORTNIGHT = 21004,
  /*
   Year (Time dimension)
   */
  UNIT_ID_YEAR = 22000,
  /*
   Decade (Time dimension)
   */
  UNIT_ID_DECADE = 22001,
  /*
   Century (Time dimension)
   */
  UNIT_ID_CENTURY = 22002,
  /*
   Millennium (Time dimension)
   */
  UNIT_ID_MILLENNIUM = 22003,
  /*
   JulianYear (Time dimension)
   */
  UNIT_ID_JULIAN_YEAR = 22004,
  /*
   JulianCentury (Time dimension)
   */
  UNIT_ID_JULIAN_CENTURY = 22005,
  /*
   SiderealDay (Time dimension)
   */
  UNIT_ID_SIDEREAL_DAY = 23000,
  /*
   SynodicMonth (Time dimension)
   */
  UNIT_ID_SYNODIC_MONTH = 23001,
  /*
   SiderealYear (Time dimension)
   */
  UNIT_ID_SIDEREAL_YEAR = 23002,
  /*
   Milliradian (Angle dimension)
   */
  UNIT_ID_MILLIRADIAN = 30000,
  /*
   Radian (Angle dimension)
   */
  UNIT_ID_RADIAN = 30001,
  /*
   MicroArcsecond (Angle dimension)
   */
  UNIT_ID_MICRO_ARCSECOND = 31000,
  /*
   MilliArcsecond (Angle dimension)
   */
  UNIT_ID_MILLI_ARCSECOND = 31001,
  /*
   Arcsecond (Angle dimension)
   */
  UNIT_ID_ARCSECOND = 31002,
  /*
   Arcminute (Angle dimension)
   */
  UNIT_ID_ARCMINUTE = 31003,
  /*
   Degree (Angle dimension)
   */
  UNIT_ID_DEGREE = 31004,
  /*
   Gradian (Angle dimension)
   */
  UNIT_ID_GRADIAN = 32000,
  /*
   Turn (Angle dimension)
   */
  UNIT_ID_TURN = 32001,
  /*
   HourAngle (Angle dimension)
   */
  UNIT_ID_HOUR_ANGLE = 32002,
  /*
   Yoctogram (Mass dimension)
   */
  UNIT_ID_YOCTOGRAM = 40000,
  /*
   Zeptogram (Mass dimension)
   */
  UNIT_ID_ZEPTOGRAM = 40001,
  /*
   Attogram (Mass dimension)
   */
  UNIT_ID_ATTOGRAM = 40002,
  /*
   Femtogram (Mass dimension)
   */
  UNIT_ID_FEMTOGRAM = 40003,
  /*
   Picogram (Mass dimension)
   */
  UNIT_ID_PICOGRAM = 40004,
  /*
   Nanogram (Mass dimension)
   */
  UNIT_ID_NANOGRAM = 40005,
  /*
   Microgram (Mass dimension)
   */
  UNIT_ID_MICROGRAM = 40006,
  /*
   Milligram (Mass dimension)
   */
  UNIT_ID_MILLIGRAM = 40007,
  /*
   Centigram (Mass dimension)
   */
  UNIT_ID_CENTIGRAM = 40008,
  /*
   Decigram (Mass dimension)
   */
  UNIT_ID_DECIGRAM = 40009,
  /*
   Gram (Mass dimension)
   */
  UNIT_ID_GRAM = 40010,
  /*
   Decagram (Mass dimension)
   */
  UNIT_ID_DECAGRAM = 40011,
  /*
   Hectogram (Mass dimension)
   */
  UNIT_ID_HECTOGRAM = 40012,
  /*
   Kilogram (Mass dimension)
   */
  UNIT_ID_KILOGRAM = 40013,
  /*
   Megagram (Mass dimension)
   */
  UNIT_ID_MEGAGRAM = 40014,
  /*
   Gigagram (Mass dimension)
   */
  UNIT_ID_GIGAGRAM = 40015,
  /*
   Teragram (Mass dimension)
   */
  UNIT_ID_TERAGRAM = 40016,
  /*
   Petagram (Mass dimension)
   */
  UNIT_ID_PETAGRAM = 40017,
  /*
   Exagram (Mass dimension)
   */
  UNIT_ID_EXAGRAM = 40018,
  /*
   Zettagram (Mass dimension)
   */
  UNIT_ID_ZETTAGRAM = 40019,
  /*
   Yottagram (Mass dimension)
   */
  UNIT_ID_YOTTAGRAM = 40020,
  /*
   Grain (Mass dimension)
   */
  UNIT_ID_GRAIN = 41000,
  /*
   Ounce (Mass dimension)
   */
  UNIT_ID_OUNCE = 41001,
  /*
   Pound (Mass dimension)
   */
  UNIT_ID_POUND = 41002,
  /*
   Stone (Mass dimension)
   */
  UNIT_ID_STONE = 41003,
  /*
   ShortTon (Mass dimension)
   */
  UNIT_ID_SHORT_TON = 41004,
  /*
   LongTon (Mass dimension)
   */
  UNIT_ID_LONG_TON = 41005,
  /*
   Carat (Mass dimension)
   */
  UNIT_ID_CARAT = 42000,
  /*
   Tonne (Mass dimension)
   */
  UNIT_ID_TONNE = 42001,
  /*
   AtomicMassUnit (Mass dimension)
   */
  UNIT_ID_ATOMIC_MASS_UNIT = 42002,
  /*
   SolarMass (Mass dimension)
   */
  UNIT_ID_SOLAR_MASS = 42003,
  /*
   Yoctowatt (Power dimension)
   */
  UNIT_ID_YOCTOWATT = 50000,
  /*
   Zeptowatt (Power dimension)
   */
  UNIT_ID_ZEPTOWATT = 50001,
  /*
   Attowatt (Power dimension)
   */
  UNIT_ID_ATTOWATT = 50002,
  /*
   Femtowatt (Power dimension)
   */
  UNIT_ID_FEMTOWATT = 50003,
  /*
   Picowatt (Power dimension)
   */
  UNIT_ID_PICOWATT = 50004,
  /*
   Nanowatt (Power dimension)
   */
  UNIT_ID_NANOWATT = 50005,
  /*
   Microwatt (Power dimension)
   */
  UNIT_ID_MICROWATT = 50006,
  /*
   Milliwatt (Power dimension)
   */
  UNIT_ID_MILLIWATT = 50007,
  /*
   Deciwatt (Power dimension)
   */
  UNIT_ID_DECIWATT = 50008,
  /*
   Watt (Power dimension)
   */
  UNIT_ID_WATT = 50009,
  /*
   Decawatt (Power dimension)
   */
  UNIT_ID_DECAWATT = 50010,
  /*
   Hectowatt (Power dimension)
   */
  UNIT_ID_HECTOWATT = 50011,
  /*
   Kilowatt (Power dimension)
   */
  UNIT_ID_KILOWATT = 50012,
  /*
   Megawatt (Power dimension)
   */
  UNIT_ID_MEGAWATT = 50013,
  /*
   Gigawatt (Power dimension)
   */
  UNIT_ID_GIGAWATT = 50014,
  /*
   Terawatt (Power dimension)
   */
  UNIT_ID_TERAWATT = 50015,
  /*
   Petawatt (Power dimension)
   */
  UNIT_ID_PETAWATT = 50016,
  /*
   Exawatt (Power dimension)
   */
  UNIT_ID_EXAWATT = 50017,
  /*
   Zettawatt (Power dimension)
   */
  UNIT_ID_ZETTAWATT = 50018,
  /*
   Yottawatt (Power dimension)
   */
  UNIT_ID_YOTTAWATT = 50019,
  /*
   ErgPerSecond (Power dimension)
   */
  UNIT_ID_ERG_PER_SECOND = 51000,
  /*
   HorsepowerMetric (Power dimension)
   */
  UNIT_ID_HORSEPOWER_METRIC = 51001,
  /*
   HorsepowerElectric (Power dimension)
   */
  UNIT_ID_HORSEPOWER_ELECTRIC = 51002,
  /*
   SolarLuminosity (Power dimension)
   */
  UNIT_ID_SOLAR_LUMINOSITY = 51003,
  /*
   SquareMeter (Area dimension)
   */
  UNIT_ID_SQUARE_METER = 60000,
  /*
   SquareKilometer (Area dimension)
   */
  UNIT_ID_SQUARE_KILOMETER = 60001,
  /*
   SquareCentimeter (Area dimension)
   */
  UNIT_ID_SQUARE_CENTIMETER = 60002,
  /*
   SquareMillimeter (Area dimension)
   */
  UNIT_ID_SQUARE_MILLIMETER = 60003,
  /*
   Hectare (Area dimension)
   */
  UNIT_ID_HECTARE = 60004,
  /*
   Are (Area dimension)
   */
  UNIT_ID_ARE = 60005,
  /*
   SquareInch (Area dimension)
   */
  UNIT_ID_SQUARE_INCH = 60006,
  /*
   SquareFoot (Area dimension)
   */
  UNIT_ID_SQUARE_FOOT = 60007,
  /*
   SquareYard (Area dimension)
   */
  UNIT_ID_SQUARE_YARD = 60008,
  /*
   SquareMile (Area dimension)
   */
  UNIT_ID_SQUARE_MILE = 60009,
  /*
   Acre (Area dimension)
   */
  UNIT_ID_ACRE = 60010,
  /*
   CubicMeter (Volume dimension)
   */
  UNIT_ID_CUBIC_METER = 70000,
  /*
   CubicKilometer (Volume dimension)
   */
  UNIT_ID_CUBIC_KILOMETER = 70001,
  /*
   CubicCentimeter (Volume dimension)
   */
  UNIT_ID_CUBIC_CENTIMETER = 70002,
  /*
   CubicMillimeter (Volume dimension)
   */
  UNIT_ID_CUBIC_MILLIMETER = 70003,
  /*
   Liter (Volume dimension)
   */
  UNIT_ID_LITER = 70004,
  /*
   Milliliter (Volume dimension)
   */
  UNIT_ID_MILLILITER = 70005,
  /*
   Microliter (Volume dimension)
   */
  UNIT_ID_MICROLITER = 70006,
  /*
   Centiliter (Volume dimension)
   */
  UNIT_ID_CENTILITER = 70007,
  /*
   Deciliter (Volume dimension)
   */
  UNIT_ID_DECILITER = 70008,
  /*
   CubicInch (Volume dimension)
   */
  UNIT_ID_CUBIC_INCH = 70009,
  /*
   CubicFoot (Volume dimension)
   */
  UNIT_ID_CUBIC_FOOT = 70010,
  /*
   UsGallon (Volume dimension)
   */
  UNIT_ID_US_GALLON = 70011,
  /*
   UsFluidOunce (Volume dimension)
   */
  UNIT_ID_US_FLUID_OUNCE = 70012,
  /*
   MeterPerSecondSquared (Acceleration dimension)
   */
  UNIT_ID_METER_PER_SECOND_SQUARED = 80000,
  /*
   StandardGravity (Acceleration dimension)
   */
  UNIT_ID_STANDARD_GRAVITY = 80001,
  /*
   Newton (Force dimension)
   */
  UNIT_ID_NEWTON = 90000,
  /*
   Micronewton (Force dimension)
   */
  UNIT_ID_MICRONEWTON = 90001,
  /*
   Millinewton (Force dimension)
   */
  UNIT_ID_MILLINEWTON = 90002,
  /*
   Kilonewton (Force dimension)
   */
  UNIT_ID_KILONEWTON = 90003,
  /*
   Meganewton (Force dimension)
   */
  UNIT_ID_MEGANEWTON = 90004,
  /*
   Giganewton (Force dimension)
   */
  UNIT_ID_GIGANEWTON = 90005,
  /*
   Dyne (Force dimension)
   */
  UNIT_ID_DYNE = 91000,
  /*
   PoundForce (Force dimension)
   */
  UNIT_ID_POUND_FORCE = 92000,
  /*
   Joule (Energy dimension)
   */
  UNIT_ID_JOULE = 100000,
  /*
   Picojoule (Energy dimension)
   */
  UNIT_ID_PICOJOULE = 100001,
  /*
   Nanojoule (Energy dimension)
   */
  UNIT_ID_NANOJOULE = 100002,
  /*
   Microjoule (Energy dimension)
   */
  UNIT_ID_MICROJOULE = 100003,
  /*
   Millijoule (Energy dimension)
   */
  UNIT_ID_MILLIJOULE = 100004,
  /*
   Kilojoule (Energy dimension)
   */
  UNIT_ID_KILOJOULE = 100005,
  /*
   Megajoule (Energy dimension)
   */
  UNIT_ID_MEGAJOULE = 100006,
  /*
   Gigajoule (Energy dimension)
   */
  UNIT_ID_GIGAJOULE = 100007,
  /*
   Terajoule (Energy dimension)
   */
  UNIT_ID_TERAJOULE = 100008,
  /*
   WattHour (Energy dimension)
   */
  UNIT_ID_WATT_HOUR = 100009,
  /*
   KilowattHour (Energy dimension)
   */
  UNIT_ID_KILOWATT_HOUR = 100010,
  /*
   Erg (Energy dimension)
   */
  UNIT_ID_ERG = 101000,
  /*
   Electronvolt (Energy dimension)
   */
  UNIT_ID_ELECTRONVOLT = 101001,
  /*
   Kiloelectronvolt (Energy dimension)
   */
  UNIT_ID_KILOELECTRONVOLT = 101002,
  /*
   Megaelectronvolt (Energy dimension)
   */
  UNIT_ID_MEGAELECTRONVOLT = 101003,
  /*
   Calorie (Energy dimension)
   */
  UNIT_ID_CALORIE = 102000,
  /*
   Kilocalorie (Energy dimension)
   */
  UNIT_ID_KILOCALORIE = 102001,
  /*
   BritishThermalUnit (Energy dimension)
   */
  UNIT_ID_BRITISH_THERMAL_UNIT = 102002,
  /*
   Therm (Energy dimension)
   */
  UNIT_ID_THERM = 102003,
  /*
   Pascal (Pressure dimension)
   */
  UNIT_ID_PASCAL = 110000,
  /*
   Millipascal (Pressure dimension)
   */
  UNIT_ID_MILLIPASCAL = 110001,
  /*
   Hectopascal (Pressure dimension)
   */
  UNIT_ID_HECTOPASCAL = 110002,
  /*
   Kilopascal (Pressure dimension)
   */
  UNIT_ID_KILOPASCAL = 110003,
  /*
   Megapascal (Pressure dimension)
   */
  UNIT_ID_MEGAPASCAL = 110004,
  /*
   Gigapascal (Pressure dimension)
   */
  UNIT_ID_GIGAPASCAL = 110005,
  /*
   Bar (Pressure dimension)
   */
  UNIT_ID_BAR = 110006,
  /*
   Atmosphere (Pressure dimension)
   */
  UNIT_ID_ATMOSPHERE = 111000,
  /*
   Torr (Pressure dimension)
   */
  UNIT_ID_TORR = 111001,
  /*
   MillimeterOfMercury (Pressure dimension)
   */
  UNIT_ID_MILLIMETER_OF_MERCURY = 111002,
  /*
   PoundPerSquareInch (Pressure dimension)
   */
  UNIT_ID_POUND_PER_SQUARE_INCH = 111003,
  /*
   InchOfMercury (Pressure dimension)
   */
  UNIT_ID_INCH_OF_MERCURY = 111004,
  /*
   SquareDegree (SolidAngle dimension)
   */
  UNIT_ID_SQUARE_DEGREE = 120000,
  /*
   Steradian (SolidAngle dimension)
   */
  UNIT_ID_STERADIAN = 120001,
  /*
   SquareMilliradian (SolidAngle dimension)
   */
  UNIT_ID_SQUARE_MILLIRADIAN = 120002,
  /*
   SquareArcminute (SolidAngle dimension)
   */
  UNIT_ID_SQUARE_ARCMINUTE = 121000,
  /*
   SquareArcsecond (SolidAngle dimension)
   */
  UNIT_ID_SQUARE_ARCSECOND = 121001,
  /*
   Kelvin (Temperature dimension)
   */
  UNIT_ID_KELVIN = 130000,
  /*
   Rankine (Temperature dimension)
   */
  UNIT_ID_RANKINE = 130001,
  /*
   WattPerSquareMeterSteradian (Radiance dimension)
   */
  UNIT_ID_WATT_PER_SQUARE_METER_STERADIAN = 140000,
  /*
   ErgPerSecondSquareCentimeterSteradian (Radiance dimension)
   */
  UNIT_ID_ERG_PER_SECOND_SQUARE_CENTIMETER_STERADIAN = 140001,
  /*
   WattPerSquareMeterSteradianMeter (SpectralRadiance dimension)
   */
  UNIT_ID_WATT_PER_SQUARE_METER_STERADIAN_METER = 150000,
  /*
   WattPerSquareMeterSteradianNanometer (SpectralRadiance dimension)
   */
  UNIT_ID_WATT_PER_SQUARE_METER_STERADIAN_NANOMETER = 150001,
  /*
   ErgPerSecondSquareCentimeterSteradianAngstrom (SpectralRadiance dimension)
   */
  UNIT_ID_ERG_PER_SECOND_SQUARE_CENTIMETER_STERADIAN_ANGSTROM = 150002,
  /*
   PhotonPerSquareMeterSecondSteradian (PhotonRadiance dimension)
   */
  UNIT_ID_PHOTON_PER_SQUARE_METER_SECOND_STERADIAN = 160000,
  /*
   PhotonPerSquareCentimeterSecondSteradian (PhotonRadiance dimension)
   */
  UNIT_ID_PHOTON_PER_SQUARE_CENTIMETER_SECOND_STERADIAN = 160001,
  /*
   PhotonPerSquareCentimeterNanosecondSteradian (PhotonRadiance dimension)
   */
  UNIT_ID_PHOTON_PER_SQUARE_CENTIMETER_NANOSECOND_STERADIAN = 160002,
  /*
   PhotonPerSquareMeterSecondSteradianMeter (SpectralPhotonRadiance dimension)
   */
  UNIT_ID_PHOTON_PER_SQUARE_METER_SECOND_STERADIAN_METER = 170000,
  /*
   PhotonPerSquareCentimeterSecondSteradianAngstrom (SpectralPhotonRadiance dimension)
   */
  UNIT_ID_PHOTON_PER_SQUARE_CENTIMETER_SECOND_STERADIAN_ANGSTROM = 170001,
  /*
   PhotonPerSquareCentimeterSecondSteradianNanometer (SpectralPhotonRadiance dimension)
   */
  UNIT_ID_PHOTON_PER_SQUARE_CENTIMETER_SECOND_STERADIAN_NANOMETER = 170002,
  /*
   PhotonPerSquareCentimeterNanosecondSteradianNanometer (SpectralPhotonRadiance dimension)
   */
  UNIT_ID_PHOTON_PER_SQUARE_CENTIMETER_NANOSECOND_STERADIAN_NANOMETER = 170003,
  /*
   S10 (InverseSolidAngle dimension)
   */
  UNIT_ID_S10 = 180000,
  /*
   Candela (LuminousIntensity dimension)
   */
  UNIT_ID_CANDELA = 190000,
  /*
   Lumen (LuminousFlux dimension)
   */
  UNIT_ID_LUMEN = 200000,
  /*
   Millilumen (LuminousFlux dimension)
   */
  UNIT_ID_MILLILUMEN = 200001,
  /*
   Kilolumen (LuminousFlux dimension)
   */
  UNIT_ID_KILOLUMEN = 200002,
  /*
   Lux (Illuminance dimension)
   */
  UNIT_ID_LUX = 210000,
  /*
   Millilux (Illuminance dimension)
   */
  UNIT_ID_MILLILUX = 210001,
  /*
   Kilolux (Illuminance dimension)
   */
  UNIT_ID_KILOLUX = 210002,
  /*
   Hertz (Frequency dimension)
   */
  UNIT_ID_HERTZ = 220000,
  /*
   Millihertz (Frequency dimension)
   */
  UNIT_ID_MILLIHERTZ = 220001,
  /*
   Kilohertz (Frequency dimension)
   */
  UNIT_ID_KILOHERTZ = 220002,
  /*
   Megahertz (Frequency dimension)
   */
  UNIT_ID_MEGAHERTZ = 220003,
  /*
   Gigahertz (Frequency dimension)
   */
  UNIT_ID_GIGAHERTZ = 220004,
  /*
   Terahertz (Frequency dimension)
   */
  UNIT_ID_TERAHERTZ = 220005,
  /*
   Mole (AmountOfSubstance dimension)
   */
  UNIT_ID_MOLE = 230000,
  /*
   Nanomole (AmountOfSubstance dimension)
   */
  UNIT_ID_NANOMOLE = 230001,
  /*
   Micromole (AmountOfSubstance dimension)
   */
  UNIT_ID_MICROMOLE = 230002,
  /*
   Millimole (AmountOfSubstance dimension)
   */
  UNIT_ID_MILLIMOLE = 230003,
  /*
   Kilomole (AmountOfSubstance dimension)
   */
  UNIT_ID_KILOMOLE = 230004,
  /*
   Ampere (Current dimension)
   */
  UNIT_ID_AMPERE = 240000,
  /*
   Microampere (Current dimension)
   */
  UNIT_ID_MICROAMPERE = 240001,
  /*
   Milliampere (Current dimension)
   */
  UNIT_ID_MILLIAMPERE = 240002,
  /*
   Kiloampere (Current dimension)
   */
  UNIT_ID_KILOAMPERE = 240003,
  /*
   Coulomb (Charge dimension)
   */
  UNIT_ID_COULOMB = 250000,
  /*
   Millicoulomb (Charge dimension)
   */
  UNIT_ID_MILLICOULOMB = 250001,
  /*
   Microcoulomb (Charge dimension)
   */
  UNIT_ID_MICROCOULOMB = 250002,
  /*
   Kilocoulomb (Charge dimension)
   */
  UNIT_ID_KILOCOULOMB = 250003,
  /*
   Volt (Voltage dimension)
   */
  UNIT_ID_VOLT = 260000,
  /*
   Microvolt (Voltage dimension)
   */
  UNIT_ID_MICROVOLT = 260001,
  /*
   Millivolt (Voltage dimension)
   */
  UNIT_ID_MILLIVOLT = 260002,
  /*
   Kilovolt (Voltage dimension)
   */
  UNIT_ID_KILOVOLT = 260003,
  /*
   Megavolt (Voltage dimension)
   */
  UNIT_ID_MEGAVOLT = 260004,
  /*
   Ohm (Resistance dimension)
   */
  UNIT_ID_OHM = 270000,
  /*
   Milliohm (Resistance dimension)
   */
  UNIT_ID_MILLIOHM = 270001,
  /*
   Kilohm (Resistance dimension)
   */
  UNIT_ID_KILOHM = 270002,
  /*
   Megaohm (Resistance dimension)
   */
  UNIT_ID_MEGAOHM = 270003,
  /*
   Farad (Capacitance dimension)
   */
  UNIT_ID_FARAD = 280000,
  /*
   Picofarad (Capacitance dimension)
   */
  UNIT_ID_PICOFARAD = 280001,
  /*
   Nanofarad (Capacitance dimension)
   */
  UNIT_ID_NANOFARAD = 280002,
  /*
   Microfarad (Capacitance dimension)
   */
  UNIT_ID_MICROFARAD = 280003,
  /*
   Millifarad (Capacitance dimension)
   */
  UNIT_ID_MILLIFARAD = 280004,
  /*
   Henry (Inductance dimension)
   */
  UNIT_ID_HENRY = 290000,
  /*
   Microhenry (Inductance dimension)
   */
  UNIT_ID_MICROHENRY = 290001,
  /*
   Millihenry (Inductance dimension)
   */
  UNIT_ID_MILLIHENRY = 290002,
  /*
   Weber (MagneticFlux dimension)
   */
  UNIT_ID_WEBER = 300000,
  /*
   Milliweber (MagneticFlux dimension)
   */
  UNIT_ID_MILLIWEBER = 300001,
  /*
   Tesla (MagneticFluxDensity dimension)
   */
  UNIT_ID_TESLA = 310000,
  /*
   Millitesla (MagneticFluxDensity dimension)
   */
  UNIT_ID_MILLITESLA = 310001,
  /*
   Microtesla (MagneticFluxDensity dimension)
   */
  UNIT_ID_MICROTESLA = 310002,
  /*
   KilogramPerCubicMeter (Density dimension)
   */
  UNIT_ID_KILOGRAM_PER_CUBIC_METER = 320000,
  /*
   GramPerCubicCentimeter (Density dimension)
   */
  UNIT_ID_GRAM_PER_CUBIC_CENTIMETER = 320001,
  /*
   GramPerMilliliter (Density dimension)
   */
  UNIT_ID_GRAM_PER_MILLILITER = 320002,
  /*
   PoundPerCubicFoot (Density dimension)
   */
  UNIT_ID_POUND_PER_CUBIC_FOOT = 321000,
};
#ifndef __cplusplus
typedef uint32_t UnitId;
#endif // __cplusplus

/*
 A derived quantity representing a compound unit (numerator/denominator).

 This is useful for quantities like velocity (m/s), frequency (rad/s), etc.

 # ABI Safety

 Like [`QttyQuantity`], the unit fields are raw `u32` values to prevent UB
 from invalid enum discriminants constructed by C callers.

 # ABI Stability

 This struct has `#[repr(C)]` layout:
 - `value` at offset 0 (8 bytes)
 - `numerator` at offset 8 (4 bytes)
 - `denominator` at offset 12 (4 bytes)
 - Total size: 16 bytes

 # Example

 ```rust
 use qtty_ffi::{QttyDerivedQuantity, UnitId};

 // Create a velocity: 100 m/s
 let velocity = QttyDerivedQuantity::new(100.0, UnitId::Meter, UnitId::Second);
 assert_eq!(velocity.value, 100.0);
 assert_eq!(velocity.numerator, UnitId::Meter as u32);
 assert_eq!(velocity.denominator, UnitId::Second as u32);
 ```
 */
typedef struct qtty_derived_quantity_t {
  /*
   The numeric value of the derived quantity.
   */
  double value;
  /*
   Raw numerator unit identifier.
   */
  uint32_t numerator;
  /*
   Raw denominator unit identifier.
   */
  uint32_t denominator;
} qtty_derived_quantity_t;

/*
 A POD quantity carrier type suitable for FFI.

 This struct represents a physical quantity as a value paired with its unit.
 It is `#[repr(C)]` to ensure a stable, predictable memory layout across
 language boundaries.

 # ABI Safety

 The `unit` field is a raw `u32` rather than a [`UnitId`] enum so that C
 callers can never construct a value with an invalid discriminant (which
 would be instant UB in Rust).  Use [`unit_id()`](Self::unit_id) to
 validate and decode the field on the Rust side.

 # Memory Layout

 - `value`: 8 bytes (f64)
 - `unit`: 4 bytes (u32)
 - Padding: 4 bytes (for alignment)
 - Total: 16 bytes on most platforms

 # Example

 ```rust
 use qtty_ffi::{QttyQuantity, UnitId};

 let q = QttyQuantity {
     value: 1000.0,
     unit: UnitId::Meter as u32,
 };
 ```
 */
typedef struct qtty_quantity_t {
  /*
   The numeric value of the quantity.
   */
  double value;
  /*
   Raw unit identifier.  Use [`unit_id()`](Self::unit_id) to validate.
   */
  uint32_t unit;
} qtty_quantity_t;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/*
 Creates a new derived quantity (compound unit like m/s).

 # Arguments

 * `value`          - The numeric value
 * `numerator_id`   - Raw `uint32_t` numerator unit identifier
 * `denominator_id` - Raw `uint32_t` denominator unit identifier
 * `out`            - Pointer to store the resulting [`QttyDerivedQuantity`]

 # Safety

 `out` must be a valid, writable pointer to [`QttyDerivedQuantity`], or null.
 */
QttyStatus qtty_derived_make(double value,
                             uint32_t numerator_id,
                             uint32_t denominator_id,
                             struct qtty_derived_quantity_t *out);

/*
 Converts a derived quantity to different units.

 # Safety

 `out` must be a valid, writable pointer to [`QttyDerivedQuantity`], or null.
 */
QttyStatus qtty_derived_convert(struct qtty_derived_quantity_t src,
                                uint32_t target_num_id,
                                uint32_t target_den_id,
                                struct qtty_derived_quantity_t *out);

/*
 Creates a new quantity with the given value and unit.

 # Arguments

 * `value`   - The numeric value
 * `unit_id` - Raw `uint32_t` unit identifier
 * `out`     - Pointer to store the resulting [`QttyQuantity`]

 # Returns

 * [`QttyStatus::Ok`] on success
 * [`QttyStatus::NullOut`] if `out` is null
 * [`QttyStatus::UnknownUnit`] if the unit ID is not recognized

 # Safety

 `out` must be a valid, writable pointer to [`QttyQuantity`], or null.
 */
QttyStatus qtty_quantity_make(double value, uint32_t unit_id, struct qtty_quantity_t *out);

/*
 Converts a quantity to a different unit.

 # Arguments

 * `src`         - The source quantity
 * `dst_unit_id` - Raw `uint32_t` target unit identifier
 * `out`         - Pointer to store the converted [`QttyQuantity`]

 # Returns

 * [`QttyStatus::Ok`] on success
 * [`QttyStatus::NullOut`] if `out` is null
 * [`QttyStatus::UnknownUnit`] if either unit ID is not recognized
 * [`QttyStatus::IncompatibleDim`] if units have different dimensions

 # Safety

 `out` must be a valid, writable pointer to [`QttyQuantity`], or null.
 */
QttyStatus qtty_quantity_convert(struct qtty_quantity_t src,
                                 uint32_t dst_unit_id,
                                 struct qtty_quantity_t *out);

/*
 Converts a raw value from one unit to another.

 # Arguments

 * `value`       - The numeric value to convert
 * `src_unit_id` - Raw `uint32_t` source unit identifier
 * `dst_unit_id` - Raw `uint32_t` target unit identifier
 * `out_value`   - Pointer to store the converted `f64`

 # Returns

 * [`QttyStatus::Ok`] on success
 * [`QttyStatus::NullOut`] if `out_value` is null
 * [`QttyStatus::UnknownUnit`] if either unit ID is not recognized
 * [`QttyStatus::IncompatibleDim`] if units have different dimensions

 # Safety

 `out_value` must be a valid, writable pointer to `f64`, or null.
 */
QttyStatus qtty_quantity_convert_value(double value,
                                       uint32_t src_unit_id,
                                       uint32_t dst_unit_id,
                                       double *out_value);

/*
 Formats a quantity as a human-readable string into a caller-provided buffer.

 Produces strings like `"1234.57 m"`, `"1.23e3 km"`, or `"1.23E3 km"`.

 # Arguments

 * `qty`       - The quantity to format.
 * `precision` - Decimal digits after the point; `-1` for default (shortest).
 * `flags`     - `QTTY_FMT_DEFAULT` (0), `QTTY_FMT_LOWER_EXP` (1), or
   `QTTY_FMT_UPPER_EXP` (2).
 * `buf`       - Caller-allocated output buffer (non-null).
 * `buf_len`   - Size of `buf` in bytes (must include space for NUL).

 # Returns

 * `QttyStatus::Ok` and the buffer is filled on success.
 * `QttyStatus::BufferTooSmall` if `buf_len` is zero or insufficient.
 * `QttyStatus::NullOut` if `buf` is null.
 * `QttyStatus::UnknownUnit` if the quantity's unit is not recognized.

 # Safety

 `buf` must point to a writable allocation of at least `buf_len` bytes.
 */
QttyStatus qtty_quantity_format(struct qtty_quantity_t qty,
                                int32_t precision,
                                uint32_t flags,
                                char *buf,
                                size_t buf_len);

/*
 Checks if a raw unit ID is valid (recognized by the registry).

 # Arguments

 * `unit_id` - Raw `uint32_t` unit identifier

 # Returns

 `true` if the unit is valid, `false` otherwise.
 */
bool qtty_unit_is_valid(uint32_t unit_id);

/*
 Gets the dimension of a unit.

 # Arguments

 * `unit_id` - Raw `uint32_t` unit identifier
 * `out`     - Pointer to store the [`DimensionId`]

 # Returns

 * [`QttyStatus::Ok`] on success
 * [`QttyStatus::NullOut`] if `out` is null
 * [`QttyStatus::UnknownUnit`] if the unit ID is not recognized

 # Safety

 `out` must be a valid, writable pointer to `DimensionId`, or null.
 */
QttyStatus qtty_unit_dimension(uint32_t unit_id, DimensionId *out);

/*
 Checks if two units are compatible (same dimension).

 # Arguments

 * `a_id` - Raw `uint32_t` unit identifier for the first unit
 * `b_id` - Raw `uint32_t` unit identifier for the second unit
 * `out`  - Pointer to store the result (`bool`)

 # Returns

 * [`QttyStatus::Ok`] on success
 * [`QttyStatus::NullOut`] if `out` is null
 * [`QttyStatus::UnknownUnit`] if either unit ID is not recognized

 # Safety

 `out` must be a valid, writable pointer to `bool`, or null.
 */
QttyStatus qtty_units_compatible(uint32_t a_id, uint32_t b_id, bool *out);

/*
 Gets the name of a unit as a NUL-terminated C string.

 # Arguments

 * `unit_id` - Raw `uint32_t` unit identifier

 # Returns

 A pointer to a static, NUL-terminated C string, or null if the unit ID is
 not recognized. The pointer points to static memory; the caller must not
 free or modify it.
 */
const char *qtty_unit_name(uint32_t unit_id);

/*
 Returns the FFI ABI version (major*10000 + minor*100 + patch).

 Current version: 0.7.0 → 700

 The 0.7.x ABI extends the FFI unit catalog to cover the full `qtty`
 linear-unit inventory while continuing to use raw `u32` unit identifiers in
 `QttyQuantity` and `QttyDerivedQuantity` so C callers cannot construct
 invalid Rust enum discriminants across the FFI boundary.
 */
uint32_t qtty_ffi_version(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* QTTY_FFI_H */

/* End of qtty_ffi.h */
