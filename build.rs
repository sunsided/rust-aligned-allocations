fn main() {
    #[cfg(feature = "ffi")]
    {
        build_ffi_wrapper();
    }
}

/// Find the location of the `target/` directory. Note that this may be
/// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR`
/// variable.
fn target_dir() -> std::path::PathBuf {
    if let Ok(target) = std::env::var("OUT_DIR") {
        std::path::PathBuf::from(target)
    } else {
        std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}

#[cfg(feature = "ffi")]
fn build_ffi_wrapper() {
    use cbindgen::Config;
    use std::env;

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let output_file = target_dir()
        .join(format!("{}.hpp", package_name))
        .display()
        .to_string();

    let config = Config {
        namespace: Some(String::from("ffi")),
        ..Default::default()
    };

    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file(&output_file);
}
