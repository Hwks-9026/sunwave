#include <iostream>
#include <vector>
#include <string>
#include <cctype>

#ifdef _WIN32
    #include <windows.h>
    #include <conio.h>
#else
    #include <termios.h>
    #include <unistd.h>
#endif

#include "interpreter.h"
#include "repl.h"

Repl::Repl(Interpreter& interp) : sw(interp), active(true), history_index(0) {}

// --- Platform Specific Terminal Handling ---

void Repl::enable_raw_mode() {
#ifdef _WIN32
    HANDLE hInput = GetStdHandle(STD_INPUT_HANDLE);
    DWORD mode;
    GetConsoleMode(hInput, &mode);
    // Disable line input, echo, and processed input
    SetConsoleMode(hInput, mode & ~(ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT));
#else
    struct termios t;
    tcgetattr(STDIN_FILENO, &t);
    t.c_lflag &= ~(ICANON | ECHO);
    tcsetattr(STDIN_FILENO, TCSANOW, &t);
#endif
}

void Repl::disable_raw_mode() {
#ifdef _WIN32
    HANDLE hInput = GetStdHandle(STD_INPUT_HANDLE);
    DWORD mode;
    GetConsoleMode(hInput, &mode);
    SetConsoleMode(hInput, mode | (ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT));
#else
    struct termios t;
    tcgetattr(STDIN_FILENO, &t);
    t.c_lflag |= (ICANON | ECHO);
    tcsetattr(STDIN_FILENO, TCSANOW, &t);
#endif
}

// --- REPL Logic ---

void Repl::run() {
    std::cout << "Sunwave REPL (Ctrl+D or 'exit' to quit)" << std::endl;
    while (active) {
        std::string input = read_input();
        if (input == "exit") {active = false; break;}
        if (input.empty()) continue;

        process_command(input);
    }
}

void Repl::clear_screen() {
    // Works on Linux and modern Windows 10+ terminals
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

void Repl::replace_line(std::string& current, const std::string& next) {
    // Erase current line visually
    for (size_t i = 0; i < current.length(); ++i) std::cout << "\b \b";
    current = next;
    std::cout << current << std::flush;
}

// Helper to keep history logic DRY (Don't Repeat Yourself)
void Repl::navigate_history(std::string& line, bool up) {
    if (up) {
        if (!history.empty() && history_index > 0) {
            history_index--;
            replace_line(line, history[history_index]);
        }
    } else {
        if (history_index < (int)history.size() - 1) {
            history_index++;
            replace_line(line, history[history_index]);
        } else if (history_index == (int)history.size() - 1) {
            history_index = history.size();
            replace_line(line, "");
        }
    }
}

std::string Repl::read_input() {
    std::string line;
    history_index = history.size();

    std::cout << ">> " << std::flush;
    enable_raw_mode();

    while (true) {
        int c;
#ifdef _WIN32
        c = _getch();
#else
        char ch;
        if (read(STDIN_FILENO, &ch, 1) <= 0) break;
        c = (unsigned char)ch;
#endif

        if (c == '\r' || c == '\n') { // Enter
            std::cout << std::endl;
            break;
        } 
        else if (c == 12) { // Ctrl + L
            clear_screen();
            std::cout << ">> " << line << std::flush;
        } 
        else if (c == 4) { // Ctrl + D
            if (line.empty()) { 
                active = false; 
                line = "exit"; 
                break; 
            }
        } 
#ifdef _WIN32
        else if (c == 0 || c == 224) { // Windows arrow keys
            int special = _getch();
            if (special == 72) navigate_history(line, true);  // Up
            else if (special == 80) navigate_history(line, false); // Down
        }
#else
        else if (c == 27) { // POSIX escape sequence
            char seq[2];
            if (read(STDIN_FILENO, &seq[0], 1) > 0 && read(STDIN_FILENO, &seq[1], 1) > 0) {
                if (seq[0] == '[') {
                    if (seq[1] == 'A') navigate_history(line, true);
                    else if (seq[1] == 'B') navigate_history(line, false);
                }
            }
        }
#endif
        else if (c == 8 || c == 127) { // Backspace
            if (!line.empty()) {
                line.pop_back();
                std::cout << "\b \b" << std::flush;
            }
        } 
        else if (isprint(c)) {
            line += (char)c;
            std::cout << (char)c << std::flush;
        }
    }

    disable_raw_mode();
    return line;
}
