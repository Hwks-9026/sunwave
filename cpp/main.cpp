#include <string>
#include "interpreter.h"
#include "repl.h"

#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <termios.h>
#include <unistd.h>

using namespace std;

struct TermState {
    struct termios orig;
    void enable_raw() {
        tcgetattr(STDIN_FILENO, &orig);
        struct termios raw = orig;
        raw.c_lflag &= ~(ECHO | ICANON); // Disable echoing and line buffering
        tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw);
    }
    void disable_raw() {
        tcsetattr(STDIN_FILENO, TCSAFLUSH, &orig);
    }
};

string read_file(const std::string& path) {
    ifstream file(path);
    if (!file.is_open()) {
        throw std::runtime_error("Could not open file: " + path);
    }
    stringstream buffer;
    buffer << file.rdbuf();
    return buffer.str();
}

int main(int argc, char** argv) {
    try {
        Interpreter sw = Interpreter();

        // If a file path was provided, execute it, otherwise start the REPL
        if (argc > 1) {
            std::string path = argv[1];
            std::string code = read_file(path);
            sw.exec(code);
        } else {
            TermState t = TermState();
            Repl r = Repl(sw);
            t.enable_raw();
            r.run();
            t.disable_raw();

        }
    } catch (const std::exception& e) {
        std::cerr << "Fatal Error: " << e.what() << std::endl;
        return 1;
    }

    return 0;
}



