#[path = "../manual_bindings.rs"]
mod manual_bindings;

use manual_bindings::root::Demo::{
    create_new_person, get_person_info, print_helloworld, print_person_info, Location, Person,
    Sex_Male,
};
use std::ffi::{CStr, CString};

use crate::manual_bindings::root::Demo::release_person_pointer;

///
fn main() -> () {
    println!("[ Manual FFI bindgins call demo ]\n");

    // helloworld
    unsafe {
        print_helloworld();
    }

    // `Person` related
    let wison_first_name = CString::new("Wison").unwrap();
    let wison_last_name = CString::new("Ye").unwrap();
    let wison_street = CString::new("My street").unwrap();
    let wison_city = CString::new("My city").unwrap();
    let wison_state = CString::new("My state").unwrap();
    let wison_country = CString::new("My country").unwrap();
    let wison_location = Location {
        street_address: wison_street.as_ptr(),
        city: wison_city.as_ptr(),
        state: wison_state.as_ptr(),
        country: wison_country.as_ptr(),
    };

    unsafe {
        // Get back `Person` raw pointer
        let wison: *mut Person = create_new_person(
            wison_first_name.as_ptr(),
            wison_last_name.as_ptr(),
            Sex_Male,
            18,
            wison_location,
        );

        let temp_first_name: String = CStr::from_ptr((*wison).first_name)
            .to_string_lossy()
            .into_owned();
        let temp_last_name: String = CStr::from_ptr((*wison).last_name)
            .to_string_lossy()
            .into_owned();
        let temp_street: String = CStr::from_ptr((*wison).location.street_address)
            .to_string_lossy()
            .into_owned();
        let temp_country: String = CStr::from_ptr((*wison).location.country)
            .to_string_lossy()
            .into_owned();

        // Customize print `Person` instance
        let wison_info = format!(
        "\n>>> Customized print >>>\n[ Wison Info ]:\nFirst name: {}\nLast name: {}\nLocation:\n\tStreet: {}\n\tCountry: {}\n",
        temp_first_name, temp_last_name, temp_street, temp_country
        );
        println!("{}", wison_info);

        // `print_person_info` print
        println!(">>> 'print_person_info' print >>>");
        print_person_info(wison);

        // `get_person_info` print
        let wison_info_from_c_string: String = CStr::from_ptr(get_person_info(wison))
            .to_string_lossy()
            .into_owned();
        println!(
            ">>> 'get_person_info' print >>>{}",
            wison_info_from_c_string
        );

        // Remember to free the instance
        release_person_pointer(wison);

        // Want to try double free? :)
        // release_person_pointer(wison);
    }
}
