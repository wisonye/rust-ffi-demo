#include <iostream>
#include "./dynamic-lib/lib.h"

using namespace std;
using namespace Demo;

int main() {
    #ifdef ENABLE_DEBUG
        std::cout << "C++ Demo debug version" << std::endl;
    #else
        std::cout << "C++ Demo" << std::endl;
    #endif

    // Call lib function from here >>>>>>>>>>>
    
    print_helloworld();

    print_person_info(NULL);


    Person wison = create_new_person(
        "Wison", 
        "Ye", 
        Sex::Male, 
        88, 
        Location {
           "No 10, ABC Street",
           "Nice city",
           "Nice state",
           "Nice country"
        }
    );
    print_person_info(&wison);
    cout << get_person_info(&wison) << endl;


    Person* ptr_to_mike = create_new_person_and_return_pointer(
        "Lucy", 
        "Chen", 
        Sex::Male, 
        11, 
        Location {
           "No 11, ABC Street",
           "Nice city",
           "Nice state",
           "Nice country"
        }
    );

    // release_person_pointer(ptr_to_mike);

    print_person_info(ptr_to_mike);
    cout << get_person_info(ptr_to_mike) << endl;


    release_person_pointer(ptr_to_mike);

    return 0;
} 
