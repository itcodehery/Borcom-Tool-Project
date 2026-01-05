# Borcom: Borrow Checker for C

Borcom is an project aimed at bringing Rust's powerful memory safety guarantees—specifically the **Borrow Checker**—to the C programming language.

## The Vision

Modern C development is plagued by memory safety issues such as use-after-free, double-free, and data races. Rust solved these problems through ownership and borrowing. **Borcom** aims to replicate this functionality for C, providing a static analysis tool that can verify ownership semantics and enforce safety rules without the overhead of a garbage collector.

## Current Status

The project is in its early stages. Currently, it features:
- **C Parser**: A custom-built recursive descent parser (implemented in Rust) capable of tokenizing and parsing basic C structures.
- **CLI Interface**: A user-friendly command-line interface (powered by `clap`) to validate `.c` files.

## Roadmap

- [x] Initial C Lexer and Parser implementation.
- [ ] Support for more complex C syntax (structs, pointers, etc.).
- [ ] Static analysis engine for tracking ownership.
- [ ] Lifetime inference for C variables.
- [ ] Integration with build systems.

## Quick Start

To parse a C file:
```bash
cargo run -- --file your_file.c
```

## Contributing

We are in the early stages of development and welcome contributions from those interested in compilers, static analysis, and systems safety.
