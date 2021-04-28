#include "lib.h"
#include <cstring>
#include <iostream>
#include <sstream>
#include <string>

using namespace std;

namespace Demo {

//
// Simple case
//
void print_helloworld() { cout << "Hello world to FFI:)" << endl; }

//
// Customized destructor
//
Person::~Person() {
  cout << "[ Person instance get destroyed ] - first name: " << first_name
       << ", last name: " << last_name << endl;
}

//
// Apply `<<` operator to `Person` struct
//
std::ostream &operator<<(ostream &stream_out, const Person &p) {
  stream_out << "[ Person Info ]\n{"
             << "\n\tFirst name: " << p.first_name
             << "\n\tLast name: " << p.last_name << "\n\tSex: " << p.sex
             << "\n\tAge: " << (int)p.age << "\n\tLocation: "
             << "\n\t\tStreet address: " << p.location.street_address
             << "\n\t\tCity: " << p.location.city
             << "\n\t\tcountry: " << p.location.state
             << "\n\t\tcountry: " << p.location.country << "\n}\n\n";

  return stream_out;
}

//
// Create `Person` instance on the heap and return pointer
//
Person *create_new_person(
    // string first_name,
    // string last_name,
    const char *first_name, const char *last_name, Sex sex, uint8_t age,
    Location location) {
  // Allocate on the heap
  Person *return_person = new Person{first_name, last_name, sex, age, location};
  return return_person;
}

//
// Print person info to console
//
void print_person_info(Person *ptr) {
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
const char *get_person_info(Person *p) {

  ostringstream os;
  os << "\n[ Person Info ]\n{"
     << "\n\tFirst name: " << p->first_name << "\n\tLast name: " << p->last_name
     << "\n\tSex: " << p->sex << "\n\tAge: " << (int)p->age << "\n\tLocation: "
     << "\n\t\tStreet address: " << p->location.street_address
     << "\n\t\tCity: " << p->location.city
     << "\n\t\tcountry: " << p->location.state
     << "\n\t\tcountry: " << p->location.country << "\n}\n\n";

  const string internal_str = os.str();

  // Allocate the new `char []` and save the info
  const char *temp_chars = internal_str.c_str();
  char *info = new char[internal_str.length() + 1];
  strcpy(info, temp_chars);
  return info;
}

//
// Release the person in memory
//
void release_person_pointer(Person *ptr) { delete ptr; }

} // namespace Demo
