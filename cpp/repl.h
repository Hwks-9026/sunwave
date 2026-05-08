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
    // Members reordered to match constructor initialization order
    Interpreter& sw;
    bool active = true;
    int history_index = 0;
    std::vector<std::string> history;

    // Terminal state management
    void enable_raw_mode();
    void disable_raw_mode();

    // Input and UI logic
    void clear_screen();
    void process_command(const std::string& input);
    std::string read_input();
    
    // History navigation helpers
    void navigate_history(std::string& line, bool up);
    void handle_escape_sequence(std::string& line); 
    void replace_line(std::string& current, const std::string& next);
};

#endif
