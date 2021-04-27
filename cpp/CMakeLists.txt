cmake_minimum_required(VERSION "3.17.2")

#-------------------------------------------------------------------------------------
# [Cross] compilation settings 
#-------------------------------------------------------------------------------------

# `CMAKE_HOST_SYSTEM_NAME`: The name of the CPU CMake is running on.
#
# Run `uname -s` to get the value of `CMAKE_HOST_SYSTEM_NAME` and `CMAKE_SYSTEM_NAME`:
# - Linux
# - Windows
# - Darwain
#
# set(CMAKE_HOST_SYSTEM_NAME Darwin)


# `CMAKE_SYSTEM_NAME` is by default set to the same value as the `CMAKE_HOST_SYSTEM_NAME` 
# variable so that the build targets the host system. If you want to do a cross-complication,
# then set to target OS value.
#
# set(CMAKE_SYSTEM_NAME Darwin)


# `CMAKE_HOST_SYSTEM_PROCESSOR`: The name of the CPU CMake is running on.
# 
# - Windows: this variable is set to the value of the environment variable `PROCESSOR_ARCHITECTURE`.
# 
# - Unix Platforms: On systems that support `uname`, this variable is set to the output of:
# 
#   - `uname -m` on GNU, Linux, Cygwin, Android, or
#
#   - `arch` on OpenBSD, or

#   - on other systems:
#       - uname -p if its exit code is nonzero, or
#       - uname -m otherwise.
# 
# - macOS Platforms: The value of `uname -m` is used by default.
# 
#   On Apple Silicon hosts, the architecture printed by uname -m may vary based on
#   CMake's own architecture and that of the invoking process tree.
#
set(CMAKE_HOST_SYSTEM_PROCESSOR X86_64)


# `CMAKE_SYSTEM_PROCESSOR`: 
#
# When not cross-compiling, this variable has the same value as the CMAKE_HOST_SYSTEM_PROCESSOR variable
#
# set(CMAKE_SYSTEM_PROCESSOR  X86_64)


# `CMAKE_C_COMPILER` and `CMAKE_CXX_COMPILER`: The full path to the compiler for `LANG`.
#
set(CMAKE_C_COMPILER clang)
set(CMAKE_CXX_COMPILER clang++  -stdlib=libc++)

# Same with adding the compile flag `-std=c++17`
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED on)

# If you want to do a cross-complication, then set to the target OS C/C++ compiler:
#
# [Cross] compilation settings for Linux GNU
# set(CMAKE_C_COMPILER /usr/bin/aarch64-linux-gnu-gcc)
# set(CMAKE_CXX_COMPILER /usr/bin/aarch64-linux-gnu-g++)
#
#
# [Cross] compilation settings for Linux MUSL
# set(CMAKE_C_COMPILER /usr/local/bin/aarch64-linux-musl-gcc)
# set(CMAKE_CXX_COMPILER /usr/local/bin/aarch64-linux-musl-g++)



# Build type
set(CMAKE_BUILD_TYPE Release)


#-------------------------------------------------------------------------------------
# Project settings
#-------------------------------------------------------------------------------------

# Define project name. After this, we can use "${PROJECT_NAME}" var to 
# dereference/re-use the project name as a String value.
project("ffi-demo-cpp")


# Compile the particular source code as a static library which named as
# - static lib:  lib${LIBRARY_NAME}.a
# - dynamic lib: lib${LIBRARY_NAME}.so
set(LIBRARY_NAME "demo")
set(LIBRARY_SOURCE_FILE "src/dynamic-lib/lib.cpp")
set(LIBRARY_HEADER_FILE "src/dynamic-lib/lib.h")

add_library("${LIBRARY_NAME}" SHARED "${LIBRARY_SOURCE_FILE}")
# add_library("${LIBRARY_NAME}" STATIC "${LIBRARY_SOURCE_FILE}")


# Compile and build the executable
add_executable("${PROJECT_NAME}" "src/main.cpp")
add_executable("${PROJECT_NAME}_debug_version" "src/main.cpp")


# Conditional compilation
target_compile_definitions("${PROJECT_NAME}_debug_version" PRIVATE ENABLE_DEBUG)


# Linker flag for creating static link executable, it will static link the `glibc`
# or `musl` to generate a single independent binary. 
#
# 1. For the `musl` build, you should enable this. Otherwise, when you run the executable
#    in `Alpine`, it will complain that missing the `/lib/ld-musl-aarch64.so.1` library.
#
# 2. It won't work if `add_library` with `SHARED` flag, as that's not make sense!!!
#
# set(CMAKE_EXE_LINKER_FLAGS " -static")


# Link the particular library to the executable we build
target_link_libraries("${PROJECT_NAME}" "${LIBRARY_NAME}")
target_link_libraries("${PROJECT_NAME}_debug_version" "${LIBRARY_NAME}")


# The settings below just copy the target into different output folder
#
# By default, `make install` will install to `/usr/local` which means:
#
# /usr/local/bin
# /usr/local/lib
# /usr/local/include
#
# Of course, you can change it by running `cmake` like below if you want:
#
# `cmake .. -DCMAKE_INSTALL_PREFIX=/cpp/build`
#
#  After that, `make install` will installed all targets to:
#
# /cpp/build/bin
# /cpp/build/lib
# /cpp/build/include
#
install(TARGETS "${LIBRARY_NAME}" DESTINATION lib/)
install(TARGETS "${PROJECT_NAME}" DESTINATION bin/)
install(TARGETS "${PROJECT_NAME}_debug_version" DESTINATION bin/)
install(FILES "${LIBRARY_HEADER_FILE}" DESTINATION include/)
