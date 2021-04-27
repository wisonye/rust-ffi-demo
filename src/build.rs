// FFI custom build script.
fn main() {
    // The `rustc-link-lib` instruction tells `Cargo` to link the 
    // given library using the compiler's `-l` flag. This is typically 
    // used to link a native library using FFI.
    //
    // If you've already add a `#[link(name = "demo"]` in the `extern`
    // block, then you don't need to provide this.
    println!("cargo:rustc-link-lib=dylib=demo");

    println!("cargo:rustc-link-search=native=/Users/wison/Rust/rust-ffi-demo/cpp/build");

}
