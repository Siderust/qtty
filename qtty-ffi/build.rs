use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // Re-run if units.csv changes
    println!("cargo:rerun-if-changed=units.csv");
    
    // Parse units from CSV
    let units = parse_units_csv(&crate_dir);
    
    // Generate code files
    generate_unit_enum(&units, &out_dir);
    generate_unit_names(&units, &out_dir);
    generate_unit_names_cstr(&units, &out_dir);
    generate_from_u32(&units, &out_dir);
    generate_registry(&units, &out_dir);
    
    eprintln!("cargo:warning=Generated FFI bindings for {} units from units.csv", units.len());
    
    // Generate C header (existing functionality)
    generate_c_header(&crate_dir);
}

#[derive(Debug, Clone)]
struct UnitDef {
    name: String,
    symbol: String,
    dimension: String,
    discriminant: u32,
    ratio: String,
}

fn parse_units_csv(crate_dir: &str) -> Vec<UnitDef> {
    let csv_path = PathBuf::from(crate_dir).join("units.csv");
    let content = fs::read_to_string(&csv_path)
        .expect("Failed to read units.csv");
    
    let mut units = Vec::new();
    
    for line in content.lines() {
        let line = line.trim();
        
        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 5 {
            eprintln!("cargo:warning=Skipping invalid line: {}", line);
            continue;
        }
        
        units.push(UnitDef {
            discriminant: parts[0].parse().expect(&format!("Invalid discriminant: {}", parts[0])),
            dimension: parts[1].to_string(),
            name: parts[2].to_string(),
            symbol: parts[3].to_string(),
            ratio: parts[4].to_string(),
        });
    }
    
    units
}

fn generate_unit_enum(units: &[UnitDef], out_dir: &str) {
    let mut code = String::from("// Auto-generated from units.csv\n");
    code.push_str("/// Unit identifier for FFI.\n");
    code.push_str("///\n");
    code.push_str("/// Each variant corresponds to a specific unit supported by the FFI layer.\n");
    code.push_str("/// All discriminant values are explicitly assigned and are part of the ABI contract.\n");
    code.push_str("#[repr(u32)]\n");
    code.push_str("#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]\n");
    code.push_str("pub enum UnitId {\n");
    
    for unit in units {
        code.push_str(&format!("    /// {} ({})\n", unit.name, unit.symbol));
        code.push_str(&format!("    {} = {},\n", unit.name, unit.discriminant));
    }
    
    code.push_str("}\n");
    
    let dest_path = PathBuf::from(out_dir).join("unit_id_enum.rs");
    fs::write(&dest_path, code).expect("Failed to write unit_id_enum.rs");
}

fn generate_unit_names(units: &[UnitDef], out_dir: &str) {
    let mut code = String::from("// Auto-generated from units.csv\n");
    code.push_str("match self {\n");
    
    for unit in units {
        code.push_str(&format!("    UnitId::{} => \"{}\",\n", unit.name, unit.name));
    }
    
    code.push_str("}\n");
    
    let dest_path = PathBuf::from(out_dir).join("unit_names.rs");
    fs::write(&dest_path, code).expect("Failed to write unit_names.rs");
}

fn generate_unit_names_cstr(units: &[UnitDef], out_dir: &str) {
    let mut code = String::from("// Auto-generated from units.csv\n");
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
    let mut code = String::from("// Auto-generated from units.csv\n");
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
    let mut code = String::from("// Auto-generated from units.csv\n");
    code.push_str("match id {\n");
    
    for unit in units {
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
            unit.ratio
        ));
        code.push_str(&format!(
            "        name: \"{}\",\n",
            unit.name
        ));
        code.push_str("    }),\n");
    }
    
    code.push_str("}\n");
    
    let dest_path = PathBuf::from(out_dir).join("unit_registry.rs");
    fs::write(&dest_path, code).expect("Failed to write unit_registry.rs");
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
