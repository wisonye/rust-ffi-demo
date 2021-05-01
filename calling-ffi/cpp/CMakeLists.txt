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
