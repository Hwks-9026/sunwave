#ifndef REPL_H
#define REPL_H

#include "interpreter.h"
#include <string>
#include <vector>

class Repl {
public:
    Repl(Interpreter& interp);
    void run();

private:
    Interpreter& sw;
    std::vector<std::string> history;
    int history_index = -1;
    bool active = true;

    void clear_screen();
    void process_command(const std::string& input);
    std::string read_input(); // Handles char-by-char input
    void handle_escape_sequence(std::string& line); // handles arrow key history
    void replace_line(std::string& current, const std::string& next);
};

#endif
