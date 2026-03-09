#ifndef SUNWAVE_H
#define SUNWAVE_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque handle to the Rust Environment.
typedef struct SunwaveContext SunwaveContext;

// Constructor & Destructor
SunwaveContext* sunwave_new_context();
void sunwave_free_context(SunwaveContext* ctx);

// Execution Logic
char* sunwave_execute(SunwaveContext* ctx, const char* code);

// Rust CStr Memory Management
// sunwave_execute returns a pointer that MUST be freed 
// via sunwave_free_string to avoid memory leaks.
void sunwave_free_string(char* s);

#ifdef __cplusplus
}
#endif

#endif /* SUNWAVE_H */
