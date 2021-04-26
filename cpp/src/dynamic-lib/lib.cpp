#include "lib.h"
#include <cstddef>
#include <cstring>
#include <string>
#include <iostream>
#include <sstream>

using namespace std;

namespace Demo {

//
// Apply `<<` operator to `Person` struct
//
std::ostream& operator<<(ostream& stream_out, const Person& p) {
    stream_out << 
        "\n[ Person Info ]\n{" <<
        "\n\tFirst name: " << p.first_name << 
        "\n\tLast name: " << p.last_name <<
        "\n\tSex: " << p.sex <<
        "\n\tAge: " << (int)p.age <<
        "\n\tLocation: " <<
        "\n\t\tStreet address: " << p.location.street_address <<
        "\n\t\tCity: " << p.location.city <<
        "\n\t\tcountry: " << p.location.state <<
        "\n\t\tcountry: " << p.location.country <<
        "\n}\n\n";

    return stream_out;
}

//
// Create new person instance and return it
//
Person create_new_person(
        // string first_name, 
        // string last_name, 
        const char* first_name, 
        const char* last_name, 
        Sex sex,
        uint8_t age,
        Location location) {
    return { first_name, last_name, sex, age, location };
}

//
// Create new person instance on the heap and return that pointer
//
Person *create_new_person_and_return_pointer(
        // string first_name, 
        // string last_name, 
        const char* first_name, 
        const char* last_name, 
        Sex sex,
        uint8_t age,
        Location location) {
    // Allocate on the heap
    Person* return_person = new Person { first_name, last_name, sex, age, location };
    return return_person;
}

//
// Print person info to console
//
void print_person_info(Person* ptr) {
    if (ptr == nullptr) {
        cout << "[ print_person_info ] - 'ptr' is NULL.\n";
        return;
    }

    std::ostringstream ss;
    ss << *ptr;
    cout << std::move(ss).str();
}

//
// Get back the person info (string) value
//
const char* get_person_info(Person* p) {

    ostringstream os;
    os << 
        "\n[ Person Info ]\n{" <<
        "\n\tFirst name: " << p->first_name << 
        "\n\tLast name: " << p->last_name <<
        "\n\tSex: " << p->sex <<
        "\n\tAge: " << (int)p->age <<
        "\n\tLocation: " <<
        "\n\t\tStreet address: " << p->location.street_address <<
        "\n\t\tCity: " << p->location.city <<
        "\n\t\tcountry: " << p->location.state <<
        "\n\t\tcountry: " << p->location.country <<
        "\n}\n\n";

    // Allocate the new `char []` and save the info
    const char* temp_chars = os.str().c_str();
    // char* info = new char[strlen(temp_chars) +1];
    char* info = new char[strlen(temp_chars)];
    strcpy(info, temp_chars);
    return info;
}

//
// Release the person in memory
//
void release_person_pointer(Person* ptr) {
    delete ptr;
    memset(ptr, 0x00, sizeof(Person));
}

} // namespace Demo
