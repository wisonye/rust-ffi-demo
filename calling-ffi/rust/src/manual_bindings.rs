#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod Demo {
        #[allow(unused_imports)]
        use self::super::super::root;
        extern "C" {
            #[link_name = "\u{1}__ZN4Demo16print_helloworldEv"]
            pub fn print_helloworld();
        }
        pub const Gender_Female: root::Demo::Gender = 0;
        pub const Gender_Male: root::Demo::Gender = 1;
        pub type Gender = ::std::os::raw::c_uint;
        #[repr(C)]
        pub struct Location {
            pub street_address: *const ::std::os::raw::c_char,
            pub city: *const ::std::os::raw::c_char,
            pub state: *const ::std::os::raw::c_char,
            pub country: *const ::std::os::raw::c_char,
        }
        #[repr(C)]
        pub struct Person {
            pub first_name: *const ::std::os::raw::c_char,
            pub last_name: *const ::std::os::raw::c_char,
            pub sex: root::Demo::Gender,
            pub age: ::std::os::raw::c_uchar,
            pub location: root::Demo::Location,
        }
        extern "C" {
            #[link_name = "\u{1}__ZN4Demo6PersonD1Ev"]
            pub fn Person_Person_destructor(this: *mut root::Demo::Person);
        }
        impl Person {
            #[inline]
            pub unsafe fn destruct(&mut self) {
                Person_Person_destructor(self)
            }
        }
        extern "C" {
            #[link_name = "\u{1}__ZN4Demo17create_new_personEPKcS1_NS_6GenderEhNS_8LocationE"]
            pub fn create_new_person(
                first_name: *const ::std::os::raw::c_char,
                last_name: *const ::std::os::raw::c_char,
                sex: root::Demo::Gender,
                age: ::std::os::raw::c_uchar,
                location: root::Demo::Location,
            ) -> *mut root::Demo::Person;
        }
        extern "C" {
            #[link_name = "\u{1}__ZN4Demo17print_person_infoEPNS_6PersonE"]
            pub fn print_person_info(ptr: *mut root::Demo::Person);
        }
        extern "C" {
            #[link_name = "\u{1}__ZN4Demo15get_person_infoEPNS_6PersonE"]
            pub fn get_person_info(p: *mut root::Demo::Person) -> *const ::std::os::raw::c_char;
        }
        extern "C" {
            #[link_name = "\u{1}__ZN4Demo22release_person_pointerEPNS_6PersonE"]
            pub fn release_person_pointer(ptr: *mut root::Demo::Person);
        }
    }
}
