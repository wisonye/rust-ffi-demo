// mod ffi_bindings;
// 
// use ffi_bindings::root::Demo;

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use root::Demo;

fn main() {
    println!("[ Rust FFI Demo ]\n\n");

    unsafe {
        Demo::print_helloworld();
    }
}
