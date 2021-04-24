#pragma once
#include <string>

using std::string;

namespace Demo {

struct Location {
    string street_address;
    string city;
    string state;
    string country;
};

struct Person {
    string first_name;
    string last_name;
    uint8_t age;
    Location location;
};

void print_helloworld();

Person *create_new_person(
        string first_name, 
        string last_name, 
        uint8_t age,
        Location location);

} // namespace Demo
