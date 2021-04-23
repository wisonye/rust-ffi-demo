#include "lib.h"
#include <iostream>

namespace Demo {

//
void print_helloworld() {
  std::cout << "'Helloword' from C++ dynamic lib." << std::endl;
}

//
Person *create_new_person(
        string first_name, 
        string last_name, 
        uint8_t age,
        Location location) {
    Person new_person = { first_name, last_name, age, location };
    Person* ptr = &new_person;
    return ptr;
}

} // namespace Demo
