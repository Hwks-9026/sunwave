#include <string>
#include "interpreter.h"
#include "sunwave.h"
#include <iostream>

Interpreter::Interpreter() {
    this->ctx = sunwave_new_context();
}
Interpreter::~Interpreter() {
    sunwave_free_context(this->ctx);
}

void Interpreter::exec(std::string code) {
    std::string result(sunwave_execute(this->ctx, code.c_str()));
    if ( result != "Ok") {
        std::cout << result << std::endl;
    }
}

