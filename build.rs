fn main() {
    #[cfg(feature = "ffi")]
    {
        build_ffi_wrapper();
    }
}

#[cfg(feature = "ffi")]
fn build_ffi_wrapper() {
    use cbindgen::Config;
    use std::env;
    use std::path::PathBuf;

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let output_file = target_dir()
        .join(format!("{}.hpp", package_name))
        .display()
        .to_string();

    let mut config = Config::default();
    config.namespace = Some(String::from("ffi"));

    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file(&output_file);

    /// Find the location of the `target/` directory. Note that this may be
    /// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR`
    /// variable.
    fn target_dir() -> PathBuf {
        if let Ok(target) = env::var("OUT_DIR") {
            PathBuf::from(target)
        } else {
            PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
        }
    }
}
