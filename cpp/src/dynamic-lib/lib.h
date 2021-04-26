#pragma once
//#include <string>
#include <iostream>

using namespace std;

namespace Demo {

// Simple function case
void print_helloworld();

//
// A more complex case with `enum`, `struct`, and a couple of
// functions to manipulate those data.
//
enum Sex {
    Female, Male
};

struct Location {
    // string street_address;
    // string city;
    // string state;
    // string country;
    const char* street_address;
    const char* city;
    const char* state;
    const char* country;
};

struct Person {
    // string first_name;
    // string last_name;
    const char* first_name;
    const char* last_name;
    Sex sex;
    uint8_t age;
    Location location;

    ~Person() {
        cout << "[ Person instance get destroyed ] - first name: " 
            << first_name << ", last name: " << last_name << endl;
    }
};

// Return `Person` instance
Person create_new_person(
        // string first_name, 
        // string last_name, 
        const char* first_name, 
        const char* last_name, 
        Sex sex,
        uint8_t age,
        Location location);

// Return `Person` instance pointer
Person *create_new_person_and_return_pointer(
        // string first_name, 
        // string last_name, 
        const char* first_name, 
        const char* last_name, 
        Sex sex,
        uint8_t age,
        Location location);

// Pass the `Person` pointer as parameter
void print_person_info(Person* ptr);

// Pass the `Person` pointer as parameter and get back C-style string
const char* get_person_info(Person* p);

// Pass the `Person` pointer as parameter
void release_person_pointer(Person* ptr);

} // namespace Demo
