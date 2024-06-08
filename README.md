## Introduction
This is BrainFuck interpreter written in rust. The 1st version (or possibly more) is just vanilla BrainFuck, the other are upgraded versions with more commands etc. It is just some funny project, not for real usage.

## Usage
- To use this interpreter download source files and compile it with rust or download precompiled program from Releases
- Then run it from console/terminal with path to BrainFuck (.bf) file in argument eg.
```
.\BrainRust.exe .\code.bf
```

## Current commands
```
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
```