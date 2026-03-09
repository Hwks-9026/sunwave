# Sunwave - The Language of Chill Vibes and Headaches

## Overview

**Sunwave** is a functional programming language based on side effects. If that weren't cursed enough, the project has a c++ user interface, and a rust backend.

For documentation on the language features, please see [Documentation](./docs/overview.md).

For syntax highliting, the sunwave language provides a vimscript file and a vscode plugin, both located in [syntax](./syntax/). 

./std contains the Sunwave standard library.

**Sunwave* source code uses the same file extension as Sway, a blockchain contract langauge. Both this langauge and that one are esoteric enough that it should not matter. Just note that in the github langauge breakdown, any code marked as 'sway' is actually sunwave source code from either examples or the standard library.

## Installation and Dependencies

To compile sunwave langauge, run `make`. Make will default to the C++ frontend (for the time being). The final binary will be ./bin/sunwave. 

Requirements for compilation are as follows:

- Cargo
- Rust version 1.88.0 or later
- The GCC C++ compiler with C++ 17

To verify that these dependencies are installed, run `make verify`. Make will also automatically run verify each time `make` is run.
If the rust toolchain isn't installed, you can install it with the following command, found on the [rust website](https://rust-lang.org/learn/get-started/). 
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

This command will error if you already have rust installed.
If rust is installed with rustup but not at a sufficiently recent version, `rustup update` should fix that.

## Sunwave Architecture

To learn more about the architecture of the language, and how the interpreter works, see [about_interpreter.md](./about_interpreter.md)
