# Sunwave - The Language of Chill Vibes and Headaches

## Overview

**Sunwave** is a functional programming language with side effects. If that weren't cursed enough, the project has two frontends, one written in rust, and the other in C++. They both use the rust backend.

For documentation on the language features, please see [Documentation](./docs/overview.md).

For syntax highliting, the sunwave language provides a vimscript file and a vscode plugin, both located in [syntax](./syntax/). 

./std contains the Sunwave standard library.

## Installation and Dependencies

To compile sunwave langauge, run `make`. Make will default to the C++ frontend (for the time being). Requirements for compilation are as follows:

- Cargo
- Rust version 1.88.0 or later
- The GCC C++ compiler with C++ 17

To verify that these dependencies are installed, run `make verify`.
If the rust toolchain isn't installed, you can install it with the following command, found on the [rust website](https://rust-lang.org/learn/get-started/). 
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

This command will error if you already have rust installed.
If rust is installed with rustup but not at a sufficiently recent version, `rustup update` should fix that.

## Sunwave Architecture

To learn more about the architecture of the language, and how the interpreter works, see [about_interpreter.md](./about_interpreter.md)
