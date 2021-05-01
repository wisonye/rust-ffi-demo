#pragma once

//
// Declare extern FFI functions from Rust dynamic library
//
#ifdef __cplusplus
extern "C" {
#endif

typedef struct person person_t;

person_t *create_new_person(const char *first_name,
    const char *last_name,
    unsigned char gender, unsigned char age,
    const char *street_address,
    const char *city, const char *state,
    const char *country);

void release_person_pointer(person_t *);

void print_person_info(person_t *);

char *get_person_info(person_t *);

void release_get_person_info(char *);

#ifdef __cplusplus
}
#endif
