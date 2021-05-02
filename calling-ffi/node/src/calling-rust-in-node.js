const ffi = require('ffi-napi');

// console.log(ffi.types);

//
// Load `librust` dynmaic library
//
const librust = ffi.Library(`../../ffi-dynamic-lib/rust/target/release/librust`, {

    // 
    // #[no_mangle]
    // pub extern "C" fn create_new_person(
    //     first_name: *const c_char,
    //     last_name: *const c_char,
    //     gender: c_uchar,
    //     age: c_uchar,
    //     street_address: *const c_char,
    //     city: *const c_char,
    //     state: *const c_char,
    //     country: *const c_char,
    //
    'create_new_person': ['pointer', [
        'string',
        'string',
        'uchar',
        'uchar',
        'string',
        'string',
        'string',
        'string'
    ]
    ],

    //
    // #[no_mangle]
    // pub extern "C" fn release_get_person_info(info_ptr: *mut c_char) {
    //
    'release_person_pointer': ['void', ['pointer']],

    //
    // #[no_mangle]
    // pub extern "C" fn print_person_info(ptr: *mut Person) {
    //
    'print_person_info': ['void', ['pointer']],

    //
    // #[no_mangle]
    // pub extern "C" fn get_person_info(ptr: *mut Person) -> *mut c_char {
    //
    'get_person_info': ['char *', ['pointer']],

    //
    // #[no_mangle]
    // pub extern "C" fn release_get_person_info(info_ptr: *mut c_char) {
    //
    'release_get_person_info': ['void', ['char *']],
})

const newPersonPtr = librust.create_new_person(
    `Wison`,
    `Ye`,
    1,
    50,
    `Wison's street_address here`,
    `Wison's city here`,
    `Wison's state here`,
    `Wison's country here`,
)

try {
    console.log(`>>> 'print_person_info' print >>>`)
    librust.print_person_info(newPersonPtr)

    const personInfoPtr = librust.get_person_info(newPersonPtr)
    console.log(`\n>>> 'get_person_info' print >>>\n${personInfoPtr.readCString()}\n`)

    librust.release_get_person_info(personInfoPtr)
} finally {
    librust.release_person_pointer(newPersonPtr)
}



