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
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Status codes
 */

/** Success status code. */
#define QTTY_OK 0

/** Error: the provided unit ID is not recognized/valid. */
#define QTTY_ERR_UNKNOWN_UNIT (-1)

/** Error: conversion requested between incompatible dimensions. */
#define QTTY_ERR_INCOMPATIBLE_DIM (-2)

/** Error: a required output pointer was null. */
#define QTTY_ERR_NULL_OUT (-3)

/** Error: the provided value is invalid (reserved for future use). */
#define QTTY_ERR_INVALID_VALUE (-4)

/**
 * Dimension identifier for FFI.
 *
 * Represents the physical dimension of a quantity. All discriminant values are
 * explicitly assigned and are part of the ABI contract.
 *
 * # ABI Contract
 *
 * **Discriminant values must never change.** New dimensions may be added with
 * new explicit discriminant values.
 */
typedef enum DimensionId {
    /** Length dimension (e.g., meters, kilometers). */
    DIMENSION_ID_LENGTH = 1,
    /** Time dimension (e.g., seconds, hours). */
    DIMENSION_ID_TIME = 2,
    /** Angle dimension (e.g., radians, degrees). */
    DIMENSION_ID_ANGLE = 3,
} DimensionId;

/**
 * Unit identifier for FFI.
 *
 * Each variant corresponds to a specific unit supported by the FFI layer.
 * All discriminant values are explicitly assigned and are part of the ABI contract.
 *
 * # ABI Contract
 *
 * **Discriminant values must never change.** New units may be added with
 * new explicit discriminant values. Units are grouped by dimension with
 * reserved ranges:
 *
 * - Length units: 100-199
 * - Time units: 200-299
 * - Angle units: 300-399
 *
 * This grouping is for organization only; the actual dimension is determined
 * by the registry, not by the discriminant range.
 */
typedef enum UnitId {
    /* Length units (100-199) */
    
    /** Meter (SI base unit for length). */
    UNIT_ID_METER = 100,
    /** Kilometer (1000 meters). */
    UNIT_ID_KILOMETER = 101,
    
    /* Time units (200-299) */
    
    /** Second (SI base unit for time). */
    UNIT_ID_SECOND = 200,
    /** Minute (60 seconds). */
    UNIT_ID_MINUTE = 201,
    /** Hour (3600 seconds). */
    UNIT_ID_HOUR = 202,
    /** Day (86400 seconds). */
    UNIT_ID_DAY = 203,
    
    /* Angle units (300-399) */
    
    /** Radian (SI unit for angles). */
    UNIT_ID_RADIAN = 300,
    /** Degree (π/180 radians). */
    UNIT_ID_DEGREE = 301,
} UnitId;

/**
 * A POD quantity carrier type suitable for FFI.
 *
 * This struct represents a physical quantity as a value paired with its unit.
 * It is `repr(C)` to ensure a stable, predictable memory layout across
 * language boundaries.
 *
 * # Memory Layout
 *
 * - `value`: 8 bytes (f64/double)
 * - `unit`: 4 bytes (u32 via UnitId)
 * - Padding: 4 bytes (for alignment)
 * - Total: 16 bytes on most platforms
 */
typedef struct qtty_quantity_t {
    /** The numeric value of the quantity. */
    double value;
    /** The unit identifier for this quantity. */
    UnitId unit;
} qtty_quantity_t;

/**
 * Returns the FFI ABI version.
 *
 * This can be used by consumers to verify compatibility. The version is
 * incremented when breaking changes are made to the ABI.
 *
 * Current version: 1
 */
uint32_t qtty_ffi_version(void);

/**
 * Checks if a unit ID is valid (recognized by the registry).
 *
 * @param unit The unit ID to validate
 * @return true if the unit is valid, false otherwise.
 */
bool qtty_unit_is_valid(UnitId unit);

/**
 * Gets the dimension of a unit.
 *
 * @param unit The unit ID to query
 * @param out Pointer to store the dimension ID
 * @return QTTY_OK on success, QTTY_ERR_NULL_OUT if out is null,
 *         QTTY_ERR_UNKNOWN_UNIT if the unit is not recognized
 */
int32_t qtty_unit_dimension(UnitId unit, DimensionId *out);

/**
 * Checks if two units are compatible (same dimension).
 *
 * @param a First unit ID
 * @param b Second unit ID
 * @param out Pointer to store the result
 * @return QTTY_OK on success, QTTY_ERR_NULL_OUT if out is null,
 *         QTTY_ERR_UNKNOWN_UNIT if either unit is not recognized
 */
int32_t qtty_units_compatible(UnitId a, UnitId b, bool *out);

/**
 * Gets the name of a unit as a NUL-terminated C string.
 *
 * @param unit The unit ID to query
 * @return A pointer to a static, NUL-terminated C string with the unit name,
 *         or a null pointer if the unit is not recognized.
 *
 * @note The returned pointer points to static memory and is valid for the lifetime
 *       of the program. The caller must not attempt to free or modify the returned string.
 */
const char *qtty_unit_name(UnitId unit);

/**
 * Creates a new quantity with the given value and unit.
 *
 * @param value The numeric value
 * @param unit The unit ID
 * @param out Pointer to store the resulting quantity
 * @return QTTY_OK on success, QTTY_ERR_NULL_OUT if out is null,
 *         QTTY_ERR_UNKNOWN_UNIT if the unit is not recognized
 */
int32_t qtty_quantity_make(double value, UnitId unit, qtty_quantity_t *out);

/**
 * Converts a quantity to a different unit.
 *
 * @param src The source quantity
 * @param dst_unit The target unit ID
 * @param out Pointer to store the converted quantity
 * @return QTTY_OK on success, QTTY_ERR_NULL_OUT if out is null,
 *         QTTY_ERR_UNKNOWN_UNIT if either unit is not recognized,
 *         QTTY_ERR_INCOMPATIBLE_DIM if units have different dimensions
 */
int32_t qtty_quantity_convert(qtty_quantity_t src, UnitId dst_unit, qtty_quantity_t *out);

/**
 * Converts a value from one unit to another.
 *
 * This is a convenience function that operates on raw values instead of
 * qtty_quantity_t structs.
 *
 * @param value The numeric value to convert
 * @param src_unit The source unit ID
 * @param dst_unit The target unit ID
 * @param out_value Pointer to store the converted value
 * @return QTTY_OK on success, QTTY_ERR_NULL_OUT if out_value is null,
 *         QTTY_ERR_UNKNOWN_UNIT if either unit is not recognized,
 *         QTTY_ERR_INCOMPATIBLE_DIM if units have different dimensions
 */
int32_t qtty_quantity_convert_value(double value, UnitId src_unit, UnitId dst_unit, double *out_value);

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* QTTY_FFI_H */
