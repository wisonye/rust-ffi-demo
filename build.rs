// FFI custom build script.

#[cfg(not(feature = "enable-manual-bindings"))]
use bindgen;

#[cfg(not(feature = "enable-manual-bindings"))]
use std::env;

#[cfg(not(feature = "enable-manual-bindings"))]
use std::path::PathBuf;

fn main() {
    //
    // The `rustc-link-lib` instruction tells `Cargo` to link the
    // given library using the compiler's `-l` flag. This is typically
    // used to link a native library using FFI.
    //
    // If you've already add a `#[link(name = "demo"]` in the `extern`
    // block, then you don't need to provide this.
    //
    println!("cargo:rustc-link-lib=dylib=demo");

    //
    // The `rustc-link-search` instruction tells Cargo to pass the `-L`
    // flag to the compiler to add a directory to the library search path.
    //
    // The optional `KIND` may be one of the values below:
    //
    // - `dependency`: Only search for transitive dependencies in this directory.
    // - `crate`: Only search for this crate's direct dependencies in this directory.
    // - `native`: Only search for native libraries in this directory.
    // - `framework`: Only search for macOS frameworks in this directory.
    // - `all`: Search for all library kinds in this directory. This is the default
    //          if KIND is not specified.
    //
    println!("cargo:rustc-link-search=native=cpp/build");

    #[cfg(not(feature = "enable-manual-bindings"))]
    {
        // Tell cargo to invalidate the built crate whenever the wrapper changes
        println!("cargo:rerun-if-changed=cpp/src/dynamic-lib/lib.h");

        //
        // Write the bindings to the $OUT_DIR/bindings.rs file.
        //
        // For example:
        // - target/debug/build/ffi-demo-XXXXXXXXXXXXXXXX/out/bindings.rs
        // - target/release/build/ffi-demo-XXXXXXXXXXXXXXXX/out/bindings.rs
        //
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        println!("out_put: {:#?}", &out_path);

        // The bindgen::Builder is the main entry point to bindgen, and lets 
        // you build up options for the resulting bindings.
        let bindings = bindgen::Builder::default()
            // The input header we would like to generate bindings for.
            .header("cpp/src/dynamic-lib/lib.h")
            // Not generate the layout test code
            .layout_tests(false)
            // Not derive `Debug, Clone, Copy, Default` trait by default
            .derive_debug(false)
            .derive_copy(false)
            .derive_default(false)
            // Enable C++ namespace support
            .enable_cxx_namespaces()
            // Add extra clang args for supporting `C++`
            .clang_arg("-x")
            .clang_arg("c++")
            .clang_arg("-std=c++17")
            .clang_arg("-stdlib=libc++")
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
