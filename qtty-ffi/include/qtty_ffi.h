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
 Success status code.
 */
#define QTTY_OK 0

/*
 Error: the provided unit ID is not recognized/valid.
 */
#define QTTY_ERR_UNKNOWN_UNIT -1

/*
 Error: conversion requested between incompatible dimensions.
 */
#define QTTY_ERR_INCOMPATIBLE_DIM -2

/*
 Error: a required output pointer was null.
 */
#define QTTY_ERR_NULL_OUT -3

/*
 Error: the provided value is invalid (reserved for future use).
 */
#define QTTY_ERR_INVALID_VALUE -4

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
   PlanckLength (l_P)
   */
  UNIT_ID_PLANCK_LENGTH = 10000,
  /*
   Yoctometer (ym)
   */
  UNIT_ID_YOCTOMETER = 10001,
  /*
   Zeptometer (zm)
   */
  UNIT_ID_ZEPTOMETER = 10002,
  /*
   Attometer (am)
   */
  UNIT_ID_ATTOMETER = 10003,
  /*
   Femtometer (fm)
   */
  UNIT_ID_FEMTOMETER = 10004,
  /*
   Picometer (pm)
   */
  UNIT_ID_PICOMETER = 10005,
  /*
   Nanometer (nm)
   */
  UNIT_ID_NANOMETER = 10006,
  /*
   Micrometer (µm)
   */
  UNIT_ID_MICROMETER = 10007,
  /*
   Millimeter (mm)
   */
  UNIT_ID_MILLIMETER = 10008,
  /*
   Centimeter (cm)
   */
  UNIT_ID_CENTIMETER = 10009,
  /*
   Decimeter (dm)
   */
  UNIT_ID_DECIMETER = 10010,
  /*
   Meter (m)
   */
  UNIT_ID_METER = 10011,
  /*
   Decameter (dam)
   */
  UNIT_ID_DECAMETER = 10012,
  /*
   Hectometer (hm)
   */
  UNIT_ID_HECTOMETER = 10013,
  /*
   Kilometer (km)
   */
  UNIT_ID_KILOMETER = 10014,
  /*
   Megameter (Mm)
   */
  UNIT_ID_MEGAMETER = 10015,
  /*
   Gigameter (Gm)
   */
  UNIT_ID_GIGAMETER = 10016,
  /*
   Terameter (Tm)
   */
  UNIT_ID_TERAMETER = 10017,
  /*
   Petameter (Pm)
   */
  UNIT_ID_PETAMETER = 10018,
  /*
   Exameter (Em)
   */
  UNIT_ID_EXAMETER = 10019,
  /*
   Zettameter (Zm)
   */
  UNIT_ID_ZETTAMETER = 10020,
  /*
   Yottameter (Ym)
   */
  UNIT_ID_YOTTAMETER = 10021,
  /*
   BohrRadius (a₀)
   */
  UNIT_ID_BOHR_RADIUS = 11000,
  /*
   ClassicalElectronRadius (r_e)
   */
  UNIT_ID_CLASSICAL_ELECTRON_RADIUS = 11001,
  /*
   ElectronReducedComptonWavelength (λ̄_e)
   */
  UNIT_ID_ELECTRON_REDUCED_COMPTON_WAVELENGTH = 11002,
  /*
   AstronomicalUnit (au)
   */
  UNIT_ID_ASTRONOMICAL_UNIT = 11003,
  /*
   LightYear (ly)
   */
  UNIT_ID_LIGHT_YEAR = 11004,
  /*
   Parsec (pc)
   */
  UNIT_ID_PARSEC = 11005,
  /*
   Kiloparsec (kpc)
   */
  UNIT_ID_KILOPARSEC = 11006,
  /*
   Megaparsec (Mpc)
   */
  UNIT_ID_MEGAPARSEC = 11007,
  /*
   Gigaparsec (Gpc)
   */
  UNIT_ID_GIGAPARSEC = 11008,
  /*
   Inch (in)
   */
  UNIT_ID_INCH = 12000,
  /*
   Foot (ft)
   */
  UNIT_ID_FOOT = 12001,
  /*
   Yard (yd)
   */
  UNIT_ID_YARD = 12002,
  /*
   Mile (mi)
   */
  UNIT_ID_MILE = 12003,
  /*
   Link (lk)
   */
  UNIT_ID_LINK = 13000,
  /*
   Fathom (ftm)
   */
  UNIT_ID_FATHOM = 13001,
  /*
   Rod (rd)
   */
  UNIT_ID_ROD = 13002,
  /*
   Chain (ch)
   */
  UNIT_ID_CHAIN = 13003,
  /*
   NauticalMile (nmi)
   */
  UNIT_ID_NAUTICAL_MILE = 13004,
  /*
   NominalLunarRadius (R_☾)
   */
  UNIT_ID_NOMINAL_LUNAR_RADIUS = 15000,
  /*
   NominalLunarDistance (LD)
   */
  UNIT_ID_NOMINAL_LUNAR_DISTANCE = 15001,
  /*
   NominalEarthPolarRadius (R_⊕pol)
   */
  UNIT_ID_NOMINAL_EARTH_POLAR_RADIUS = 15002,
  /*
   NominalEarthRadius (R_⊕)
   */
  UNIT_ID_NOMINAL_EARTH_RADIUS = 15003,
  /*
   NominalEarthEquatorialRadius (R_⊕eq)
   */
  UNIT_ID_NOMINAL_EARTH_EQUATORIAL_RADIUS = 15004,
  /*
   EarthMeridionalCircumference (C_mer)
   */
  UNIT_ID_EARTH_MERIDIONAL_CIRCUMFERENCE = 15005,
  /*
   EarthEquatorialCircumference (C_eq)
   */
  UNIT_ID_EARTH_EQUATORIAL_CIRCUMFERENCE = 15006,
  /*
   NominalJupiterRadius (R_♃)
   */
  UNIT_ID_NOMINAL_JUPITER_RADIUS = 15007,
  /*
   NominalSolarRadius (R_☉)
   */
  UNIT_ID_NOMINAL_SOLAR_RADIUS = 15008,
  /*
   NominalSolarDiameter (D_☉)
   */
  UNIT_ID_NOMINAL_SOLAR_DIAMETER = 15009,
  /*
   Attosecond (as)
   */
  UNIT_ID_ATTOSECOND = 20000,
  /*
   Femtosecond (fs)
   */
  UNIT_ID_FEMTOSECOND = 20001,
  /*
   Picosecond (ps)
   */
  UNIT_ID_PICOSECOND = 20002,
  /*
   Nanosecond (ns)
   */
  UNIT_ID_NANOSECOND = 20003,
  /*
   Microsecond (µs)
   */
  UNIT_ID_MICROSECOND = 20004,
  /*
   Millisecond (ms)
   */
  UNIT_ID_MILLISECOND = 20005,
  /*
   Centisecond (cs)
   */
  UNIT_ID_CENTISECOND = 20006,
  /*
   Decisecond (ds)
   */
  UNIT_ID_DECISECOND = 20007,
  /*
   Second (s)
   */
  UNIT_ID_SECOND = 20008,
  /*
   Decasecond (das)
   */
  UNIT_ID_DECASECOND = 20009,
  /*
   Hectosecond (hs)
   */
  UNIT_ID_HECTOSECOND = 20010,
  /*
   Kilosecond (ks)
   */
  UNIT_ID_KILOSECOND = 20011,
  /*
   Megasecond (Ms)
   */
  UNIT_ID_MEGASECOND = 20012,
  /*
   Gigasecond (Gs)
   */
  UNIT_ID_GIGASECOND = 20013,
  /*
   Terasecond (Ts)
   */
  UNIT_ID_TERASECOND = 20014,
  /*
   Minute (min)
   */
  UNIT_ID_MINUTE = 21000,
  /*
   Hour (h)
   */
  UNIT_ID_HOUR = 21001,
  /*
   Day (d)
   */
  UNIT_ID_DAY = 21002,
  /*
   Week (wk)
   */
  UNIT_ID_WEEK = 21003,
  /*
   Fortnight (fn)
   */
  UNIT_ID_FORTNIGHT = 21004,
  /*
   Year (yr)
   */
  UNIT_ID_YEAR = 22000,
  /*
   Decade (dec)
   */
  UNIT_ID_DECADE = 22001,
  /*
   Century (c)
   */
  UNIT_ID_CENTURY = 22002,
  /*
   Millennium (mill)
   */
  UNIT_ID_MILLENNIUM = 22003,
  /*
   JulianYear (a)
   */
  UNIT_ID_JULIAN_YEAR = 22004,
  /*
   JulianCentury (jc)
   */
  UNIT_ID_JULIAN_CENTURY = 22005,
  /*
   SiderealDay (sd)
   */
  UNIT_ID_SIDEREAL_DAY = 23000,
  /*
   SynodicMonth (mo_s)
   */
  UNIT_ID_SYNODIC_MONTH = 23001,
  /*
   SiderealYear (yr_s)
   */
  UNIT_ID_SIDEREAL_YEAR = 23002,
  /*
   Milliradian (mrad)
   */
  UNIT_ID_MILLIRADIAN = 30000,
  /*
   Radian (rad)
   */
  UNIT_ID_RADIAN = 30001,
  /*
   MicroArcsecond (µas)
   */
  UNIT_ID_MICRO_ARCSECOND = 31000,
  /*
   MilliArcsecond (mas)
   */
  UNIT_ID_MILLI_ARCSECOND = 31001,
  /*
   Arcsecond (″)
   */
  UNIT_ID_ARCSECOND = 31002,
  /*
   Arcminute (′)
   */
  UNIT_ID_ARCMINUTE = 31003,
  /*
   Degree (°)
   */
  UNIT_ID_DEGREE = 31004,
  /*
   Gradian (gon)
   */
  UNIT_ID_GRADIAN = 32000,
  /*
   Turn (tr)
   */
  UNIT_ID_TURN = 32001,
  /*
   HourAngle (ʰ)
   */
  UNIT_ID_HOUR_ANGLE = 32002,
  /*
   Yoctogram (yg)
   */
  UNIT_ID_YOCTOGRAM = 40000,
  /*
   Zeptogram (zg)
   */
  UNIT_ID_ZEPTOGRAM = 40001,
  /*
   Attogram (ag)
   */
  UNIT_ID_ATTOGRAM = 40002,
  /*
   Femtogram (fg)
   */
  UNIT_ID_FEMTOGRAM = 40003,
  /*
   Picogram (pg)
   */
  UNIT_ID_PICOGRAM = 40004,
  /*
   Nanogram (ng)
   */
  UNIT_ID_NANOGRAM = 40005,
  /*
   Microgram (µg)
   */
  UNIT_ID_MICROGRAM = 40006,
  /*
   Milligram (mg)
   */
  UNIT_ID_MILLIGRAM = 40007,
  /*
   Centigram (cg)
   */
  UNIT_ID_CENTIGRAM = 40008,
  /*
   Decigram (dg)
   */
  UNIT_ID_DECIGRAM = 40009,
  /*
   Gram (g)
   */
  UNIT_ID_GRAM = 40010,
  /*
   Decagram (dag)
   */
  UNIT_ID_DECAGRAM = 40011,
  /*
   Hectogram (hg)
   */
  UNIT_ID_HECTOGRAM = 40012,
  /*
   Kilogram (kg)
   */
  UNIT_ID_KILOGRAM = 40013,
  /*
   Megagram (Mg)
   */
  UNIT_ID_MEGAGRAM = 40014,
  /*
   Gigagram (Gg)
   */
  UNIT_ID_GIGAGRAM = 40015,
  /*
   Teragram (Tg)
   */
  UNIT_ID_TERAGRAM = 40016,
  /*
   Petagram (Pg)
   */
  UNIT_ID_PETAGRAM = 40017,
  /*
   Exagram (Eg)
   */
  UNIT_ID_EXAGRAM = 40018,
  /*
   Zettagram (Zg)
   */
  UNIT_ID_ZETTAGRAM = 40019,
  /*
   Yottagram (Yg)
   */
  UNIT_ID_YOTTAGRAM = 40020,
  /*
   Grain (gr)
   */
  UNIT_ID_GRAIN = 41000,
  /*
   Ounce (oz)
   */
  UNIT_ID_OUNCE = 41001,
  /*
   Pound (lb)
   */
  UNIT_ID_POUND = 41002,
  /*
   Stone (st)
   */
  UNIT_ID_STONE = 41003,
  /*
   ShortTon (ton)
   */
  UNIT_ID_SHORT_TON = 41004,
  /*
   LongTon (ton_l)
   */
  UNIT_ID_LONG_TON = 41005,
  /*
   Carat (ct)
   */
  UNIT_ID_CARAT = 42000,
  /*
   Tonne (t)
   */
  UNIT_ID_TONNE = 42001,
  /*
   AtomicMassUnit (u)
   */
  UNIT_ID_ATOMIC_MASS_UNIT = 42002,
  /*
   SolarMass (M_☉)
   */
  UNIT_ID_SOLAR_MASS = 42003,
  /*
   Yoctowatt (yW)
   */
  UNIT_ID_YOCTOWATT = 50000,
  /*
   Zeptowatt (zW)
   */
  UNIT_ID_ZEPTOWATT = 50001,
  /*
   Attowatt (aW)
   */
  UNIT_ID_ATTOWATT = 50002,
  /*
   Femtowatt (fW)
   */
  UNIT_ID_FEMTOWATT = 50003,
  /*
   Picowatt (pW)
   */
  UNIT_ID_PICOWATT = 50004,
  /*
   Nanowatt (nW)
   */
  UNIT_ID_NANOWATT = 50005,
  /*
   Microwatt (µW)
   */
  UNIT_ID_MICROWATT = 50006,
  /*
   Milliwatt (mW)
   */
  UNIT_ID_MILLIWATT = 50007,
  /*
   Deciwatt (dW)
   */
  UNIT_ID_DECIWATT = 50008,
  /*
   Watt (W)
   */
  UNIT_ID_WATT = 50009,
  /*
   Decawatt (daW)
   */
  UNIT_ID_DECAWATT = 50010,
  /*
   Hectowatt (hW)
   */
  UNIT_ID_HECTOWATT = 50011,
  /*
   Kilowatt (kW)
   */
  UNIT_ID_KILOWATT = 50012,
  /*
   Megawatt (MW)
   */
  UNIT_ID_MEGAWATT = 50013,
  /*
   Gigawatt (GW)
   */
  UNIT_ID_GIGAWATT = 50014,
  /*
   Terawatt (TW)
   */
  UNIT_ID_TERAWATT = 50015,
  /*
   Petawatt (PW)
   */
  UNIT_ID_PETAWATT = 50016,
  /*
   Exawatt (EW)
   */
  UNIT_ID_EXAWATT = 50017,
  /*
   Zettawatt (ZW)
   */
  UNIT_ID_ZETTAWATT = 50018,
  /*
   Yottawatt (YW)
   */
  UNIT_ID_YOTTAWATT = 50019,
  /*
   ErgPerSecond (erg/s)
   */
  UNIT_ID_ERG_PER_SECOND = 51000,
  /*
   HorsepowerMetric (PS)
   */
  UNIT_ID_HORSEPOWER_METRIC = 51001,
  /*
   HorsepowerElectric (hp_e)
   */
  UNIT_ID_HORSEPOWER_ELECTRIC = 51002,
  /*
   SolarLuminosity (L_☉)
   */
  UNIT_ID_SOLAR_LUMINOSITY = 51003,
};
#ifndef __cplusplus
typedef uint32_t UnitId;
#endif // __cplusplus

/*
 A POD quantity carrier type suitable for FFI.

 This struct represents a physical quantity as a value paired with its unit.
 It is `#[repr(C)]` to ensure a stable, predictable memory layout across
 language boundaries.

 # Memory Layout

 - `value`: 8 bytes (f64)
 - `unit`: 4 bytes (u32 via UnitId)
 - Padding: 4 bytes (for alignment)
 - Total: 16 bytes on most platforms

 # Example

 ```rust
 use qtty_ffi::{QttyQuantity, UnitId};

 let q = QttyQuantity {
     value: 1000.0,
     unit: UnitId::Meter,
 };
 ```
 */
typedef struct qtty_quantity_t {
  /*
   The numeric value of the quantity.
   */
  double value;
  /*
   The unit identifier for this quantity.
   */
  UnitId unit;
} qtty_quantity_t;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/*
 Checks if a unit ID is valid (recognized by the registry).

 # Arguments

 * `unit` - The unit ID to validate

 # Returns

 `true` if the unit is valid, `false` otherwise.

 # Safety

 This function is safe to call from any context.
 */
bool qtty_unit_is_valid(UnitId unit);

/*
 Gets the dimension of a unit.

 # Arguments

 * `unit` - The unit ID to query
 * `out` - Pointer to store the dimension ID

 # Returns

 * `QTTY_OK` on success
 * `QTTY_ERR_NULL_OUT` if `out` is null
 * `QTTY_ERR_UNKNOWN_UNIT` if the unit is not recognized

 # Safety

 The caller must ensure that `out` points to valid, writable memory for a `DimensionId`,
 or is null (in which case an error is returned).
 */
int32_t qtty_unit_dimension(UnitId unit, DimensionId *out);

/*
 Checks if two units are compatible (same dimension).

 # Arguments

 * `a` - First unit ID
 * `b` - Second unit ID
 * `out` - Pointer to store the result

 # Returns

 * `QTTY_OK` on success
 * `QTTY_ERR_NULL_OUT` if `out` is null
 * `QTTY_ERR_UNKNOWN_UNIT` if either unit is not recognized

 # Safety

 The caller must ensure that `out` points to valid, writable memory for a `bool`,
 or is null (in which case an error is returned).
 */
int32_t qtty_units_compatible(UnitId a, UnitId b, bool *out);

/*
 Creates a new quantity with the given value and unit.

 # Arguments

 * `value` - The numeric value
 * `unit` - The unit ID
 * `out` - Pointer to store the resulting quantity

 # Returns

 * `QTTY_OK` on success
 * `QTTY_ERR_NULL_OUT` if `out` is null
 * `QTTY_ERR_UNKNOWN_UNIT` if the unit is not recognized

 # Safety

 The caller must ensure that `out` points to valid, writable memory for a `QttyQuantity`,
 or is null (in which case an error is returned).
 */
int32_t qtty_quantity_make(double value, UnitId unit, struct qtty_quantity_t *out);

/*
 Converts a quantity to a different unit.

 # Arguments

 * `src` - The source quantity
 * `dst_unit` - The target unit ID
 * `out` - Pointer to store the converted quantity

 # Returns

 * `QTTY_OK` on success
 * `QTTY_ERR_NULL_OUT` if `out` is null
 * `QTTY_ERR_UNKNOWN_UNIT` if either unit is not recognized
 * `QTTY_ERR_INCOMPATIBLE_DIM` if units have different dimensions

 # Safety

 The caller must ensure that `out` points to valid, writable memory for a `QttyQuantity`,
 or is null (in which case an error is returned).
 */
int32_t qtty_quantity_convert(struct qtty_quantity_t src,
                              UnitId dst_unit,
                              struct qtty_quantity_t *out);

/*
 Converts a value from one unit to another.

 This is a convenience function that operates on raw values instead of `QttyQuantity` structs.

 # Arguments

 * `value` - The numeric value to convert
 * `src_unit` - The source unit ID
 * `dst_unit` - The target unit ID
 * `out_value` - Pointer to store the converted value

 # Returns

 * `QTTY_OK` on success
 * `QTTY_ERR_NULL_OUT` if `out_value` is null
 * `QTTY_ERR_UNKNOWN_UNIT` if either unit is not recognized
 * `QTTY_ERR_INCOMPATIBLE_DIM` if units have different dimensions

 # Safety

 The caller must ensure that `out_value` points to valid, writable memory for an `f64`,
 or is null (in which case an error is returned).
 */
int32_t qtty_quantity_convert_value(double value,
                                    UnitId src_unit,
                                    UnitId dst_unit,
                                    double *out_value);

/*
 Gets the name of a unit as a NUL-terminated C string.

 # Arguments

 * `unit` - The unit ID to query

 # Returns

 A pointer to a static, NUL-terminated C string with the unit name,
 or a null pointer if the unit is not recognized.

 # Safety

 The returned pointer points to static memory and is valid for the lifetime
 of the program. The caller must not attempt to free or modify the returned string.
 */
const char *qtty_unit_name(UnitId unit);

/*
 Returns the FFI ABI version.

 This can be used by consumers to verify compatibility. The version is
 incremented when breaking changes are made to the ABI.

 Current version: 1
 */
uint32_t qtty_ffi_version(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* QTTY_FFI_H */

/* End of qtty_ffi.h */
