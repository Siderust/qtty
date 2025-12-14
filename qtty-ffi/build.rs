use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{Attribute, Item, Meta, spanned::Spanned};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // Generate unit definitions from qtty-core
    generate_unit_definitions(&crate_dir, &out_dir);
    
    // Generate C header (existing functionality)
    generate_c_header(&crate_dir);
}

fn generate_unit_definitions(crate_dir: &str, out_dir: &str) {
    let qtty_core_path = PathBuf::from(crate_dir).join("../qtty-core/src/units");
    
    let mut units = Vec::new();
    
    // Parse each unit file
    for file_name in &["length.rs", "time.rs", "angular.rs", "mass.rs", "power.rs"] {
        let file_path = qtty_core_path.join(file_name);
        if file_path.exists() {
            println!("cargo:rerun-if-changed={}", file_path.display());
            parse_unit_file(&file_path, &mut units);
        }
    }
    
    // Assign discriminants based on dimension
    assign_discriminants(&mut units);
    
    // Generate code files
    generate_unit_enum(&units, out_dir);
    generate_unit_names(&units, out_dir);
    generate_unit_names_cstr(&units, out_dir);
    generate_from_u32(&units, out_dir);
    generate_registry(&units, out_dir);
    
    eprintln!("cargo:warning=Generated FFI bindings for {} units", units.len());
}

#[derive(Debug, Clone)]
struct UnitDef {
    name: String,
    symbol: String,
    dimension: String,
    ratio: String,
    discriminant: u32,
    line_number: usize,
}

fn parse_unit_file(path: &Path, units: &mut Vec<UnitDef>) {
    let content = fs::read_to_string(path).expect("Failed to read unit file");
    let syntax_tree = syn::parse_file(&content).expect("Failed to parse unit file");
    
    let mut file_units = Vec::new();
    
    // Extract macro invocations with line numbers
    extract_macro_units(&content, &mut file_units);
    
    // Extract struct definitions with line numbers
    for item in syntax_tree.items {
        match item {
            Item::Struct(item_struct) => {
                if let Some(unit_def) = extract_unit_def(&item_struct.attrs, &item_struct.ident.to_string(), &item_struct.ident) {
                    file_units.push(unit_def);
                }
            }
            Item::Mod(item_mod) => {
                // Handle nested modules (like nominal)
                if let Some((_, items)) = item_mod.content {
                    for nested_item in items {
                        if let Item::Struct(item_struct) = nested_item {
                            if let Some(mut unit_def) = extract_unit_def(&item_struct.attrs, &item_struct.ident.to_string(), &item_struct.ident) {
                                // Add Nominal prefix for units in submodule
                                if item_mod.ident == "nominal" {
                                    unit_def.name = format!("Nominal{}", unit_def.name);
                                }
                                file_units.push(unit_def);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    
    // Deduplicate by name (in case a unit appears in both macro and struct form)
    let mut seen = std::collections::HashSet::new();
    file_units.retain(|u| seen.insert(u.name.clone()));
    
    units.extend(file_units);
}

/// Extract units from macro invocations like `si_meter!(...)`, `si_gram!(...)`, etc.
fn extract_macro_units(content: &str, units: &mut Vec<UnitDef>) {
    // Map of macro names to their dimension
    let macro_dims = [
        ("si_meter!", "Length"),
        ("si_gram!", "Mass"),
        ("si_watt!", "Power"),
        ("si_second!", "Time"),
    ];
    
    for (line_no, line) in content.lines().enumerate() {
        let line_trimmed = line.trim();
        for (macro_name, dimension) in &macro_dims {
            if line_trimmed.starts_with(macro_name) {
                if let Some(mut parsed) = parse_si_macro_invocation(line_trimmed, dimension) {
                    parsed.line_number = line_no + 1;
                    units.push(parsed);
                }
            }
        }
    }
}

/// Parse a line like: `si_meter!(Kilometer, "km", 1000.0, Km, Kilometers, KM);`
/// Returns UnitDef with name=Kilometer, symbol="km", dimension=Length, ratio="1000.0"
fn parse_si_macro_invocation(line: &str, dimension: &str) -> Option<UnitDef> {
    // Extract content between parentheses
    let start = line.find('(')?;
    let end = line.rfind(')')?;
    let args = &line[start + 1..end];
    
    // Split by comma
    let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
    if parts.len() < 3 {
        return None;
    }
    
    let name = parts[0].to_string();
    let symbol = parts[1].trim_matches('"').to_string();
    let ratio = parts[2].to_string();
    
    Some(UnitDef {
        name,
        symbol,
        dimension: dimension.to_string(),
        ratio,
        discriminant: 0,
        line_number: 0,
    })
}

fn extract_unit_def<T: Spanned>(attrs: &[Attribute], struct_name: &str, spanned: &T) -> Option<UnitDef> {
    for attr in attrs {
        if attr.path().is_ident("unit") {
            let mut symbol = String::new();
            let mut dimension = String::new();
            let mut ratio = String::new();
            
            if let Meta::List(meta_list) = &attr.meta {
                let tokens_str = meta_list.tokens.to_string();
                // Simple parsing of attributes (symbol = "...", dimension = ..., ratio = ...)
                for part in tokens_str.split(',') {
                    let part = part.trim();
                    if let Some(rest) = part.strip_prefix("symbol =") {
                        symbol = rest.trim().trim_matches('"').to_string();
                    } else if let Some(rest) = part.strip_prefix("dimension =") {
                        dimension = rest.trim().to_string();
                    } else if let Some(rest) = part.strip_prefix("ratio =") {
                        ratio = rest.trim().to_string();
                    }
                }
            }
            
            if !symbol.is_empty() && !dimension.is_empty() && !ratio.is_empty() {
                // Map qtty-core dimension names to FFI dimension names
                let dimension = match dimension.as_str() {
                    "Angular" => "Angle",
                    d => d,
                }.to_string();
                
                return Some(UnitDef {
                    name: struct_name.to_string(),
                    symbol,
                    dimension,
                    ratio,
                    discriminant: 0,
                    line_number: 0, // Line number not available from syn Span
                });
            }
        }
    }
    None
}

fn assign_discriminants(units: &mut [UnitDef]) {
    let mut dim_counters: HashMap<String, u32> = HashMap::new();
    dim_counters.insert("Length".to_string(), 100);
    dim_counters.insert("Time".to_string(), 200);
    dim_counters.insert("Angle".to_string(), 300);
    dim_counters.insert("Mass".to_string(), 400);
    dim_counters.insert("Power".to_string(), 500);
    
    // Special handling for nominal units - start at 150
    for unit in units.iter_mut() {
        if unit.name.starts_with("Nominal") && unit.dimension == "Length" {
            let counter = dim_counters.entry("NominalLength".to_string()).or_insert(150);
            unit.discriminant = *counter;
            *counter += 1;
        }
    }
    
    // Assign regular units
    for unit in units.iter_mut() {
        if unit.discriminant == 0 {
            if let Some(counter) = dim_counters.get_mut(&unit.dimension) {
                unit.discriminant = *counter;
                *counter += 1;
            } else {
                eprintln!("cargo:warning=Unknown dimension: {} for unit {}", unit.dimension, unit.name);
            }
        }
    }
    
    // Debug output
    let mut counts: HashMap<String, usize> = HashMap::new();
    for unit in units.iter() {
        *counts.entry(unit.dimension.clone()).or_insert(0) += 1;
    }
    for (dim, count) in counts {
        eprintln!("cargo:warning=Dimension {}: {} units", dim, count);
    }
}

fn generate_unit_enum(units: &[UnitDef], out_dir: &str) {
    let mut code = String::from("// Auto-generated by build.rs from qtty-core unit definitions\n");
    code.push_str("/// Unit identifier for FFI.\n");
    code.push_str("///\n");
    code.push_str("/// Each variant corresponds to a specific unit supported by the FFI layer.\n");
    code.push_str("/// All discriminant values are explicitly assigned and are part of the ABI contract.\n");
    code.push_str("#[repr(u32)]\n");
    code.push_str("#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]\n");
    code.push_str("pub enum UnitId {\n");
    
    for unit in units {
        // Add documentation for each variant
        code.push_str(&format!("    /// {} (symbol: {})\n", unit.name, unit.symbol));
        code.push_str(&format!("    {} = {},\n", unit.name, unit.discriminant));
    }
    
    code.push_str("}\n");
    
    let dest_path = PathBuf::from(out_dir).join("unit_id_enum.rs");
    fs::write(&dest_path, code).expect("Failed to write unit_id_enum.rs");
}

fn generate_unit_names(units: &[UnitDef], out_dir: &str) {
    let mut code = String::from("// Auto-generated by build.rs\n");
    code.push_str("match self {\n");
    
    for unit in units {
        code.push_str(&format!("    UnitId::{} => \"{}\",\n", unit.name, unit.name));
    }
    
    code.push_str("}\n");
    
    let dest_path = PathBuf::from(out_dir).join("unit_names.rs");
    fs::write(&dest_path, code).expect("Failed to write unit_names.rs");
}

fn generate_unit_names_cstr(units: &[UnitDef], out_dir: &str) {
    let mut code = String::from("// Auto-generated by build.rs\n");
    code.push_str("match self {\n");
    
    for unit in units {
        code.push_str(&format!(
            "    UnitId::{} => b\"{}\\0\".as_ptr() as *const c_char,\n",
            unit.name, unit.name
        ));
    }
    
    code.push_str("}\n");
    
    let dest_path = PathBuf::from(out_dir).join("unit_names_cstr.rs");
    fs::write(&dest_path, code).expect("Failed to write unit_names_cstr.rs");
}

fn generate_from_u32(units: &[UnitDef], out_dir: &str) {
    let mut code = String::from("// Auto-generated by build.rs\n");
    code.push_str("match value {\n");
    
    for unit in units {
        code.push_str(&format!(
            "    {} => Some(UnitId::{}),\n",
            unit.discriminant, unit.name
        ));
    }
    
    code.push_str("    _ => None,\n}\n");
    
    let dest_path = PathBuf::from(out_dir).join("unit_from_u32.rs");
    fs::write(&dest_path, code).expect("Failed to write unit_from_u32.rs");
}

fn generate_registry(units: &[UnitDef], out_dir: &str) {
    let mut code = String::from("// Auto-generated by build.rs\n{\n");
    code.push_str("use core::f64::consts::PI;\n\n");
    code.push_str("// Precomputed constants\n");
    code.push_str("const METERS_PER_LIGHT_YEAR: f64 = 9_460_730_472_580_800.0;\n");
    code.push_str("const METERS_PER_PARSEC: f64 = 30_856_775_814_913_672.8;\n");
    code.push_str("const SECONDS_PER_DAY: f64 = 86_400.0;\n");
    code.push_str("const SECONDS_PER_YEAR: f64 = 31_556_952.0;\n");
    code.push_str("const SECONDS_PER_JULIAN_YEAR: f64 = 31_557_600.0;\n\n");
    
    code.push_str("match id {\n");
    
    for unit in units {
        let ratio_expr = convert_ratio_expression(&unit.ratio);
        code.push_str(&format!(
            "    UnitId::{} => Some(UnitMeta {{\n",
            unit.name
        ));
        code.push_str(&format!(
            "        dim: DimensionId::{},\n",
            unit.dimension
        ));
        code.push_str(&format!(
            "        scale_to_canonical: {},\n",
            ratio_expr
        ));
        code.push_str(&format!(
            "        name: \"{}\",\n",
            unit.name
        ));
        code.push_str("    }),\n");
    }
    
    code.push_str("}\n}\n");
    
    let dest_path = PathBuf::from(out_dir).join("unit_registry.rs");
    fs::write(&dest_path, code).expect("Failed to write unit_registry.rs");
}

fn convert_ratio_expression(ratio: &str) -> String {
    // Convert qtty-core ratio expressions to FFI canonical units
    // qtty-core uses Degree as canonical for Angular, but FFI uses Radian
    
    // Handle common patterns
    let ratio = ratio.replace("core::f64::consts::", "");
    
    // For Angular dimension, convert degree-based ratios to radian-based
    if ratio.contains("180.0") && ratio.contains("PI") {
        // This is already a radian conversion (e.g., "180.0 / PI" for radians)
        return format!("({})", ratio);
    }
    
    // Handle special constants
    let ratio = ratio
        .replace("METERS_PER_LIGHT_YEAR", "METERS_PER_LIGHT_YEAR")
        .replace("METERS_PER_PARSEC", "METERS_PER_PARSEC")
        .replace("SECONDS_PER_DAY", "SECONDS_PER_DAY")
        .replace("SECONDS_PER_YEAR", "SECONDS_PER_YEAR")
        .replace("SECONDS_PER_JULIAN_YEAR", "SECONDS_PER_JULIAN_YEAR");
    
    ratio
}

fn generate_c_header(crate_dir: &str) {
    if env::var("DOCS_RS").is_ok() {
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
