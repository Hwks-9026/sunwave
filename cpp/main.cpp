#include <string>
#include "sunwave.h"

int main (int argc, char *argv[]) {
    const std::string test_code = "x := (1, 2, 3)";
    SunwaveContext* ctx = sunwave_new_context();    
    sunwave_free_string(sunwave_execute(ctx, test_code.c_str()));
    return 0;
}



