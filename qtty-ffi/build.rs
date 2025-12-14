use std::env;
use std::path::PathBuf;

fn main() {
    // Only generate header if this is the main build (not doc build, test, etc.)
    // and we're not in a docs.rs build environment
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = PathBuf::from(&crate_dir).join("include");

    // Create include directory if it doesn't exist
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!("cargo:warning=Failed to create include directory: {}", e);
        return;
    }

    let config_path = PathBuf::from(&crate_dir).join("cbindgen.toml");
    let config = match cbindgen::Config::from_file(&config_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("cargo:warning=Failed to read cbindgen.toml: {}", e);
            return;
        }
    };

    let header_path = out_dir.join("qtty_ffi.h");
    match cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(config)
        .generate()
    {
        Ok(bindings) => {
            bindings.write_to_file(&header_path);
            println!("cargo:rerun-if-changed=src/");
            println!("cargo:rerun-if-changed=cbindgen.toml");
            eprintln!("cargo:warning=Generated header at: {:?}", header_path);
        }
        Err(e) => {
            eprintln!("cargo:warning=Failed to generate C header: {}", e);
        }
    }
}
