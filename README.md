# Rust `FFI` (Foreign Function Interface) Demo

## 1. What is `ABI` and `FFI`?

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

## 2. How Rust deal with `FFI`?

Rust uses `#[link]` attribute and `extern` keyword to interact with the external function calls:

- `#[link]`:

    The link attribute specifies the name of a native library that the compiler should link with for the items within an extern block. 

    ```rust
    #[link(name = "demo")]
    extern {
        // …
    }
    ```

    In the above sample, `rustc` would try to link with `libdemo.so` on unix-like systems and 
    `demo.dll `on Windows at runtime. It panics if it can't find something to link to.

    That's why you need to make sure `rustc` can find the libaray file via the system lib path.
    For example in `macOS`, it would one of the path below:

    - `$LD_LIBRARY_PATH`
    - `/Library/Developer/CommandLineTools/usr/lib/`
    - `/Library/Developer/CommandLineTools/usr/lib/clang/{VERSION}/lib/darwin/`
    - etc.

    </br>

    Or, you can put the extra path with the `name` like below:

    ```rust
    #[link(name = "cpp/build/demo.dylib")]
    extern {
        // …
    }
    ```

    </br>

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

- `extern`:
    
    The [`extern`](https://doc.rust-lang.org/stable/std/keyword.extern.html) block includes all the external function signatures.

    ```rust
    #[link(name = "demo_c_lib")]
    extern "C" {
        fn my_c_function(x: i32) -> bool;
    }
    ```

    The `C` part actually is the `ABI` string, you can just write `extern` without `C` as the `C` is the default ABI.

    Below is the `ABI` support list from [official `ABI` section](https://doc.rust-lang.org/stable/reference/items/external-blocks.html):
    
    ![rust-abi-list.png](./images/rust-abi-list.png)
</br>

## 3. Let's build a `C++` library for this tutorial

- What will export via the `C++ Dynamic Library`:

    ```c++
    #pragma once
    //#include <string>
    #include <iostream>
    
    using namespace std;
    
    namespace Demo {
    
    // Simple function case
    void print_helloworld();
    
    //
    // A more complex case with `enum`, `struct`, and a couple of
    // functions to manipulate those data.
    //
    enum Sex {
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
        Sex sex;
        uint8_t age;
        Location location;
    
        ~Person() {
            cout << "[ Person instance get destroyed ] - first name: " 
                << first_name << ", last name: " << last_name << endl;
        }
    };
    
    // Return `Person` instance
    Person create_new_person(
            // string first_name, 
            // string last_name, 
            const char* first_name, 
            const char* last_name, 
            Sex sex,
            uint8_t age,
            Location location);
    
    // Return `Person` instance pointer
    Person *create_new_person_and_return_pointer(
            // string first_name, 
            // string last_name, 
            const char* first_name, 
            const char* last_name, 
            Sex sex,
            uint8_t age,
            Location location);
    
    // Pass the `Person` pointer as parameter
    void print_person_info(Person* ptr);
    
    // Pass the `Person` pointer as parameter and get back C-style string
    const char* get_person_info(Person* ptr);
    
    // Pass the `Person` pointer as parameter
    void release_person_pointer(Person* ptr);
    
    } // namespace Demo
    ```

    As you can see above, the `C++ Dynamic Library` will export some `enum` and `struct` types and some functions to maniplulate those stuff.

    Because the `std::string` actually is a `class` (like a `vector<char>` or `vector<w_char>`) to manage the strings, it uses to enhance the `C-style string` (a char array), so we don't use this type at this moment to reduce the complexicy.

    </br>

- Use `cmake` to compile a dynamic library

    ```bash
    cd cpp && rm -rf build && mkdir build && cd build
    cmake ../ && make
    ```

    After that, `cmake` compiles your cpp project and generate the files below in the `cpp/build` folder:

    ```bash
    ffi-demo-cpp-lib_debug_version
    ffi-demo-cpp-lib

    # This is the C++ dynamic library which uses in this FFI demo
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
    objdump -T libdemo.so | grep "person\|Person\|Location"


    # MacOS(Darwin) can't print dynamic symbol table directly, so you can do either in this way:
    objdump -t libdemo.dylib | grep "person\|Person\|Location"
    # 0000000000002390 lw    F __TEXT,__text  __ZN4Demo6PersonD2Ev
    # 00000000000018f0 g     F __TEXT,__text  __ZN4Demo15get_person_infoEPNS_6PersonE
    # 00000000000015a0 g     F __TEXT,__text  __ZN4Demo17create_new_personEPKcS1_NS_3SexEhNS_8LocationE
    # 0000000000001640 g     F __TEXT,__text  __ZN4Demo17print_person_infoEPNS_6PersonE
    # 0000000000001ca0 g     F __TEXT,__text  __ZN4Demo22release_person_pointerEPNS_6PersonE
    # 00000000000015e0 g     F __TEXT,__text  __ZN4Demo36create_new_person_and_return_pointerEPKcS1_NS_3SexEhNS_8LocationE
    # 00000000000013d0 g     F __TEXT,__text  __ZN4DemolsERNSt3__113basic_ostreamIcNS0_11char_traitsIcEEEERKNS_6PersonE


    # Or this way:
    nm -f bsd libdemo.dylib | grep "person\|Person\|Location"


    # Also, you can print the shared libraries used for linked Mach-O files:
    objdump -macho -dylibs-used libdemo.dylib
    # libdemo.dylib:
    #         @rpath/libdemo.dylib (compatibility version 0.0.0, current version 0.0.0)
    #         /usr/lib/libc++.1.dylib (compatibility version 1.0.0, current version 400.9.4)
    #         /usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1252.250.1)
    ```
    
    </br>

## 3. Let's call that `C++` library function in `Rust`

Before calling `C++` function, we need to solve a couples of problems below:

</br>

- How `Rust` transfers data type between `Rust` and `C/C++`?

    There are two modules to handle that:

    - [`std::os:raw`](https://doc.rust-lang.org/stable/std/os/raw/index.html`): Platform-specific types, as defined by C.
        
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

- How to generate the `extern` block from a `C/C++` header file?

</br>

- How `cargo build` know where to link the `C++ dynamic library`?

    Placing a file named `build.rs` in the root of a package will cause `Cargo` to compile that script and execute it just **before** building the package. That's the right place to let `rustc` to know where to link the `C++ dynamic library`:

    ```rust
    ```

</br>

Rust guarantees that the layout of a struct is compatible with the platform's representation in C only if the #[repr(C)] attribute is applied to it. #[repr(C, packed)] can be used to lay out struct members without padding. #[repr(C)] can also be applied to an enum.
Let's figure out and solve them one by one:)

- How `Rust` knows about the user-defined type in `C++` (like: `struct`, `class`, `enum`, etc.)?

https://doc.rust-lang.org/nomicon/ffi.html
https://docs.rs/cmake/0.1.45/cmake/

</br>

- How `build.rs` and `bindgen` can help us and make it easier



https://doc.rust-lang.org/cargo/reference/build-scripts.html

```bash
# bindgen [FLAGS] [OPTIONS] <header> -- <clang-args>...

bindgen --enable-cxx-namespaces \
    --no-layout-tests \
    cpp/src/dynamic-lib/lib.h \
    -- -x c++ \
    -std=c++17 \
    -stdlib=libc++ \
> ~/temp/temp_bindgen.rs
```

For `macOS`, you might see the error below:

```bash
fatal error: 'string' file not found
```

Then try to add the `-I` `clang` flag explicitly like below:

```bash
bindgen --enable-cxx-namespaces \
    --no-layout-tests \
    cpp/src/dynamic-lib/lib.h \
    -- -x c++ \
    -I/Library/Developer/CommandLineTools/usr/include/c++/v1 \
    -std=c++17 \
    -stdlib=libc++ \
> ~/temp/temp_bindgen.rs
```

- Why we need to use the `#[no_mangle]` 

## Let's generate a `Rust` library

## Let's call that `Rust` library function in `C++`
