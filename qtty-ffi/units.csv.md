# FFI Unit Definitions

This file defines all units exposed through the qtty-ffi C API.

## Format

```
discriminant,dimension,name,symbol,ratio
```

### Discriminant Encoding

Discriminants use a **DSSCC** format:
- **D** (1 digit): Dimension (1=Length, 2=Time, 3=Angle, 4=Mass, 5=Power)
- **SS** (2 digits): System/Category code (00, 10, 20, 30, etc.)
- **CC** (2 digits): Counter within that system (00-99)

**Example:** `21000` = Minute (Dimension=2 [Time], System=10 [Common], Counter=00)

### Fields

- **discriminant**: Unique u32 ID (ABI-stable, NEVER change once assigned) - sorted for easy lookup
- **dimension**: One of: `Length`, `Time`, `Angle`, `Mass`, `Power` (groups units by type)
- **name**: Rust identifier (PascalCase, no spaces)
- **symbol**: Display symbol (can include Unicode)
- **ratio**: Conversion factor to canonical unit (exact or best-available)

### Canonical Units

Each dimension has a canonical unit (ratio = 1.0):

- **Length**: Meter (discriminant 10011)
- **Time**: Second (discriminant 20008)  
- **Angle**: Radian (discriminant 30001)
- **Mass**: Gram (discriminant 40010)
- **Power**: Watt (discriminant 50009)

### Discriminant Ranges

Units are grouped by dimension and system:

**Length (1xxxx):**
- 100xx: SI metric (Meter, Kilometer, etc.)
- 110xx: Astronomical (AU, LightYear, Parsec, etc.)
- 120xx: Imperial (Inch, Foot, Mile, etc.)
- 130xx: Nautical (NauticalMile, Fathom, etc.)
- 150xx: Nominal (SolarRadius, EarthRadius, etc.)

**Time (2xxxx):**
- 200xx: SI metric (Second, Millisecond, etc.)
- 210xx: Common (Minute, Hour, Day, Week, etc.)
- 220xx: Calendar (Year, Decade, Century, etc.)
- 230xx: Astronomical (SiderealDay, SiderealYear, etc.)

**Angle (3xxxx):**
- 300xx: Radian-based (Radian, Milliradian)
- 310xx: Degree-based (Degree, Arcminute, Arcsecond, etc.)
- 320xx: Other (Gradian, Turn, HourAngle)

**Mass (4xxxx):**
- 400xx: SI metric (Gram, Kilogram, etc.)
- 410xx: Imperial (Pound, Ounce, Stone, Ton, etc.)
- 420xx: Special (Carat, AtomicMassUnit, SolarMass, etc.)

**Power (5xxxx):**
- 500xx: SI metric (Watt, Kilowatt, etc.)
- 510xx: Other (Horsepower, SolarLuminosity, etc.)

### ABI Stability

**CRITICAL**: Once a discriminant is assigned and released, it MUST NEVER CHANGE.
This is a C ABI contract. Changing discriminants breaks binary compatibility.

When adding new units:
1. Choose an unused discriminant in the appropriate range
2. Add a new line to this file
3. Commit and release

## Adding a New Unit

Example: Adding Angstrom (10^-10 meters) to SI Length units

```csv
10022,Length,Angstrom,Å,1e-10
```

The discriminant breaks down as:
- `1` = Length dimension
- `00` = SI system  
- `22` = Counter (next available after Yottameter at 21)

Then rebuild: `cargo build`

The build script will automatically generate all necessary Rust code.

## Benefits of CSV Approach

✅ **Simple**: No AST parsing, just read CSV  
✅ **Explicit**: All units visible in one file  
✅ **Reviewable**: Git diffs show exactly what changed  
✅ **Maintainable**: Easy to add/modify units  
✅ **Debuggable**: Parse errors point to specific lines  
✅ **Independent**: FFI doesn't depend on qtty-core internals  

## Validation

The build script validates:
- Each line has exactly 5 fields
- Discriminants are valid u32 integers
- Dimension names match known dimensions

Invalid lines are skipped with a warning during build.
