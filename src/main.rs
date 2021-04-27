mod ffi_bindings;

use ffi_bindings::root::Demo;

fn main() {
    println!("[ Rust FFI Demo ]\n\n");

    unsafe {
        Demo::print_helloworld();
    }
}
