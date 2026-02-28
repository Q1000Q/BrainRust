(p)     // Cell value 0, shouldn't print
+       // Increment cell
(p)     // Cell value 1, should print

>
0xAA
>
0d123

D       // Debug print
$2{++ > 0d005}		// Moves the pointer by 2, adds 2 to current cell, moves right, sets the cell to value 5 and then returns the pointer to original value
D

$0{s"Hello World!"}[.>]		// Sets the n next cells to string "Hello World!" and then returns to original pointer location and prints from the start