use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uchar};

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

///
impl std::fmt::Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("[ Person ]")
            .field("First name", &self.first_name)
            .field("Last name", &self.last_name)
            .field("Gender", &self.gender)
            .field("age", &self.age)
            .field("location", &self.location)
            .finish()
    }
}

///
impl Location {
    ///
    pub fn create_loction(
        street_address: *const c_char,
        city: *const c_char,
        state: *const c_char,
        country: *const c_char,
    ) -> Self {
        unsafe {
            Location {
                street_address: CStr::from_ptr(street_address)
                    .to_string_lossy()
                    .into_owned(),
                city: CStr::from_ptr(city).to_string_lossy().into_owned(),
                state: CStr::from_ptr(state).to_string_lossy().into_owned(),
                country: CStr::from_ptr(country).to_string_lossy().into_owned(),
            }
        }
    }
}

///
impl Person {
    ///
    pub fn print_info(&self) {
        println!("{:#?}", self);
    }

    ///
    pub fn get_info(&self) -> String {
        format!("{:#?}", self)
    }
}

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

// -----------------------------------------------------------------------------------
// Extern functions below
// -----------------------------------------------------------------------------------

///
/// As the FFI caller doesn't have the ownership concept, we can't return a new
/// `Person` instance and `move` the ownership to the outside world!
///
/// The easy way is that allocates a new `Person` instance on the heap, and then
/// return its raw pointer.
///
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

///
/// Release the raw pointer which returned from `create_new_person()`
/// and convert it back into `Box<Person>`, then the box destructor
/// will cleanup the `Person` instance correctly.
///
#[no_mangle]
pub extern "C" fn release_person_pointer(ptr: *mut Person) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        Box::from_raw(ptr);
    }
}

///
///
///
#[no_mangle]
pub extern "C" fn print_person_info(ptr: *mut Person) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        (*ptr).print_info();
    };
}

///
///
///
#[no_mangle]
pub extern "C" fn get_person_info(ptr: *mut Person) -> *mut c_char {
    if ptr.is_null() {
        return CString::new("").unwrap().into_raw();
    }

    unsafe { CString::new((*ptr).get_info()).unwrap().into_raw() }
}

///
///
///
#[no_mangle]
pub extern "C" fn release_get_person_info(info_ptr: *mut c_char) {
    if info_ptr.is_null() {
        return;
    }

    unsafe {
        CString::from_raw(info_ptr);
    }
}

