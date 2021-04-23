# Rust `FFI` (Foreign Function Interface) Demo


## What is `FFI` and when to use it

## Let's generate a `C++` library

- Use `cmake` to compile a dynamic library

    ```bash
    cd cpp && rm -rf build && mkdir build && cd build
    cmake ../ && make
    ```

    After that, `cmake` compiles your cpp project and generate the files below in the `cpp/build` folder:

    ```bash
    ffi-demo-cpp-lib_debug_version
    ffi-demo-cpp-lib

    # This is the dynamic C++ library we will used in `Rust` FFI demo
    # The library filename extension will be:
    # - `.dylib` on `MacOS`
    # - `.so` on `Linux`
    # - `.dll` on `Windows`
    libdemo.dylib
    ```

    </br>

- How to inspect the library's dynamic symbol table

    ```bash
    # Linux print dynamic symbol table
    objdump -T libdemo.so | grep "hello\|person\|Person\|Location"

    # MacOS(Darwin) can't print dynamic symbol table directly, so you can do either in this way:
    objdump -t libdemo.dylib | grep "hello\|person\|Person\|Location"

    # Or this way:
    nm -f bsd libdemo.dylib | grep "hello\|person\|Person\|Location"

    # Also, you can print the shared libraries used for linked Mach-O files:
    objdump -macho -dylibs-used libdemo.dylib
    ```
    
    </br>

## Let's call that `C++` library function in `Rust`

- Why we need to use the `#[no_mangle]` 

## Let's generate a `Rust` library

## Let's call that `Rust` library function in `C++`
