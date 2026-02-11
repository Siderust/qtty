use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn fixture_manifest(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
        .join("Cargo.toml")
}

fn cargo_check(manifest_path: &Path) -> Output {
    let target_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("qtty-vec-feature-matrix");

    Command::new(env!("CARGO"))
        .arg("check")
        .arg("--manifest-path")
        .arg(manifest_path)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("CARGO_TERM_COLOR", "never")
        .env("CARGO_TARGET_DIR", target_dir)
        .output()
        .expect("failed to run cargo check")
}

#[test]
fn qtty_vec_feature_matrix() {
    let std_manifest = fixture_manifest("qtty-vec-std");
    let std_out = cargo_check(&std_manifest);
    assert!(
        std_out.status.success(),
        "std mode should compile\nstderr:\n{}",
        String::from_utf8_lossy(&std_out.stderr)
    );

    let no_std_alloc_manifest = fixture_manifest("qtty-vec-no-std-alloc");
    let no_std_alloc_out = cargo_check(&no_std_alloc_manifest);
    assert!(
        no_std_alloc_out.status.success(),
        "no_std + alloc mode should compile\nstderr:\n{}",
        String::from_utf8_lossy(&no_std_alloc_out.stderr)
    );

    let no_std_manifest = fixture_manifest("qtty-vec-no-std");
    let no_std_out = cargo_check(&no_std_manifest);
    assert!(
        !no_std_out.status.success(),
        "pure no_std mode should fail for vec form"
    );

    let stderr = String::from_utf8_lossy(&no_std_out.stderr);
    assert!(
        stderr.contains("requires the `std` or `alloc` feature"),
        "expected a clear feature requirement in compiler output\nstderr:\n{stderr}"
    );
}
