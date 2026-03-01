// =========================
// VANILLA COMMANDS TEST
// =========================

// + - > <
+++++        // cell0 = 5
>++++        // cell1 = 4
<-
>+
<

>>

// . print ASCII
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
    // prints 'A'

// , input
,.

>

// [ ] loop
++++++[>++++++<-]    // multiply 6*6  ('$' in ASCII)
>.

// =========================
// EXTENDED VALUE SETTING
// =========================

// newline
\
.

// char literal
b'Z'        // Prints 'Z'
.

// string literal
s"Hello"    // Types Hello and moves the pointer

// hex
0x41        // 'A' in ASCII, prints it
.

// decimal
0d065       // 'A' in ASCII, prints it
.

// binary
0b01000010  // 'B' in ASCII, prints it
.

// =========================
// POINTER MOVEMENT EXTENSIONS
// =========================

>10     // Moves 10 forward
<5      // Moves 5 backward

// =========================
// CELL OPERATIONS
// =========================

^           // zero cell
++++
p           // print numeric value

:           // copy to next
>
p

<           // go back
;           // swap
>
p

A           // print pointer address

\.          // print new line
R           // random value
p

0d001S          // sleep 1 second (cell value = 1)

// =========================
// CONDITIONAL EXECUTION
// =========================

\.      // print new line

(+++++p)    // runs once if non-zero

^
(+++++.)    // skipped

// =========================
// RELATIVE EXECUTION
// =========================

$3{+++++p}  // operate 5 cells away

// =========================
// DEBUG PRINT
// =========================

D

// =========================
// FILE OPERATIONS
// =========================

// create/open file and use as tape
f(testfile.txt){
    s"FILE"
}

<15 // Goes to start of the tape

// read file into tape
r(testfile.txt)

>6 ^>^>^>^>^>^>^>^>^> // Cleans the calculations from previous test, so they don't corrupt the file

// write tape
w(testfile_out.txt)

// append tape
a(testfile_out.txt)

// =========================
// MULTI-TAPE TEST
// =========================

T1
+++++p  // Goes to tape '1' and increments it by 5, it is new tape, so it prints '5'

T0
p       // Goes back to tape '0' and prints current numeric value, which is 0, because of cleaning before writing to file

// =========================
// MACRO SYSTEM
// =========================

@(incprint){
    +p           // Icrements the cell by 1 and prints decimal value of the cell
}

#(incprint)     // Cell was 0, so it incremented it by 1 and printed it ('1')
#(incprint)     // Cell was 1, so it incremented it by 1 and printed it ('2')

// =========================
// COMMENTS TEST
// =========================

// this should be ignored
+++++ // comment after code also should be ignored



// Expected output of executing this file:
// ```
// A{your inputed character}$
// ZAAB44415
// {random number 0-255}
// 155
// [15] 0 0 4 0 0 0 5
//            ^
// 5012
// ```
// The curly brackets are thigs that change depending on input/randomness
// 
// it will also create 2 files:  
// `testfile.txt` with content: `FILE`
// `testfile_out.txt` with content: `FILEHelloBFILEHelloB`