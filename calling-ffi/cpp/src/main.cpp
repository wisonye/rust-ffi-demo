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
