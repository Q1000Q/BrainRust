# BrainRust

## Introduction
BrainRust is a Brainfuck interpreter written in Rust.

It supports all vanilla Brainfuck commands and also includes extra custom commands for convenience and experimentation. These extensions are intentionally not "pure" Brainfuck—they are included just for fun.

This project is mainly a hobby project and is not intended for production use.

## Usage
- Build from source, or download a precompiled binary from Releases.
- Run the executable and pass a path to a Brainfuck source file.

Windows:
`\.\path\to\brainrust.exe [--vanilla] .\path\to\code.bf`

Linux:
`./path/to/brainrust [--vanilla] ./path/to/code.bf`

Notes:
- On Windows, SmartScreen may warn when running a downloaded `.exe`.
- On Linux, make sure the binary has execute permissions.
- For macOS, compile from source.

### Build from source
1. Clone the repository: `git clone https://github.com/Q1000Q/BrainRust`
2. Install Rust and Cargo: https://doc.rust-lang.org/cargo/getting-started/installation.html
3. In the project directory, run: `cargo build -r`
4. The executable will be in `target/release/`

## Commands

### Vanilla Brainfuck commands
- `+`: Increment the value at the current cell by 1
- `-`: Decrement the value at the current cell by 1
- `>`: Move the data pointer one cell to the right
- `<`: Move the data pointer one cell to the left
- `.`: Print the ASCII character at the current cell (`65` = `A`)
- `,`: Read a single input character into the current cell
- `[`: If current cell is `0`, jump to matching `]`; otherwise continue
- `]`: If current cell is non-zero, jump back to matching `[`; otherwise continue

`[` and `]` form a while loop and must be balanced.

### Extended commands (disabled with `--vanilla`)
- `\`: Set current cell to `10` (line feed)
- `b'x'`: Set current cell to ASCII value of character `x`
- `s"abc"`: Write string bytes to current and following cells (use `\"` to include `"`)

- `0xAA`: Set current cell from hex literal (`0x` + exactly 2 hex digits)
- `0d123`: Set current cell from decimal literal (`0d` + exactly 3 decimal digits)
- `0b11001010`: Set current cell from binary literal (`0b` + exactly 8 bits)

- `f(file_path){operations}`: Open/create `file_path`, use file content as a new tape (length `30000`), then execute `operations`
- `r(file_path)`: Read file content into current tape starting at current pointer
- `w(file_path)`: Write tape to file starting from cell `0`
- `a(file_path)`: Append tape to file starting from cell `0`

Path behavior:
- Relative paths are resolved relative to the `.bf` file location.
- To resolve relative to current working directory, prefix path with `@`.
- Example: `f(@./cwdfile.txt){s"Im in current user's directory"}`

Pointer movement extension:
- You can use `>123` to move 123 cells right and `<32` to move 32 cells left.

Other commands:
- `^`: Set current cell to `0`
- `p`: Print numeric value of current cell
- `;`: Swap current cell with next cell
- `:`: Copy current cell value to next cell
- `A`: Print pointer address (current cell index)
- `R`: Put a random value into current cell
- `S`: Sleep for number of seconds equal to current cell value
- `$distance{operations}`: Run `operations` with pointer temporarily offset by `distance` (can be negative), then restore pointer
- `(operations)`: Execute once only if current cell is non-zero
- `D`: Print a tape slice with pointer position/address (debug helper)
- `//`: Line comment (everything after `//` to end of line is ignored)
- `Tn`: Switch to tape `n` (starting tape is `0`, and `n` is a single character)
- `@(name){operations}`: Define macro `name`
- `#(name)`: Execute macro `name`