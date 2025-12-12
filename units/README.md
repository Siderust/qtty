# unit

Strongly typed physical and astronomical units for Rust.

## Features

- **Zero-cost abstractions**: All type checking happens at compile time with no runtime overhead
- **Type safety**: Catch unit mismatches at compile time
- **Comprehensive units**: Time, angular, length, velocity, mass, power, and more
- **Easy conversions**: Seamless conversion between compatible units
- **Arithmetic operations**: Natural mathematical operations between quantities
- **no_std support**: Works in embedded environments

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
unit = "0.1.0"
```

## Example

```rust
use unit::*;

// Create quantities with units
let distance = Kilometers::new(100.0);
let time = Hours::new(2.0);

// Perform calculations
let speed: KilometersPerHour = distance / time;
assert_eq!(speed.value(), 50.0);

// Convert between units
let distance_m: Meters = distance.to();
assert_eq!(distance_m.value(), 100_000.0);

// Angular units
let degrees = Degrees::new(180.0);
let radians: Radians = degrees.to();
assert!((radians.value() - std::f64::consts::PI).abs() < 1e-10);
```

## Available Units

- **Time**: Seconds, Minutes, Hours, Days, Years, Centuries
- **Angular**: Degrees, Radians, Arcseconds, DMS (Degrees-Minutes-Seconds), HMS (Hour-Minutes-Seconds)
- **Length**: Meters, Kilometers, Astronomical Units, Light Years
- **Velocity**: Meters/Second, Kilometers/Second, AU/Day
- **Mass**: Kilograms, Solar Masses
- **Power**: Watts, Solar Luminosities
- **Frequency**: Hertz

## License

This project is licensed under the AGPL-3.0 License.
