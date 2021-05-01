# Rust `FFI` (Foreign Function Interface) Demo

[1. What is `ABI` and `FFI`?](#1-what-is-abi-and-ffi)</br>

[2. Let's build a `C++ Dynamic library` for this tutorial](#2-lets-build-a-c-dynamic-library-for-this-tutorial)</br>
[2.1 What will export via the `C++ Dynamic Library`](#21-what-will-export-via-the-c-dynamic-library)</br>
[2.2 Install `C++` and `cmake` building tools](#22-install-c-and-cmake-building-tools)</br>
[2.3 Use `cmake` to compile a dynamic library](#23-use-cmake-to-compile-a-dynamic-library)</br>
[2.4 How to inspect the library's dynamic symbol table](#24-how-to-inspect-the-librarys-dynamic-symbol-table)</br>

[3. How Rust deal with `FFI`?](#3-how-rust-deal-with-ffi)</br>
[3.1 `#[link]`](#31-link)</br>
[3.2 `extern` block](#32-extern)</br>
[3.3 How to transfers data type between `Rust` and `C/C++`?](#33-how-to-transfers-data-type-between-rust-and-cc)</br>
[3.4 How to generate the `extern` block from a `C/C++` header file?](#34-how-to-generate-the-extern-block-from-a-cc-header-file)</br>
[3.5 How `cargo build` knows where to link the `C++ dynamic library`?](#35-how-cargo-build-knows-where-to-link-the-c-dynamic-library)</br>

[4. Let's call `C++` function in `Rust`](#4-lets-call-c-function-in-rust)</br>
[4.1 Use manual `FFI` bindings](#41-use-manual-ffi-bindings)</br>
[4.2 Use `bindgen` automatic `FFI` bindings](#42-use-bindgen-automatic-ffi-bindings)</br>

[5. Let's build a `Rust Dynamic library`](#5-lets-build-a-rust-dynamic-library)</br>
[5.1 What will export via the `C++ Rust Library`](#51-what-will-export-via-the-rust-dynamic-library)</br>
[5.2 How to inspect the library's dynamic symbol table](#52-how-to-inspect-the-librarys-dynamic-symbol-table)</br>

[6. Let's call `Rust` function in `C++`](#6-lets-call-rust-function-in-c)</br>
[6.1 Create `calling-ffi/cpp/src/ffi.h`](#61-create-calling-fficppsrcffih-with-the-following-content)</br>
[6.2 Create `calling-ffi/cpp/src/main.cpp`](#62-create-calling-fficppsrcmaincpp-with-the-following-content)</br>
[6.3 Create `calling-ffi/cpp/CMakeLists.txt`](#63-create-calling-fficppcmakeliststxt-with-the-following-content)</br>
[6.4 Build and run](#64-build-and-run)</br>

[7. Let's call `Rust` function in `Node.JS`](#6-lets-call-rust-function-in-nodes)</br>

## 1. What is `ABI` and `FFI`

- `ABI` which stands for `Application Binary Interface`.

    It's an interface between two binary program modules. It looks like the `API` but focus on the `Compiler & Linker`, 
    as it covers:

    - processor instruction set (with details like register file structure, stack organization, memory access types, ...)
    - the sizes, layouts, and alignments of basic data types that the processor can directly access
    - the calling convention, which controls how the arguments of functions are passed, and return values retrieved. For example, it controls:
        - whether all parameters are passed on the stack, or some are passed in registers;
        - which registers are used for which function parameters;
        - and whether the first function parameter passed on the stack is pushed first or last onto the stack.
    - how an application should make system calls to the operating system, and if the ABI specifies direct system calls rather than procedure calls to system call stubs, the system call numbers.
    - and in the case of a complete operating system ABI, the binary format of object files, program libraries, and so on.

</br>

- `FFI` which stands for `Foreign Function Interface`

    It's talking about how the rust code can call the function outside the rust world.

</br>

## 2. Let's build a `C++ Dynamic library` for this tutorial

#### 2.1 What will export via the `C++ Dynamic Library`:

```c++
#pragma once
//#include <string>

namespace Demo {

// Simple function case
void print_helloworld();

//
// A more complex case with `enum`, `struct`, and a couple of
// functions to manipulate those data.
//
enum Gender {
    Female, Male
};

struct Location {
    // string street_address;
    // string city;
    // string state;
    // string country;
    const char* street_address;
    const char* city;
    const char* state;
    const char* country;
};

struct Person {
    // string first_name;
    // string last_name;
    const char* first_name;
    const char* last_name;
    Gender gender;
    unsigned char age;
    Location location;

    ~Person();
};

// Create `Person` instance on the heap and return pointer
Person* create_new_person(
        // string first_name, 
        // string last_name, 
        const char* first_name, 
        const char* last_name, 
        Gender gender,
        unsigned char age,
        Location location);

// Pass the `Person` pointer as parameter
void print_person_info(Person* ptr);

// Pass the `Person` pointer as parameter and get back C-style string
const char* get_person_info(Person* p);

// Pass the `Person` pointer as parameter
void release_person_pointer(Person* ptr);

} // namespace Demo
```

As you can see above, the `C++ Dynamic Library` will export some `enum` and `struct` types and some functions to maniplulate those stuff.

Because the `std::string` actually is a `class` (like a `vector<char>` or `vector<w_char>`) to manage the strings, it uses to enhance the `C-style string` (a char array), so we don't use this type at this moment to reduce the complexicy.

</br>

#### 2.2 Install `C++` and `cmake` building tools

- Arch
    
    ```bash
    sudo pacman --sync --refresh clang libc++ cmake
    ```

- MacOS

    ```bash
    brew install llvm clang cmake
    ```

</br>

#### 2.3 Use `cmake` to compile a dynamic library

```bash
cd ffi-dynamic-lib/cpp/ && rm -rf build && mkdir build && cd build
cmake ../ && make
```

After that, `cmake` compiles your cpp project and generate the files below in the `cpp/build` folder:

```bash
ffi-demo-cpp-lib_debug_version
ffi-demo-cpp-lib

# This is the C++ dynamic library which uses in this FFI demo
# The library filename extension will be:
# - `.dylib` on `MacOS`
libdemo.dylib
# - `.so` on `Linux`
libdemo.so
# - `.dll` on `Windows`
libdemo.dll
```

</br>

#### 2.4 How to inspect the library's dynamic symbol table

- Linux

    ```bash
    objdump -T libdemo.so | grep "hello\|person\|Person\|Location"
    # 0000000000000000      DF *UND*  0000000000000000              __gxx_personality_v0
    # 0000000000003310 g    DF .text  00000000000000b4  Base        _ZN4Demo16print_helloworldEv
    # 00000000000036f0 g    DF .text  0000000000000224  Base        _ZN4Demo17print_person_infoEPNS_6PersonE
    # 00000000000033d0 g    DF .text  0000000000000107  Base        _ZN4Demo6PersonD1Ev
    # 00000000000033d0 g    DF .text  0000000000000107  Base        _ZN4Demo6PersonD2Ev
    # 0000000000003920 g    DF .text  00000000000003ed  Base        _ZN4Demo15get_person_infoEPNS_6PersonE
    # 00000000000034e0 g    DF .text  00000000000001ba  Base        _ZN4DemolsERNSt3__113basic_ostreamIcNS0_11char_traitsIcEEEERKNS_6PersonE
    # 0000000000003d10 g    DF .text  0000000000000018  Base        _ZN4Demo22release_person_pointerEPNS_6PersonE
    # 00000000000036a0 g    DF .text  0000000000000049  Base        _ZN4Demo17create_new_personEPKcS1_NS_6GenderEhNS_8LocationE


    # Or
    nm -f bsd libdemo.so | grep "hello\|person\|Person\|Location"
    # 0000000000007128 d DW.ref.__gxx_personality_v0
    #                  U __gxx_personality_v0
    # 0000000000003920 T _ZN4Demo15get_person_infoEPNS_6PersonE
    # 0000000000003310 T _ZN4Demo16print_helloworldEv
    # 00000000000036a0 T _ZN4Demo17create_new_personEPKcS1_NS_6GenderEhNS_8LocationE
    # 00000000000036f0 T _ZN4Demo17print_person_infoEPNS_6PersonE
    # 0000000000003d10 T _ZN4Demo22release_person_pointerEPNS_6PersonE
    # 00000000000033d0 T _ZN4Demo6PersonD1Ev
    # 00000000000033d0 T _ZN4Demo6PersonD2Ev
    # 00000000000034e0 T _ZN4DemolsERNSt3__113basic_ostreamIcNS0_11char_traitsIcEEEERKNS_6PersonE
    ```

    </br>

- MacOS

    ```bash
    objdump -t libdemo.dylib | grep "hello\|person\|Person\|Location"
    # 00000000000019a0 g     F __TEXT,__text  __ZN4Demo15get_person_infoEPNS_6PersonE
    # 0000000000001310 g     F __TEXT,__text  __ZN4Demo16print_helloworldEv
    # 0000000000001690 g     F __TEXT,__text  __ZN4Demo17create_new_personEPKcS1_NS_6GenderEhNS_8LocationE
    # 00000000000016f0 g     F __TEXT,__text  __ZN4Demo17print_person_infoEPNS_6PersonE
    # 0000000000001d80 g     F __TEXT,__text  __ZN4Demo22release_person_pointerEPNS_6PersonE
    # 00000000000014b0 g     F __TEXT,__text  __ZN4Demo6PersonD1Ev
    # 00000000000013b0 g     F __TEXT,__text  __ZN4Demo6PersonD2Ev
    # 00000000000014c0 g     F __TEXT,__text  __ZN4DemolsERNSt3__113basic_ostreamIcNS0_11char_traitsIcEEEERKNS_6PersonE


    # Or
    nm -f bsd libdemo.dylib | grep "hello\|person\|Person\|Location"
    # 00000000000019a0 T __ZN4Demo15get_person_infoEPNS_6PersonE
    # 0000000000001310 T __ZN4Demo16print_helloworldEv
    # 0000000000001690 T __ZN4Demo17create_new_personEPKcS1_NS_6GenderEhNS_8LocationE
    # 00000000000016f0 T __ZN4Demo17print_person_infoEPNS_6PersonE
    # 0000000000001d80 T __ZN4Demo22release_person_pointerEPNS_6PersonE
    # 00000000000014b0 T __ZN4Demo6PersonD1Ev
    # 00000000000013b0 T __ZN4Demo6PersonD2Ev
    # 00000000000014c0 T __ZN4DemolsERNSt3__113basic_ostreamIcNS0_11char_traitsIcEEEERKNS_6PersonE


    # Also, you can print the shared libraries used for linked Mach-O files:
    objdump -macho -dylibs-used libdemo.dylib
    # libdemo.dylib:
    #         @rpath/libdemo.dylib (compatibility version 0.0.0, current version 0.0.0)
    #         /usr/lib/libc++.1.dylib (compatibility version 1.0.0, current version 400.9.4)
    #         /usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1252.250.1)
    ```

</br>

## 3. How Rust deal with `FFI`

#### 3.1 `#[link]`

The link attribute specifies the name of a native library 
that the compiler should link with for the items within an
extern block. 

```rust
#[link(name = "demo")]
extern {
    // …
}
```

In the above sample, `rustc` would try to link with `libdemo.so`
on unix-like systems and `demo.dll `on Windows at runtime. It 
panics if it can't find something to link to.  That's why you need
to make sure `rustc` can find the library file when linking. 

Also, you can add the `kind` value to say which kind the library it is:

- `dylib` — Indicates a dynamic library. This is the default if kind is not specified.
- `static` — Indicates a static library.
- `framework` — Indicates a `macOS` framework. This is only valid for `macOS` targets.

</br>

Here is the sample:

```rust
#[link(name = "CoreFoundation", kind = "framework")]
extern {
    // …
}
```

</br>

Another value you can put there is the `wasm_import_module` which use for linking
to the `WebAssembly` module case:

```rust
#[link(wasm_import_module = "wasm_demo")]
extern {
    // …
}
```

</br>


**Actually, the best practice is NOT use `#[link]` on the `extern` block. Instead, use the cargo instructions below in **`build.rs`** which will mentioned in the later chapters.**

- `cargo:rustc-link-lib=dylib=demo`
- `cargo:rustc-link-search=native=cpp/build`

</br>

#### 3.2 `extern`
    
The [`extern`](https://doc.rust-lang.org/stable/std/keyword.extern.html) block includes all the external function signatures.

```rust
#[link(name = "demo_c_lib")]
extern "C" {
    #[link_name = "\u{1}_ZN4Demo16my_c_functionEv"]
    fn my_c_function(x: i32) -> bool;
}
```

The `C` part actually is the `ABI` string, you can just write `extern` without `C` as the `C` is the default ABI.

Below is the `ABI` support list from [official `ABI` section](https://doc.rust-lang.org/stable/reference/items/external-blocks.html):

![rust-abi-list.png](./images/rust-abi-list.png)

</br>

The `#[link_name]` helps link to the correct external function which can be generated by the `bindgen` command.

</br>

#### 3.3 How to transfers data type between `Rust` and `C/C++`?

There are two modules to handle that:

- [`std::os:raw`](https://doc.rust-lang.org/stable/std/os/raw/index.html): Platform-specific types, as defined by C.
    
    | Rust type     | C/C++ type |
    |-------------- |----------------------------
    | `c_char`      | Equivalent to C's `char` type.
    | `c_double`    | Equivalent to C's `double` type.
    | `c_float`     | Equivalent to C's `float` type.
    | `c_int`       | Equivalent to C's `signed int (int)` type.
    | `c_long`      | Equivalent to C's `signed long (long)` type.
    | `c_longlong`  | Equivalent to C's `signed long long (long long)` type.
    | `c_schar`     | Equivalent to C's `signed char` type.
    | `c_short`     | Equivalent to C's `signed short (short)` type.
    | `c_uchar`     | Equivalent to C's `unsigned char` type.
    | `c_uint`      | Equivalent to C's `unsigned int` type.
    | `c_ulong`     | Equivalent to C's `unsigned long` type.
    | `c_ulonglong` | Equivalent to C's `unsigned long long` type.
    | `c_ushort`    | Equivalent to C's `unsigned short` type.

    </br>

- [`std::ffi`](https://doc.rust-lang.org/stable/std/ffi/index.html): 

    In particular, the `C-Style String` is the standard `C` string when dealing with `C/C++ FFI`.
    `C-Style String` just an `array of char (char[])`, but the string is nul-terminated which means
    they have a `\0` character at the end of the string. 

    The usual form is been using as a pointer:

    ```c++
    // const char[] pointer
    const char* ptr;

    // char[] pointer
    char* ptr;
    ```

    Below is usual case in `C/C++` function to
    accept a `C-Style String` or return a `C-Style String`:

    ```c++
    // `*const c_char` is rust type which equivalent to `const char*` in C/C++
    extern "C" { fn c_function_return_c_style_string() -> *const c_char; }

    // `*const c_char` is rust type which equivalent to `const char*` in C/C++
    extern "C" { fn c_function_accept_c_style_string_parameter(s: *const c_char); }
    ```

    For dealing with that `C-Style string`, `std::ffi` module introduces 2 extra data types:

    | Rust type     | Use case
    |-------------- |------------------------------------------
    | `CString`     | Pass `Rust String` as `C-Style String`
    | `CStr`        | Get back the `Rust String` by the `C-Style String`

    </br>

    So here is the use case sample:

    - Get back the `Rust String` by the `C-Style String`:

        As `c_function_return_c_style_string()` return `const char*` which means it
        just a raw pointer NOT guarantees still valid, that's why you need to wrap in
        `unsafe` block!

        The methods below dont' own the `C heap allocated` string which means you can
        use that string without copying or allocating:
        
        - `CStr::from_ptr().to_str()`
        - `CStr::from_ptr().to_string_lossy()` 
        - `CStr::from_ptr().into_c_string()` 

        </br>

        But if you're responsible for destroying that `C heap-allocated` string, then 
        you should own it and drop it after leaving the scope!

        </br>


        ```rust
        unsafe {
            let rust_string: String = CStr::from_ptr(c_function_return_c_style_string())
                .to_string_lossy()
                .into_owned();
        }
        ```

        </br>

    - Pass `Rust String` as `C-Style String`

        ```rust
        let c_string = CString::new("Hello, world!").unwrap();
        unsafe {
            c_function_accept_c_style_string_parameter(c_string.as_ptr());
        }
        ```

        </br>

#### 3.4 How to generate the `extern` block from a `C/C++` header file?

```bash
# Install `bindgen`:
cargo install bindgen

#
# bindgen [FLAGS] [OPTIONS] <header> -- <clang-args>...
#
# --disable-header-comment: Not include bindgen's version.
# --enable-cxx-namespaces: Enable support for C++ namespaces.
# --ignore-functions: Ignore functions, good for the case you only care about the `struct`.
# --no-derive-copy: No `#[derive(Copy)]` needed.
# --no-derive-debug: No `#[derive(Debug)]` needed.
# --no-doc-comments: No doc comment needed.
# --no-include-path-detection: Do not try to detect default include paths
# --no-layout-tests: No layout tests for any type.
#
# `--` Follow by all `clang_arg`:
# `-x c++`: Indictes that's the C++ if the header file not end with `.hpp`
# `-std=c++17`: The language standard version
# `-stdlib=libc++`: C++ standard library to use
#
cd calling-ffi/rust

bindgen \
    --disable-header-comment \
    --enable-cxx-namespaces \
    --no-derive-copy \
    --no-derive-debug \
    --no-doc-comments \
    --no-include-path-detection \
    --no-layout-tests \
    --output src/manual_bindings.rs \
    ../../ffi-dynamic-lib/cpp/src/dynamic-lib/lib.h \
    -- -x c++ \
    -std=c++17 \
    -stdlib=libc++
```

For `macOS`, you might see the error below:

```bash
fatal error: 'XXXX' file not found
```

Then try to add the `-I` `clang` flag explicitly like below:

```bash
cd calling-ffi/rust

bindgen \
    --disable-header-comment \
    --enable-cxx-namespaces \
    --no-derive-copy \
    --no-derive-debug \
    --no-doc-comments \
    --no-include-path-detection \
    --no-layout-tests \
    --output src/manual_bindings.rs \
    ../../ffi-dynamic-lib/cpp/src/dynamic-lib/lib.h \
    -- -x c++ \
    -I/Library/Developer/CommandLineTools/usr/include/c++/v1 \
    -std=c++17 \
    -stdlib=libc++
```
</br>

#### 3.5 How `cargo build` knows where to link the `C++ dynamic library`?

That's what exactly the `build script` does.

Placing a file named `build.rs` in the root of a package will cause `Cargo` to compile that script and execute it just **before** building the package. That's the right place to let `rustc` to know where to link the `C++ dynamic library`:

```rust
// FFI custom build script.
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
    println!("cargo:rustc-link-search=native=../../ffi-dynmaic-lib/cpp/build");
}
```

</br>

## 4. Let's call `C++` function in `Rust`

#### 4.1 Use manual `FFI` bindings

</br>

**Make sure `cd calling-ffi/rust` before doing the following steps!!!**

</br>

- Add the particular features to `Cargo.toml`:

    ```rust
    [features]
    default = []
    enable-manual-bindings = []
    ```

    `enable-manual-bindings` uses for compiling `build.rs` with the particular condition.

    </br>

- Generate [`src/manual_bindings.rs`](https://github.com/wisonye/rust-ffi-demo/blob/master/calling-ffi/rust/src/manual_bindings.rs) by running the command below:

    ```bash
    bindgen \
        --disable-header-comment \
        --enable-cxx-namespaces \
        --no-derive-debug \
        --no-derive-copy \
        --no-doc-comments \
        --no-include-path-detection \
        --no-layout-tests \
        --output src/manual_bindings.rs \
        ../../ffi-dynamic-lib/cpp/src/dynamic-lib/lib.h \
        -- -x c++ \
        -std=c++17 \
        -stdlib=libc++
    ```

    After that, you can see some bindings like below:

    ```rust
    #[repr(C)]
    pub struct Person {
        pub first_name: *const ::std::os::raw::c_char,
        pub last_name: *const ::std::os::raw::c_char,
        pub gender: root::Demo::Gender,
        pub age: ::std::os::raw::c_uchar,
        pub location: root::Demo::Location,
    }

    extern "C" {
        #[link_name = "\u{1}__ZN4Demo17create_new_personEPKcS1_NS_6GenderEhNS_8LocationE"]
        pub fn create_new_person(
            first_name: *const ::std::os::raw::c_char,
            last_name: *const ::std::os::raw::c_char,
            gender: root::Demo::Gender,
            age: ::std::os::raw::c_uchar,
            location: root::Demo::Location,
        ) -> *mut root::Demo::Person;
    }
    ```

    </br>

    - `#[repr(C)]`:

        `repr` stands for `representation`, it describes a `Type Layout` which you will find
        more explanation at [here](https://doc.rust-lang.org/reference/type-layout.html).

        This is the most important `repr`. It has fairly simple intent: **do what C does**. The 
        order, size, and alignment of fields is exactly what you would expect from C or C++. Any 
        type you expect to pass through an FFI boundary should have `repr(C)`.

        If you don't do that, you will get the warning like below and your executable will crash 
        with `SIGSEGV` error.:

        ```bash
        warning: `extern` block uses type `Person`, which is not FFI-safe
        ```

    - `[link_name]`

        The `link_name` attribute indicates the symbol to import for the given function which
        you've already saw it above via the `objdump` command.

    </br>


- [`src/bin/manual_ffi_binding_demo.rs`](https://github.com/wisonye/rust-ffi-demo/blob/master/calling-ffi/rust/src/bin/manual_ffi_binding_demo.rs) includes all the FFI calling samples.

</br>

- Create [`build.rs`](https://github.com/wisonye/rust-ffi-demo/blob/master/calling-ffi/rust/build.rs) with the following content:

    ```rust
    // FFI custom build script.
    fn main() {
        //
        // Link to `libdemo` dynamic library file
        //
        println!("cargo:rustc-link-lib=dylib=demo");
    
        //
        // Let `Cargo` to pass the `-L` flag to the compiler to add
        // the searching directory for the`native` library file
        //
        println!("cargo:rustc-link-search=native=../../ffi-dynamic-lib/cpp/build");
    }
    ```

    This allows `Cargo` to know where to link the `C++ dynamic library` file.

</br>

- Build and run the demo
    
    ```bash
    cargo clean && cargo build \
        --bin manual_ffi_binding_demo \
        --features "enable-manual-bindings" \
        --release
    
    LD_LIBRARY_PATH=../../ffi-dynamic-lib/cpp/build/ ./target/release/manual_ffi_binding_demo
    ```

    You should see demo output like below:

    ![manual_ffi_binding_demo-png.png](./images/manual_ffi_binding_demo-png.png)

    If you print the symbol table for the release executable, you should be able to 
    notic that it relies on the FFI functions in the `C++ Dynamic Library`:

    ```bash
    nm -f bsd target/release/manual_ffi_binding_demo | grep "hello\|person\|Person\|Location"
                     U __ZN4Demo15get_person_infoEPNS_6PersonE
                     U __ZN4Demo16print_helloworldEv
                     U __ZN4Demo17create_new_personEPKcS1_NS_6GenderEhNS_8LocationE
                     U __ZN4Demo17print_person_infoEPNS_6PersonE
                     U __ZN4Demo22release_person_pointerEPNS_6PersonE
    ```

</br>

So, you've already learned how to do that in a `manual bindings` way. The advantage of this 
way is that you have an FFI binding source code when you're coding, then your editor (with
Rust language plugin) can detect any error or show you the code completion handy feature 
when you're typing. 

But the disadvantage is that you need to run `bindgen` manually every time, as the function
symbol will be changed every time after you re-generate the `C++ Dynamic Library`. That
will be trouble or inconvenience. That's how `bindgen` automatic `FFI` bindings can help.

</br>

#### 4.2 Use `bindgen` automatic `FFI` bindings

</br>

**Make sure `cd calling-ffi/rust` before doing the following steps!!!**

</br>

- Add the build dependencies to `Cargo.toml`:

    ```rust
    [build-dependencies]
    bindgen = "~0.53.1"
    ```

    </br>

- Replace the following content to the [`build.rs`](https://github.com/wisonye/rust-ffi-demo/blob/master/calling-ffi/rust/build.rs):

    ```rust
    // FFI custom build script.
    
    #[cfg(not(feature = "enable-manual-bindings"))]
    use bindgen;
    
    #[cfg(not(feature = "enable-manual-bindings"))]
    use std::env;
    
    #[cfg(not(feature = "enable-manual-bindings"))]
    use std::path::PathBuf;
    
    fn main() {
        //
        // Link to `libdemo` dynamic library file
        //
        println!("cargo:rustc-link-lib=dylib=demo");
    
        //
        // Let `Cargo` to pass the `-L` flag to the compiler to add
        // the searching directory for the`native` library file
        //
        println!("cargo:rustc-link-search=native=../../ffi-dynamic-lib/cpp/build");
    

        #[cfg(not(feature = "enable-manual-bindings"))]
        {
            // Tell cargo to invalidate the built crate whenever the wrapper changes
            println!("cargo:rerun-if-changed=../../ffi-dynamic-lib/cpp/src/dynamic-lib/lib.h");
    
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
                .header("../../ffi-dynamic-lib/cpp/src/dynamic-lib/lib.h")
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
    ```

    </br>

- Add the following content to [`src/main.rs`](https://github.com/wisonye/rust-ffi-demo/blob/master/calling-ffi/rust/src/main.rs):

    ```rust
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    
    use root::Demo::{
        create_new_person, get_person_info, print_helloworld, print_person_info,
        release_person_pointer, Gender_Male, Location, Person,
    };
    use std::ffi::{CStr, CString};
    
    fn main() {
        println!("[ Auto FFI bindgins call demo ]\n");
    
        //
        // Ignore the same source code from `src/bin/manual_ffi_binding_demo.rs` here
        //
    
    }
    ```

    The `inclulde!` line macros actually put all the bindings source code into
    that line.

    If you have the problem below when using `rust-analyzer`:

    ![rust-analyzer-out-dir-issue.png](images/rust-analyzer-out-dir-issue.png)

    Plz make sure you DO NOT have the `"rust-analyzer.cargo.loadOutDirsFromCheck": false,` 
    settings in your configuration file (like `coc-settings.json` for example).

    </br>

- Build and run the demo:

    ```bash
    cargo clean && cargo build --release

    LD_LIBRARY_PATH=../../ffi-dynamic-lib/cpp/build/ ./target/release/ffi-demo
    ```
    
    </br>

From now on, `target/{BUILD_TYPE}/build/ffi-demo-XXXXXXXXXXXXXXXX/out/bindings.rs` will
be generated automatic every time when you run `cargo build`. Then you don't need to 
worry about running that manually or `bindings.rs` is the older version, more convenient
than before.

</br>

</br>


## 5. Let's build a `Rust Dynamic library`

</br>

**Make sure `cd ffi-dynamic-lib/rust` before doing the following steps!!!**

</br>

#### 5.1 What will export via the `Rust Dynamic Library`:

There are several parts inside this library:

###### 5.1.1 The `struct` definition:

```rust
///
#[derive(Debug)]
pub enum Gender {
    Female,
    Male,
    Unknown,
}

///
#[derive(Debug)]
pub struct Location {
    street_address: String,
    city: String,
    state: String,
    country: String,
}

///
pub struct Person {
    first_name: String,
    last_name: String,
    gender: Gender,
    age: u8,
    location: Location,
}
```

</br>

###### 5.1.2 The `extern` functions export to the outside world:

Because `Rust` has the `ownership and borrowing` concept, all rust code
under `borrow checker` control, actually should say under `borrow checker`'s 
protection.

But the `FFI` caller doesn't have that concept. If we pass the instance to
the outside world, then the `borrow checker` can't guarantee and protect
that instance memory.

The easy way is that allocates the instance on the heap, and then return 
its raw pointer.

As we hand over the instance raw pointer to the `FFI` caller, that will lose
control of the memory, that's why should have the `release` extern function
to return the control of memory we given out to make sure release the instance
memory correctly!!!

</br>

- Create new `Person` instance on the heap and return the raw pointer

    ```rust
    #[no_mangle]
    pub extern "C" fn create_new_person(
        first_name: *const c_char,
        last_name: *const c_char,
        gender: c_uchar,
        age: c_uchar,
        street_address: *const c_char,
        city: *const c_char,
        state: *const c_char,
        country: *const c_char,
    ) -> *mut Person {
        let temp_gender = match gender {
            0 => Gender::Female,
            1 => Gender::Male,
            _ => Gender::Unknown,
        };
    
        unsafe {
            let new_person = Person {
                first_name: CStr::from_ptr(first_name).to_string_lossy().into_owned(),
                last_name: CStr::from_ptr(last_name).to_string_lossy().into_owned(),
                gender: temp_gender,
                age: age as u8,
                location: Location {
                    street_address: CStr::from_ptr(street_address)
                        .to_string_lossy()
                        .into_owned(),
                    city: CStr::from_ptr(city).to_string_lossy().into_owned(),
                    state: CStr::from_ptr(state).to_string_lossy().into_owned(),
                    country: CStr::from_ptr(country).to_string_lossy().into_owned(),
                },
            };
    
            Box::into_raw(Box::new(new_person))
        }
    }
    ```
    A couple of things happen here:

    - `*const c_char`:

        The `C-Style String` (`const char*`) needs to be converted into `String`, 
        that why uses `*const std::os::raw::c_char` (immutable pointer to `c_char`).

        </br>

    - `#[no_mangle]`:

        The `no_mangle` attribute instructs the `rustc` compiler to not alter the
        function name when it is inserted to a binary file. This makes it easier
        for FFI users to call it, as the name is kept as "human-readable".

        When inspecting the dynamic library symbol table, you would see something
        like this `_create_new_person` instead of this `_rust_eh_personality`.

        </br>

    - `extern "C"`:

        `extern "C"` defines that this function should be callable outside Rust
        codebases, and use the "C ABI" calling convention.

        </br>
            
    - `Box::into_raw(Box::new(new_person))`:

        `Box::new()` allocates the instance on the heap, then it can leave as long
        as needed for the `FFI` caller to use.

        `Box::into_raw()` consumes the `Box<Person>` and return the wrapped raw pointer.


    </br>

- Release the `Person` instance raw pointer correctly

    ```rust
    pub extern "C" fn release_person_pointer(ptr: *mut Person) {
        if ptr.is_null() {
            return;
        }
    
        unsafe {
            Box::from_raw(ptr);
        }
    }
    ```

    This extern function accepts a raw pointer which returned from 
    `create_new_person()` and convert it back into `Box<Person>`,
    then the box destructor will cleanup the `Person` instance correctly.

    </br>

- Release `CString` instance raw pointer correctly

    ```rust
    #[no_mangle]
    pub extern "C" fn get_person_info(ptr: *mut Person) -> *mut c_char {
        if ptr.is_null() {
            return CString::new("").unwrap().into_raw();
        }
    
        unsafe { CString::new((*ptr).get_info()).unwrap().into_raw() }
    }

    #[no_mangle]
    pub extern "C" fn release_get_person_info(info_ptr: *mut c_char) {
        if info_ptr.is_null() {
            return;
        }
    
        unsafe {
            CString::from_raw(info_ptr);
        }
    }
    ```

    Because `Person.get_info()` returns a `String` instance, but the `FFI` 
    caller can't use it, then we need to convert it into a `CString` instance
    and call its `into_raw()` to produce a raw pointer which the `FFI` caller
    can use it as a `char *` string. `CString.into_raw()` consumes the `CString`
    and transfers ownership of the string to a `FFI(C)` caller.

    In particular, that raw pointer SHOULD NOT be deallocated by using the
    standard C `free()`. That's why we have the `release_get_person_info()`
    for doing the release step.

    </br>

- The `Drop` trait:
    
    This can prove that `Person` instance (includes `Person.location`) has been destroyed correctly.

    ```rust
    ///
    /// Customized drop trait
    ///
    impl Drop for Person {
        ///
        fn drop(&mut self) {
            println!(
                " [ Person instance get destroyed ] - first name: {}, last name: {}",
                &self.first_name, &self.last_name
            );
        }
    }
    
    ///
    /// Customized drop trait
    ///
    impl Drop for Location {
        ///
        fn drop(&mut self) {
            println!(
                " [ Person location instance get destroyed ] - street address: {}, city: {}",
                &self.street_address, &self.city
            );
        }
    }
    ```

    </br>

    Here is the [`ffi-dynamic-lib/rust/src/main.rs`](https://github.com/wisonye/rust-ffi-demo/blob/master/ffi-dynamic-lib/rust/src/main.rs).

    </br>


#### 5.2 Add the content below to `Cargo.toml`

```toml
[lib]
crate-type = ["cdylib"]
```

The setting above indicates that a dynamic system library will be produced. This is 
used when compiling a dynamic library to be loaded from another language. The output 
file extension will be different for the particular OS:

- Linux: `*.so`
- MacOS: `*.dylib`
- Windows: `*.dll`

</br>


#### 5.3 Build the library

```bash
cargo clean && cargo build --release
```

</br>

#### 5.2 How to inspect the library's dynamic symbol table

- Linux

    ```bash
    objdump -T ./target/release/librust.so | grep "person\|Person\|Location"
    # 0000000000026a00 g    DF .text  0000000000000351  Base        rust_eh_personality
    # 00000000000054f0 g    DF .text  000000000000065f  Base        create_new_person
    # 0000000000005ba0 g    DF .text  000000000000006e  Base        print_person_info
    # 0000000000005b50 g    DF .text  0000000000000048  Base        release_person_pointer
    # 0000000000005c10 g    DF .text  000000000000018c  Base        get_person_info
    # 0000000000005da0 g    DF .text  0000000000000028  Base        release_get_person_info


    # Or
    nm -f bsd ./target/release/librust.so | grep "person\|Person\|Location"
    # 00000000000054f0 T create_new_person
    # 0000000000049008 d DW.ref.rust_eh_personality
    # 0000000000005c10 T get_person_info
    # 0000000000005ba0 T print_person_info
    # 0000000000005da0 T release_get_person_info
    # 0000000000005b50 T release_person_pointer
    # 0000000000026a00 T rust_eh_personality
    # 0000000000032880 t _ZN4core5panic8Location6caller17h7a7acf437630d90eE
    # 0000000000005e40 t _ZN51_$LT$rust..Location$u20$as$u20$core..fmt..Debug$GT$3fmt17hd5037e5c9d432ecbE
    # 0000000000032890 t _ZN60_$LT$core..panic..Location$u20$as$u20$core..fmt..Display$GT$3fmt17hb4680bb747c9c063E
    ```

    </br>

- MacOS

    ```bash
    objdump -t ./target/release/librust.dylib | grep "person\|Person\|Location"
    # 0000000000001eb0 l     F __TEXT,__text  __ZN51_$LT$rust..Location$u20$as$u20$core..fmt..Debug$GT$3fmt17h33f040e226ce3834E
    # 000000000002a4c0 l     F __TEXT,__text  __ZN4core5panic8Location6caller17hb3a7d4b2fc73787cE
    # 000000000002a4d0 l     F __TEXT,__text  __ZN60_$LT$core..panic..Location$u20$as$u20$core..fmt..Display$GT$3fmt17h450055633af24029E
    # 0000000000001280 g     F __TEXT,__text  _create_new_person
    # 0000000000001c70 g     F __TEXT,__text  _get_person_info
    # 0000000000001c00 g     F __TEXT,__text  _print_person_info
    # 0000000000001e10 g     F __TEXT,__text  _release_get_person_info
    # 0000000000001bb0 g     F __TEXT,__text  _release_person_pointer
    # 0000000000022460 g     F __TEXT,__text  _rust_eh_personality

    # Or
    nm -f bsd ./target/release/librust.dylib | grep "person\|Person\|Location"
    # 000000000002a4c0 t __ZN4core5panic8Location6caller17hb3a7d4b2fc73787cE
    # 0000000000001eb0 t __ZN51_$LT$rust..Location$u20$as$u20$core..fmt..Debug$GT$3fmt17h33f040e226ce3834E
    # 000000000002a4d0 t __ZN60_$LT$core..panic..Location$u20$as$u20$core..fmt..Display$GT$3fmt17h450055633af24029E
    # 0000000000001280 T _create_new_person
    # 0000000000001c70 T _get_person_info
    # 0000000000001c00 T _print_person_info
    # 0000000000001e10 T _release_get_person_info
    # 0000000000001bb0 T _release_person_pointer
    # 0000000000022460 T _rust_eh_personality

    # Also, you can print the shared libraries used for linked Mach-O files:
    objdump -macho -dylibs-used ./target/release/librust.dylib
    # ./target/release/librust.dylib:
    #         /Users/wison/Rust/rust-ffi-demo/ffi-dynamic-lib/rust/target/release/deps/librust.dylib (compatibility version 0.0.0, current version 0.0.0)
    #         /usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1252.250.1)
    #         /usr/lib/libresolv.9.dylib (compatibility version 1.0.0, current version 1.0.0)
    ```


</br>

## 6. Let's call `Rust` function in `C++`

</br>

**Make sure `cd calling-ffi/cpp` before doing the following steps!!!**

</br>

#### 6.1 Create [`calling-ffi/cpp/src/ffi.h`](https://github.com/wisonye/rust-ffi-demo/blob/master/calling-ffi/cpp/src/ffi.h) with the following content:

```c++
#pragma once

//
// Declare extern FFI functions from Rust dynamic library
//
#ifdef __cplusplus
extern "C" {
#endif

typedef struct person person_t;

person_t *create_new_person(const char *first_name,
    const char *last_name,
    unsigned char gender, unsigned char age,
    const char *street_address,
    const char *city, const char *state,
    const char *country);

void release_person_pointer(person_t *);

void print_person_info(person_t *);

char *get_person_info(person_t *);

void release_get_person_info(char *);

#ifdef __cplusplus
}
#endif
```

</br>

#### 6.2 Create [`calling-ffi/cpp/src/main.cpp`](https://github.com/wisonye/rust-ffi-demo/blob/master/calling-ffi/cpp/src/main.cpp) with the following content:

```c++
#include "ffi.h"
#include <iostream>

using namespace std;

int main() {


    //
    // Call FFI functions
    //

    const char *first_name = "Wison";
    const char *last_name = "Ye";
    const char *street_address = "Wison's street_address here";
    const char *city = "Wison's city here";
    const char *state = "Wison's state here";
    const char *country = "Wison's country here";
    person_t *wison = create_new_person(
        first_name,
        last_name,
        1,
        88,
        street_address,
        city,
        state,
        country
    );

    print_person_info(wison);

    char *person_info_ptr = get_person_info(wison);
    cout << "\n>>> C++ caller print >>>\n" << person_info_ptr << "\n\n";
    release_get_person_info(person_info_ptr);
    
    release_person_pointer(wison);
    
    return 0;
}
```
</br>


#### 6.3 Create [`calling-ffi/cpp/CMakeLists.txt`](https://github.com/wisonye/rust-ffi-demo/blob/master/calling-ffi/cpp/CMakeLists.txt) with the following content:

```bash
cmake_minimum_required(VERSION "3.17.2")

set(CMAKE_HOST_SYSTEM_PROCESSOR X86_64)

set(CMAKE_C_COMPILER clang)
set(CMAKE_CXX_COMPILER clang++ -stdlib=libc++)

# Same with adding the compile flag `-std=c++17`
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED on)

# Build type
set(CMAKE_BUILD_TYPE Release)


#-------------------------------------------------------------------------------------
# Project settings
#-------------------------------------------------------------------------------------

# Define project name. After this, we can use "${PROJECT_NAME}" var to 
# dereference/re-use the project name as a String value.
project("calling-rust-in-cpp")

# Add directories in which the linker will look for libraries.
# This setting HAS TO define BEFORE `add_executable`!!!
link_directories(../../ffi-dynamic-lib/rust/target/release)

# Compile and build the executable
add_executable("${PROJECT_NAME}" "src/main.cpp")

# Link the particular library to the executable we build.
# It asks the linker to use `-llibrust` option which means
# link to the particular library file below for different OS:
#
# Linux   - librust.so
# MacOS   - librust.dylib
# Windows - librust.dll
target_link_libraries("${PROJECT_NAME}" "rust")
```

</br>


#### 6.4 Build and run

```bash
rm -rf build && mkdir build && cd build
cmake ../ && make

./calling-rust-in-cpp
```

You should see the output like below:

![calling-ffi-cpp-demo.png](./images/calling-ffi-cpp-demo.png)

</br>


