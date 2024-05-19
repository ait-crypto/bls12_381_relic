use std::{env, path::PathBuf};

#[cfg(feature = "vendored")]
fn build() {
    panic!("vendored not yet implemented")
}

fn find_lib() {
    #[cfg(feature = "system")]
    {
        // Try to find shared library via pkg-config
        if pkg_config::Config::new().probe("relic").is_ok() {
            return;
        }
    }

    #[cfg(feature = "vendored")]
    // Download and build static library
    build();
    #[cfg(not(feature = "vendored"))]
    panic!("Unable to find library with pkg-config and vendored is not enabled!");
}

fn main() {
    find_lib();

    // Invalidate the built crate whenever the wrapper and the build script changes.
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=wrapper.c");
    println!("cargo:rerun-if-changed=build.rs");

    let mut build = cc::Build::new();
    build.static_flag(true);
    build.flag_if_supported("-std=gnu11");
    build.flag_if_supported("-fstack-protector-strong");
    build.define("_FORTIFY_SOURCE", Some("2"));
    build.files(["wrapper.c"].iter()).compile("relic-wrapper");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .use_core()
        .allowlist_item("relic_.*")
        .allowlist_item("bn_.*")
        .allowlist_item("ep_.*")
        .allowlist_item("core_.*")
        .allowlist_item("fp_.*")
        .allowlist_item("fp.._.*")
        .allowlist_item("fp._.*")
        .allowlist_item("pc_.*")
        .allowlist_item("g._.*")
        .allowlist_item("RLC_.*")
        .allowlist_item("wrapper_.*")
        .impl_debug(false)
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        // Finish the builder and generate the bindings.
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
