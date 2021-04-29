#include "./dynamic-lib/lib.h"
#include <iostream>

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

  {
    Person *ptr_to_wison =
        create_new_person("Wison", "Ye", Gender::Male, 88,
                          Location{"No 10, ABC Street", "Nice city",
                                   "Nice state", "Nice country"});
    print_person_info(ptr_to_wison);
    cout << get_person_info(ptr_to_wison) << endl;

    release_person_pointer(ptr_to_wison);
  }

  Person *ptr_to_lucy;
  {
    ptr_to_lucy = create_new_person("Lucy", "Chen", Gender::Female, 11,
                                    Location{"No 11, ABC Street", "Nice city",
                                             "Nice state", "Nice country"});

    // Uncomment this to trigger a `SIGSEGV` error:)
    // release_person_pointer(ptr_to_lucy);

    print_person_info(ptr_to_lucy);
    cout << get_person_info(ptr_to_lucy) << endl;
  }

  release_person_pointer(ptr_to_lucy);

  return 0;
}
