#ifndef INTERPRETER_H
#define INTERPRETER_H

#include <string>
#include "sunwave.h"

enum InterpreterResult {
    Ok,
    Error,
};

class Interpreter {
    public:
        Interpreter();
        ~Interpreter();
        
        void exec(std::string code);
    private:
        SunwaveContext* ctx;
};

#endif
