#include <iostream>
#include <vector>
#include <string>
#include <termios.h>
#include <unistd.h>
#include "interpreter.h"
#include "repl.h"

Repl::Repl(Interpreter& interp) : sw(interp) {}

void Repl::run() {
    std::cout << "Sunwave REPL (Ctrl+D or 'exit' to quit)" << std::endl;
    while (active) {
        std::string input = read_input();
        if (input == "exit") break;
        if (input.empty()) continue;

        process_command(input);
    }
}

// Clear screen and move cursor to 1,1
void Repl::clear_screen() {
    std::cout << "\033[2J\033[H" << std::flush;
}

void Repl::process_command(const std::string& input) {
    if (input == "clear") {
        clear_screen();
        return;
    }
    
    history.push_back(input);
    history_index = history.size();
    
    sw.exec(input);
}

std::string Repl::read_input() {
    std::string line;
    int cursor_pos = 0;
    history_index = history.size();

    std::cout << ">> " << std::flush;

    while (true) {
        char c;
        if (read(STDIN_FILENO, &c, 1) <= 0) break;

        if (c == '\n') { // Enter
            std::cout << std::endl;
            return line;
        } else if (c == 12) { // Control + L
            clear_screen();
            std::cout << ">> " << line << std::flush;
        } else if (c == 4) { // Control + D
            if (line.empty()) { active = false; return "exit"; }
        } else if (c == 27) { // Escape Sequence (Arrows)
            handle_escape_sequence(line);
        } else if (c == 127) { // Backspace
            if (!line.empty()) {
                line.pop_back();
                std::cout << "\b \b" << std::flush;
            }
        } else if (isprint(c)) {
            line += c;
            std::cout << c << std::flush;
        }
    }
    return line;
}

void Repl::handle_escape_sequence(std::string& line) {
    char seq[2];
    if (read(STDIN_FILENO, &seq[0], 1) <= 0) return;
    if (read(STDIN_FILENO, &seq[1], 1) <= 0) return;

    if (seq[0] == '[') {
        if (seq[1] == 'A') { // Up Arrow
            if (!history.empty() && history_index > 0) {
                history_index--;
                replace_line(line, history[history_index]);
            }
        } else if (seq[1] == 'B') { // Down Arrow
            if (history_index < (int)history.size() - 1) {
                history_index++;
                replace_line(line, history[history_index]);
            } else {
                history_index = history.size();
                replace_line(line, "");
            }
        }
    }
}

void Repl::replace_line(std::string& current, const std::string& next) {
    for (size_t i = 0; i < current.length(); ++i) std::cout << "\b \b";
    current = next;
    std::cout << current << std::flush;
}
