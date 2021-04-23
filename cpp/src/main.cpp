#include <iostream>
#include "./dynamic-lib/lib.h"

int main() {
    // Demo::Person* returned_person_ptr = Demo::create_new_person(, string last_name, uint8_t age, Location location)
    
    #ifdef ENABLE_DEBUG
        std::cout << "C++ Demo debug version" << std::endl;
    #else
        std::cout << "C++ Demo" << std::endl;
    #endif

    // Call lib function
    Demo::print_helloworld();

    return 0;
} 
