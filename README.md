# rsh

> A minimal interactive shell written in Rust.

**rsh** is a small Unix-like shell built from scratch with Rust.

The project is mainly an experiment in learning how command-line shells work, while exploring process management, filesystem operations, terminal interaction, and Rust system programming.

## Features

* Interactive shell prompt
* Username and hostname display
* Current working directory display
* Built-in commands
* Directory navigation
* Directory listing
* Colored directory output
* Hidden-file filtering
* Command suggestion using Levenshtein distance
* Terminal clearing
* Lightweight and minimal codebase

## Built-in Commands

| Command | Description                         |
| ------- | ----------------------------------- |
| `echo`  | Print text to the terminal          |
| `upper` | Convert text to uppercase           |
| `lower` | Convert text to lowercase           |
| `cd`    | Change the current directory        |
| `ls`    | List directory contents             |
| `pwd`   | Print the current working directory |
| `clear` | Clear the terminal                  |
| `exit`  | Exit rsh                            |

## Example

```text
[username@hostname ~/projects]$ echo Hello
Hello

[username@hostname ~/projects]$ upper hello world
HELLO WORLD

[username@hostname ~/projects]$ lower HELLO WORLD
hello world

[username@hostname ~/projects]$ pwd
/home/username/projects

[username@hostname ~/projects]$ ls
src Cargo.toml README.md

[username@hostname ~/projects]$ pwdd
command not found
Did you mean : pwd
```

## Installation

Make sure you have **Rust and Cargo** installed.

Clone the repository:

```bash
git clone https://github.com/noxdeve/rsh.git
cd rsh
```

Build and run:

```bash
cargo run
```

For an optimized build:

```bash
cargo run --release
```

## How It Works

rsh follows a simple interactive loop:

```text
Read input
    ↓
Parse command
    ↓
Match built-in command
    ↓
Execute
    ↓
Return to prompt
```

Commands are parsed from standard input and dispatched to their corresponding Rust functions.

Unknown commands are checked against the list of available commands using **Levenshtein distance**. If a close match is found, rsh suggests the likely command.

## Built With

* **Rust**
* `std::io`
* `std::env`
* `std::fs`
* `std::process`
* [`colored`](https://crates.io/crates/colored)
* [`strsim`](https://crates.io/crates/strsim)
* [`whoami`](https://crates.io/crates/whoami)

## Project Structure

```text
rsh/
├── src/
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
└── LICENSE
```

## Purpose

rsh is not intended to replace Bash, Zsh, or other production shells.

It's a learning project built to understand the fundamentals behind a command-line shell and to get deeper into Rust.

## Roadmap

* [ ] External command execution
* [ ] Command history
* [ ] Pipes
* [ ] Input/output redirection
* [ ] Environment variables
* [ ] Better argument parsing
* [ ] Improved error handling
* [ ] Tab completion

## License

This project is licensed under the **MIT License**.

---

**Built by NoX Dev.**

> Build first. Learn by doing.
