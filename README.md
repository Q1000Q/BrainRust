## Introduction
This is Brainfuck interpreter written in Rust. It contains all vanilla Brainfuck commands and new added ones. I know that they are not in spirit of Brainfuck, but it's just for fun.
It is just some funny project, not for real usage.

## Usage
- To use this interpreter download source files and compile it with rust or download precompiled program from Releases
- Then run it from console/terminal with path to file with Brainfuck code in argument eg.
```
.\BrainRust.exe .\code.bf
```

## Current commands
```
**COMMANDS THAT ARE IN VANILLA BRAINFUCK**
+ : Increments the value at the current cell by one
- : Decrements the value at the current cell by one
> : Moves the data pointer to the next cell (cell on the right)
< : Moves the data pointer to the previous cell (cell on the left)
. : Prints the ASCII value at the current cell (i.e. 65 = 'A')
, : Reads a single input character into the current cell
[ : If the value at the current cell is zero, skips to the corresponding ]
    Otherwise, move to the next instruction
] : If the value at the current cell is zero, move to the next instruction.
    Otherwise, move backwards in the instructions to the corresponding [

[ and ] form a while loop. Obviously, they must be balanced.

**BELOW ARE COMMANDS THAT ARE NOT IN VANILLA BRAINFUCK, SO WITH -v OPTION THEY ARE GONNA BE SKIPPED**
\ : Sets current cell value to 10 (LFeed)
b'x' : Sets x character ASCII table value to current cell
s"abc" : Sets abc string (any numbers of characters) values to the current cell and the next ones as required, if you want to add " to the string use \"

0xAA : Sets hex value after 0x to the current cell (needs exacly 2 hex numbers after 0x)
0d123 : Sets decimal value after 0d to the current cell (needs exacly 3 decimal numbers after 0d)
0b11001010 : Sets binary value after 0b to the current cell (needs exacly 8 binary numbers after 0b)
```