## Introduction
This is Brainfuck interpreter written in Rust. It contains all vanilla Brainfuck commands and new added ones. I know that they are not in spirit of Brainfuck, but it's just for fun.
It is just some funny project, not for real usage.

## Usage
- To use this interpreter download source files and compile it with rust or download precompiled program from Releases
- Then run it from console/terminal with path to file with Brainfuck code in argument eg.
```
brainrust [--vanilla] code.bf
```

## Current commands

### COMMANDS THAT ARE IN VANILLA BRAINFUCK

`+` : Increments the value at the current cell by one <br>
`-` : Decrements the value at the current cell by one <br>
`>` : Moves the data pointer to the next cell (cell on the right) <br>
`<` : Moves the data pointer to the previous cell (cell on the left) <br>
`.` : Prints the ASCII value at the current cell (i.e. 65 = 'A') <br>
`,` : Reads a single input character into the current cell <br>
`[` : If the value at the current cell is zero, skips to the corresponding `]`, otherwise, move to the next instruction <br>
`]` : If the value at the current cell is zero, move to the next instruction, otherwise, move backwards in the instructions to the corresponding `[`

`[` and `]` form a while loop. Obviously, they must be balanced.

### BELOW ARE COMMANDS THAT ARE NOT IN VANILLA BRAINFUCK, SO WITH `--vanilla` OPTION THEY WILL BE SKIPPED

`\` : Sets current cell value to 10 (LFeed) <br>
`b'x'` : Sets x character ASCII table value to current cell <br>
`s"abc"` : Sets abc string (any numbers of characters) values to the current cell and the next ones as required, if you want to add `"` to the string use `\"`

`0xAA` : Sets hex value after 0x to the current cell (needs exacly 2 hex numbers after 0x) <br>
`0d123` : Sets decimal value after 0d to the current cell (needs exacly 3 decimal numbers after 0d) <br>
`0b11001010` : Sets binary value after 0b to the current cell (needs exacly 8 binary numbers after 0b)

`f(file_path){operations}` - opens file `file_path` or creates it and executes commands in file and the file contents is new tape (of length 30000, so content above 30000 characters in file won't be accessible and will be lost when opened) <br>
`r(file_path)` - reads the content of the file, saves it to current tape, starting from current pointer location <br>
`w(file_path)` - wrties the tape to file, starting from cell 0 <br>
`a(file_path)` - appends tape to file, starting from cell 0 <br>
**Relative paths are relative to the bf file location, not the CWD, if you want relative to CWD, add `@` before path, for example: `f(@./cwdfile.txt){s"Im in current user's directory"}`**

**If you have vanilla mode disabled you can also do `>123` to move 123 cells forward and `<32` to move 32 cells backward**

`^` : Zeros current cell <br>
`p` : Prints number value of current cell <br>
`;` : Swaps current cell value with next one <br>
`:` : Copies current cell value to the next one <br>
`A` : Prints out address of current cell (pointer value) <br>
`R` : Inserts random number to current cell <br>
`S` : Sleeps the current cell value seconds <br>
`$distance{operations}` : Executes operations with pointer moved by `distance` (distance can be negative) (pointer will return to its original location after executing operations) <br>

`(operations)` : executes operations exacly once if the current value is other than 0, skips operations otherwise

`D` : Prints a splice of a tape with current cell pointed at along its address, mainy for debbuging purposes
`//` : Comment, everything after this to end of the line won't be parsed as code <br>

`Tn` : Switch to tape n (starting tape is 0), n could be any ONE character (eg. 0, 5, a, B, ^, ...)