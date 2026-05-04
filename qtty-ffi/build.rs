// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

//! Build script for qtty-ffi.
//!
//! Generates FFI bindings by combining:
//! 1. Stable discriminant values from `discriminants.csv` (ABI contract)
//! 2. Unit metadata (ratio, symbol, type path) from `qtty`'s `Unit` trait
//!
//! This ensures zero drift between the FFI layer and the canonical Rust types:
//! all metadata is derived from the published crate itself at compile time.

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    // Re-run triggers
    println!("cargo:rerun-if-changed=discriminants.csv");

    // Step 1: Parse discriminant mapping (ABI contract)
    let disc_map = parse_discriminants(&crate_dir);

    // Step 2: Resolve discriminants to qtty::unit type paths.
    let resolved = resolve_units(&disc_map);

    eprintln!(
        "cargo:warning=Resolved {} FFI units from discriminants.csv",
        resolved.len()
    );

    // Step 4: Generate all code files
    generate_unit_enum(&resolved, &out_dir);
    generate_unit_names(&resolved, &out_dir);
    generate_unit_names_cstr(&resolved, &out_dir);
    generate_unit_symbols(&resolved, &out_dir);
    generate_from_u32(&resolved, &out_dir);
    generate_registry(&resolved, &out_dir);
    generate_unit_conversions(&resolved, &out_dir);
    generate_unit_constants(&resolved, &out_dir);

    // Step 5: Generate C header
    generate_c_header(&crate_dir);
}

// =============================================================================
// Data structures
// =============================================================================

/// A fully resolved unit ready for code generation.
#[derive(Debug, Clone)]
struct ResolvedUnit {
    /// UnitId variant name (e.g., "Meter", "NominalSolarRadius")
    ffi_name: String,
    /// Stable ABI discriminant value
    discriminant: u32,
    /// DimensionId variant name (e.g., "Length", "Time")
    dimension: String,
    /// Fully qualified Rust type path (e.g., "qtty::length::Meter")
    rust_type_path: String,
}

// =============================================================================
// Step 1: Parse discriminants.csv
// =============================================================================

fn parse_discriminants(crate_dir: &str) -> HashMap<String, u32> {
    let path = PathBuf::from(crate_dir).join("discriminants.csv");
    let content = fs::read_to_string(&path).expect("Failed to read discriminants.csv");

    let mut map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.splitn(2, ',').collect();
        if parts.len() != 2 {
            panic!("Malformed discriminants.csv line: {line}");
        }
        let discriminant: u32 = parts[0]
            .trim()
            .parse()
            .unwrap_or_else(|_| panic!("Invalid discriminant: {}", parts[0]));
        let name = parts[1].trim().to_string();
        if map.insert(name.clone(), discriminant).is_some() {
            panic!("Duplicate FFI name in discriminants.csv: {name}");
        }
    }
    map
}

// =============================================================================
// Step 2: Inventory configuration
// =============================================================================

fn resolve_units(disc_map: &HashMap<String, u32>) -> Vec<ResolvedUnit> {
    let mut resolved: Vec<_> = disc_map
        .iter()
        .map(|(ffi_name, &discriminant)| ResolvedUnit {
            ffi_name: ffi_name.clone(),
            discriminant,
            dimension: dimension_from_discriminant(discriminant).to_string(),
            rust_type_path: rust_type_path_from_ffi_name(ffi_name),
        })
        .collect();

    resolved.sort_by_key(|unit| unit.discriminant);
    resolved
}

fn dimension_from_discriminant(discriminant: u32) -> &'static str {
    match discriminant {
        10_000..=19_999 => "Length",
        20_000..=29_999 => "Time",
        30_000..=39_999 => "Angle",
        40_000..=49_999 => "Mass",
        50_000..=59_999 => "Power",
        60_000..=69_999 => "Area",
        70_000..=79_999 => "Volume",
        80_000..=89_999 => "Acceleration",
        90_000..=99_999 => "Force",
        100_000..=109_999 => "Energy",
        110_000..=119_999 => "Pressure",
        120_000..=129_999 => "SolidAngle",
        130_000..=139_999 => "Temperature",
        140_000..=149_999 => "Radiance",
        150_000..=159_999 => "SpectralRadiance",
        160_000..=169_999 => "PhotonRadiance",
        170_000..=179_999 => "SpectralPhotonRadiance",
        180_000..=189_999 => "InverseSolidAngle",
        190_000..=199_999 => "LuminousIntensity",
        200_000..=209_999 => "LuminousFlux",
        210_000..=219_999 => "Illuminance",
        220_000..=229_999 => "Frequency",
        230_000..=239_999 => "AmountOfSubstance",
        240_000..=249_999 => "Current",
        250_000..=259_999 => "Charge",
        260_000..=269_999 => "Voltage",
        270_000..=279_999 => "Resistance",
        280_000..=289_999 => "Capacitance",
        290_000..=299_999 => "Inductance",
        300_000..=309_999 => "MagneticFlux",
        310_000..=319_999 => "MagneticFluxDensity",
        320_000..=329_999 => "Density",
        _ => panic!("Unknown discriminant range for {discriminant}"),
    }
}

fn rust_type_path_from_ffi_name(ffi_name: &str) -> String {
    let rust_type_name = ffi_name.strip_prefix("Nominal").unwrap_or(ffi_name);
    format!("qtty::unit::{rust_type_name}")
}

// =============================================================================
// Code generation
// =============================================================================

fn generate_unit_enum(units: &[ResolvedUnit], out_dir: &str) {
    let mut code = String::from(
        "// Auto-generated by build.rs from discriminants.csv + qtty::unit type paths.\n\
         // Do not edit manually.\n\n",
    );
    code.push_str("/// Unit identifier for FFI.\n");
    code.push_str("///\n");
    code.push_str("/// Each variant corresponds to a specific unit supported by the FFI layer.\n");
    code.push_str(
        "/// All discriminant values are explicitly assigned and are part of the ABI contract.\n",
    );
    code.push_str("#[repr(u32)]\n");
    code.push_str("#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]\n");
    code.push_str(
        "#[cfg_attr(feature = \"pyo3\", pyo3::pyclass(eq, eq_int, from_py_object, module = \"qtty\"))]\n",
    );
    code.push_str("pub enum UnitId {\n");

    for unit in units {
        code.push_str(&format!(
            "    /// {} ({} dimension)\n",
            unit.ffi_name, unit.dimension
        ));
        code.push_str(&format!("    {} = {},\n", unit.ffi_name, unit.discriminant));
    }

    code.push_str("}\n\n");

    // PyO3 methods
    code.push_str("#[cfg(feature = \"pyo3\")]\n");
    code.push_str("#[pyo3::pymethods]\n");
    code.push_str("impl UnitId {\n");
    code.push_str("    #[new]\n");
    code.push_str("    fn __new__(value: u32) -> pyo3::PyResult<Self> {\n");
    code.push_str("        Self::from_u32(value).ok_or_else(|| {\n");
    code.push_str("            pyo3::exceptions::PyValueError::new_err(format!(\"Invalid UnitId: {}\", value))\n");
    code.push_str("        })\n");
    code.push_str("    }\n\n");
    code.push_str("    fn __getnewargs__(&self) -> (u32,) {\n");
    code.push_str("        (*self as u32,)\n");
    code.push_str("    }\n\n");
    code.push_str("    fn __hash__(&self) -> u64 {\n");
    code.push_str("        *self as u64\n");
    code.push_str("    }\n\n");
    code.push_str("    fn __repr__(&self) -> String {\n");
    code.push_str("        format!(\"Unit.{}\", self.name())\n");
    code.push_str("    }\n\n");
    code.push_str("    /// Multiply a scalar by a unit to create a Quantity.\n");
    code.push_str("    fn __mul__<'py>(&self, py: pyo3::Python<'py>, scalar: f64) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {\n");
    code.push_str("        use pyo3::types::PyAnyMethods;\n");
    code.push_str("        let qtty = py.import(\"qtty\")?;\n");
    code.push_str("        let cls = qtty.getattr(\"Quantity\")?;\n");
    code.push_str("        cls.call1((scalar, *self))\n");
    code.push_str("    }\n\n");
    code.push_str(
        "    /// Right multiplication: `Unit.Second * 9.58` → `Quantity(9.58, Unit.Second)`\n",
    );
    code.push_str("    fn __rmul__<'py>(&self, py: pyo3::Python<'py>, scalar: f64) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {\n");
    code.push_str("        self.__mul__(py, scalar)\n");
    code.push_str("    }\n");
    code.push_str("}\n");

    let dest_path = PathBuf::from(out_dir).join("unit_id_enum.rs");
    fs::write(&dest_path, code).expect("Failed to write unit_id_enum.rs");
}

fn generate_unit_names(units: &[ResolvedUnit], out_dir: &str) {
    let mut code = String::from(
        "// Auto-generated — unit names derived from UnitId variant names.\n\
         match self {\n",
    );
    for unit in units {
        code.push_str(&format!(
            "    UnitId::{} => \"{}\",\n",
            unit.ffi_name, unit.ffi_name
        ));
    }
    code.push_str("}\n");

    let dest_path = PathBuf::from(out_dir).join("unit_names.rs");
    fs::write(&dest_path, code).expect("Failed to write unit_names.rs");
}

fn generate_unit_names_cstr(units: &[ResolvedUnit], out_dir: &str) {
    let mut code = String::from(
        "// Auto-generated — unit names as C strings.\n\
         match self {\n",
    );
    for unit in units {
        code.push_str(&format!(
            "    UnitId::{} => c\"{}\".as_ptr(),\n",
            unit.ffi_name, unit.ffi_name
        ));
    }
    code.push_str("}\n");

    let dest_path = PathBuf::from(out_dir).join("unit_names_cstr.rs");
    fs::write(&dest_path, code).expect("Failed to write unit_names_cstr.rs");
}

fn generate_unit_symbols(units: &[ResolvedUnit], out_dir: &str) {
    // Symbols are derived from qtty-core at compile time via the Unit trait.
    let mut code = String::from(
        "// Auto-generated — symbols derived from <Type as qtty::Unit>::SYMBOL.\n\
         match self {\n",
    );
    for unit in units {
        code.push_str(&format!(
            "    UnitId::{} => <{} as qtty::Unit>::SYMBOL,\n",
            unit.ffi_name, unit.rust_type_path
        ));
    }
    code.push_str("}\n");

    let dest_path = PathBuf::from(out_dir).join("unit_symbols.rs");
    fs::write(&dest_path, code).expect("Failed to write unit_symbols.rs");
}

fn generate_from_u32(units: &[ResolvedUnit], out_dir: &str) {
    let mut code = String::from(
        "// Auto-generated — discriminant validation.\n\
         match value {\n",
    );
    for unit in units {
        code.push_str(&format!(
            "    {} => Some(UnitId::{}),\n",
            unit.discriminant, unit.ffi_name
        ));
    }
    code.push_str("    _ => None,\n}\n");

    let dest_path = PathBuf::from(out_dir).join("unit_from_u32.rs");
    fs::write(&dest_path, code).expect("Failed to write unit_from_u32.rs");
}

fn generate_registry(units: &[ResolvedUnit], out_dir: &str) {
    // Registry metadata is derived from qtty-core's Unit trait at compile time.
    let mut code = String::from(
        "// Auto-generated — metadata derived from <Type as qtty::Unit>::RATIO.\n\
         match id {\n",
    );
    for unit in units {
        code.push_str(&format!(
            "    UnitId::{} => Some(UnitMeta {{\n",
            unit.ffi_name
        ));
        code.push_str(&format!("        dim: DimensionId::{},\n", unit.dimension));
        code.push_str(&format!(
            "        scale_to_canonical: <{} as qtty::Unit>::RATIO,\n",
            unit.rust_type_path
        ));
        code.push_str(&format!("        name: \"{}\",\n", unit.ffi_name));
        code.push_str("    }),\n");
    }
    code.push_str("}\n");

    let dest_path = PathBuf::from(out_dir).join("unit_registry.rs");
    fs::write(&dest_path, code).expect("Failed to write unit_registry.rs");
}

fn generate_unit_conversions(units: &[ResolvedUnit], out_dir: &str) {
    let mut code = String::from(
        "// Auto-generated From/TryFrom impls.\n\
         // Each `impl_unit_ffi!` invocation generates:\n\
         //   impl From<RustQuantityType> for QttyQuantity\n\
         //   impl TryFrom<QttyQuantity> for RustQuantityType\n\n",
    );

    // For the conversion impls, we need the Quantity type alias (plural form),
    // which is `Quantity<UnitType>`. We use the qtty re-exported type path.
    // The impl_unit_ffi! macro calls .value() and ::new(), which are methods on
    // Quantity<U>, so we pass the Quantity type alias.
    //
    // qtty re-exports e.g. qtty::length::Meters = Quantity<Meter>
    // The plural form follows the pattern: unit name + "s" for most units.
    // However, we use the generic form: qtty::Quantity<unit::Type>
    for unit in units {
        // Use Quantity<UnitType> which works universally
        code.push_str(&format!(
            "crate::impl_unit_ffi!(qtty::Quantity<{}>, crate::UnitId::{});\n",
            unit.rust_type_path, unit.ffi_name
        ));
    }

    let dest_path = PathBuf::from(out_dir).join("unit_conversions.rs");
    fs::write(&dest_path, code).expect("Failed to write unit_conversions.rs");
}

fn generate_unit_constants(units: &[ResolvedUnit], out_dir: &str) {
    let mut code = String::from(
        "// Auto-generated QTTY_UNIT_* constants.\n\
         // Each constant is the raw u32 discriminant of the corresponding UnitId variant.\n\n",
    );

    for unit in units {
        let screaming = to_screaming_snake(&unit.ffi_name);
        code.push_str(&format!(
            "/// Raw unit ID constant for {} ({}).\n",
            unit.ffi_name, unit.dimension
        ));
        code.push_str(&format!(
            "pub const QTTY_UNIT_{}: u32 = UnitId::{} as u32;\n",
            screaming, unit.ffi_name
        ));
    }

    let dest_path = PathBuf::from(out_dir).join("unit_constants.rs");
    fs::write(&dest_path, code).expect("Failed to write unit_constants.rs");
}

/// Convert a PascalCase name to SCREAMING_SNAKE_CASE.
fn to_screaming_snake(name: &str) -> String {
    let mut result = String::with_capacity(name.len() + 4);
    for (i, ch) in name.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            let prev = name.as_bytes()[i - 1] as char;
            let next_is_lower = name.chars().nth(i + 1).is_some_and(|c| c.is_lowercase());
            if prev.is_lowercase() || next_is_lower {
                result.push('_');
            }
        }
        result.push(ch.to_ascii_uppercase());
    }
    result
}

// =============================================================================
// C header generation (unchanged)
// =============================================================================

fn generate_c_header(crate_dir: &str) {
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let is_nightly = std::process::Command::new(&rustc)
        .arg("--version")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("nightly"))
        .unwrap_or(false);

    if !is_nightly {
        eprintln!(
            "cargo:warning=Skipping cbindgen header regeneration (stable toolchain); \
             header maintained manually."
        );
        println!("cargo:rerun-if-changed=src/");
        println!("cargo:rerun-if-changed=cbindgen.toml");
        return;
    }

    let out_dir = PathBuf::from(crate_dir).join("include");

    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!("cargo:warning=Failed to create include directory: {}", e);
        return;
    }

    let config_path = PathBuf::from(crate_dir).join("cbindgen.toml");
    let config = match cbindgen::Config::from_file(&config_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("cargo:warning=Failed to read cbindgen.toml: {}", e);
            return;
        }
    };

    let header_path = out_dir.join("qtty_ffi.h");
    match cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
    {
        Ok(bindings) => {
            bindings.write_to_file(&header_path);
            println!("cargo:rerun-if-changed=src/");
            println!("cargo:rerun-if-changed=cbindgen.toml");
        }
        Err(e) => {
            eprintln!("cargo:warning=Failed to generate C header: {}", e);
        }
    }
}
