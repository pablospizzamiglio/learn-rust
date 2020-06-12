# The Rust Programming Language

Here I'll be documenting my journey through the _the book_.

## Installation

Rust uses `rustup`, a command line tool for managing Rust versions and associated tools.

- On [Linux/macOS](https://doc.rust-lang.org/book/ch01-01-installation.html#installing-rustup-on-linux-or-macos)
- On [Windows](https://doc.rust-lang.org/book/ch01-01-installation.html#installing-rustup-on-windows)

## Updating and Uninstalling

- `rustup update` upgrades your Rust version.
- `rustup self uninstall` uninstalls Rust from your system.

## Basic Compilation

`rustc` is the command to invoke the Rust compiler.

- `rustc main.rs` compiles `main.rs` and generates and executable in the same directory that `main.rs` is located.

## Build System and Package Manager

`cargo` handles project compilation and also takes care of gathering and building your project's dependencies. It comes by default with your Rust installation.

- `cargo new <project_name>` makes a new project with a new Git repository and a `.gitignore` file.

- `cargo build` builds your entire project and generates an executable in `target/debug`, if you pass the `--release` flag the executable will be located in `target/release` instead and it will be compiled with optimizations.

- `cargo run` compiles your project and runs the resulting executable in a single step.

- `cargo check` makes sure your project compiles witout producing an executable.
