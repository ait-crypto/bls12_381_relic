use std::{env, path::PathBuf};

#[cfg(feature = "vendored")]
fn build() -> PathBuf {
    let mut cmake = cmake::Config::new("relic");
    cmake
        .define("WSIZE", env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap())
        .define("RAND", "UDEV")
        .define("SHLIB", "OFF")
        .define("STBIN", "OFF")
        .define("STLIB", "ON")
        .define("TIMER", "")
        .define(
            "CHECK",
            if env::var("PROFILE").unwrap() == "debug"
                || env::var("PROFILE").unwrap() == "dev"
                || env::var("DEBUG").unwrap() == "0"
                || env::var("DEBUG").unwrap() == "false"
            {
                "ON"
            } else {
                "OFF"
            },
        )
        .define("BENCH", "0")
        .define("TESTS", "0")
        .define("VERBS", "OFF")
        .define("FP_PRIME", "381")
        .define("FP_METHD", "INTEG;INTEG;INTEG;MONTY;LOWER;LOWER;SLIDE")
        .define("FP_PMERS", "off")
        .define("FP_QMRES", "on")
        .define("FPX_METHD", "INTEG;INTEG;LAZYR")
        .define("EP_PLAIN", "off")
        .define("EP_SUPER", "off")
        .define("PP_METHD", "LAZYR;OATEP");
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "x86_64" {
        cmake.define("ARCH", "X64").define("ARITH", "x64-asm-382");
    }

    let dst = cmake.build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=relic_s");
    dst
}

fn find_lib() -> Option<PathBuf> {
    #[cfg(feature = "system")]
    {
        // Try to find shared library via pkg-config
        if pkg_config::Config::new().probe("relic").is_ok() {
            return None;
        }
    }

    #[cfg(not(feature = "vendored"))]
    panic!("Unable to find library with pkg-config and vendored is not enabled!");
    #[cfg(feature = "vendored")]
    // Download and build static library
    Some(build())
}

fn main() {
    let relic_path = find_lib();

    // Invalidate the built crate whenever the wrapper and the build script changes.
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=wrapper.c");
    println!("cargo:rerun-if-changed=build.rs");

    let mut build = cc::Build::new();
    build.flag_if_supported("-std=gnu11");
    build.flag_if_supported("-fstack-protector-strong");
    build.flag_if_supported("-Werror=incompatible-pointer-types");
    if let Some(ref relic_path) = relic_path {
        build.include(relic_path.join("include"));
        build.include(format!("{}/relic/include", env!("CARGO_MANIFEST_DIR")));
    }
    build.define("_FORTIFY_SOURCE", Some("2"));
    build.files(["wrapper.c"].iter()).compile("relic-wrapper");

    let binding_builder = bindgen::Builder::default()
        .header("wrapper.h")
        // Invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .use_core()
        .allowlist_item("relic_.*")
        .allowlist_item("bn_.*")
        .allowlist_item("ep2?_.*")
        .allowlist_item("core_.*")
        .allowlist_item("fp_.*")
        .allowlist_item("fp1?[0-9]_.*")
        .allowlist_item("pc_.*")
        .allowlist_item("g[12t]_.*")
        .allowlist_item("RLC_.*")
        .allowlist_item("wrapper_.*")
        .impl_debug(false)
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed);
    let binding_builder = if let Some(ref relic_path) = relic_path {
        binding_builder
            .clang_arg(format!("-I{}", relic_path.join("include").display()))
            .clang_arg(format!("-I{}/relic/include", env!("CARGO_MANIFEST_DIR")))
    } else {
        binding_builder
    };
    let bindings = binding_builder
        // Finish the builder and generate the bindings.
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");
}
