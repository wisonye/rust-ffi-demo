#pragma once
#include <string>
#include <iostream>

using namespace std;

namespace Demo {

enum Sex {
    Female, Male
};

struct Location {
    string street_address;
    string city;
    string state;
    string country;
};

struct Person {
    string first_name;
    string last_name;
    Sex sex;
    uint8_t age;
    Location location;

    ~Person() {
        cout << "[ Person instance get destroyed ] - first name: " << first_name << ", last name: " << last_name << endl;
    }
};

Person create_new_person(
        string first_name, 
        string last_name, 
        Sex sex,
        uint8_t age,
        Location location);

Person *create_new_person_and_return_pointer(
        string first_name, 
        string last_name, 
        Sex sex,
        uint8_t age,
        Location location);

void print_person_info(Person* ptr);

std::string get_person_info(Person* ptr);

void release_person_pointer(Person* ptr);

} // namespace Demo
