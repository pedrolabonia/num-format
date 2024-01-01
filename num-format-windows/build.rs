#[cfg(windows)]
fn main() {
    use std::env;
    use std::path::Path;

    use bindgen::{Builder, RustTarget};

    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let headers_path = Path::new(&root).join("wrapper.h");
    let headers = headers_path.to_str().unwrap();

    let bindings = Builder::default()
        .header(headers)
        .rust_target(RustTarget::Stable_1_33)
        // https://github.com/rust-lang/rust-bindgen/issues/2500
        // This only disables avx512 for bindgen parsing the heaers, not for the actual build,
        // this should be fine, since avx512 types are not exposed in the public API, they are
        // only an implementation detail.
        // .clang_arg("-DCROARING_COMPILER_SUPPORTS_AVX512=0")
        // .clang_arg("-U__AVX2__")
        .clang_arg("-D__AVX512FP16INTRIN_H")
        .clang_arg("-D__AVX512VLFP16INTRIN_H")
        .allowlist_var("LOCALE_NAME_MAX_LENGTH")
        .allowlist_var("LOCALE_NAME_SYSTEM_DEFAULT")
        .allowlist_var("LOCALE_SDECIMAL")
        .allowlist_var("LOCALE_SGROUPING")
        .allowlist_var("LOCALE_SNAME")
        .allowlist_var("LOCALE_SNAN")
        .allowlist_var("LOCALE_SNEGATIVESIGN")
        .allowlist_var("LOCALE_SNEGINFINITY")
        .allowlist_var("LOCALE_SPOSINFINITY")
        .allowlist_var("LOCALE_SPOSITIVESIGN")
        .allowlist_var("LOCALE_STHOUSAND")
        .generate()
        .expect("unable to generate bindings for windows.h");

    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(&out_path)
        .expect("unable to write bindings for windows.h");
}

#[cfg(not(windows))]
fn main() {}
