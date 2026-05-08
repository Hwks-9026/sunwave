#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <stdexcept>

#include "interpreter.h"
#include "repl.h"

using namespace std;

// Helper to read file content into a string
string read_file(const string& path) {
    ifstream file(path);
    if (!file.is_open()) {
        throw runtime_error("Could not open file: " + path);
    }
    stringstream buffer;
    buffer << file.rdbuf();
    return buffer.str();
}

int main(int argc, char** argv) {
    try {
        Interpreter sw;

        // If a file path was provided, execute it; otherwise, start the REPL
        if (argc > 1) {
            string path = argv[1];
            string code = read_file(path);
            sw.exec(code);
        } else {
            // The Repl class now handles its own terminal state (Raw Mode)
            // internally within its run/read_input methods.
            Repl r(sw);
            r.run();
        }
    } catch (const exception& e) {
        cerr << "Fatal Error: " << e.what() << endl;
        return 1;
    }

    return 0;
}
